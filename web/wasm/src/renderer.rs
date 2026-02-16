use nalgebra::Matrix4;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, WebGlVertexArrayObject, HtmlCanvasElement};

pub struct WebGLRenderer {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    globe_program: WebGlProgram,
    sensor_program: WebGlProgram,
    facility_program: WebGlProgram,
    globe_vao: Option<WebGlVertexArrayObject>,
    globe_vertex_buffer: Option<WebGlBuffer>,
    globe_index_buffer: Option<WebGlBuffer>,
    globe_index_count: i32,
}

impl WebGLRenderer {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, wasm_bindgen::JsValue> {
        let context = canvas
            .get_context("webgl2")?
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()?;
        
        let globe_program = Self::create_program(
            &context,
            include_str!("shaders/globe.vert"),
            include_str!("shaders/globe.frag"),
        )?;
        
        let sensor_program = Self::create_program(
            &context,
            include_str!("shaders/sensor.vert"),
            include_str!("shaders/sensor.frag"),
        )?;
        
        let facility_program = Self::create_program(
            &context,
            include_str!("shaders/sensor.vert"),
            include_str!("shaders/facility.frag"),
        )?;
        
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        
        let mut renderer = Self {
            context,
            canvas,
            globe_program,
            sensor_program,
            facility_program,
            globe_vao: None,
            globe_vertex_buffer: None,
            globe_index_buffer: None,
            globe_index_count: 0,
        };
        
        renderer.create_sphere_geometry();
        
        Ok(renderer)
    }
    
    pub fn clear(&self) {
        self.context.clear_color(0.0, 0.0, 0.05, 1.0);
        self.context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT | WebGl2RenderingContext::DEPTH_BUFFER_BIT);
    }
    
    pub fn resize(&self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
        self.context.viewport(0, 0, width as i32, height as i32);
    }
    
    pub fn draw_globe(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        self.context.use_program(Some(&self.globe_program));
        
        if let Some(vao) = &self.globe_vao {
            self.context.bind_vertex_array(Some(vao));
        }
        
        let view_loc = self.context.get_uniform_location(&self.globe_program, "u_view");
        let proj_loc = self.context.get_uniform_location(&self.globe_program, "u_projection");
        
        self.context.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            view.as_slice(),
        );
        self.context.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            projection.as_slice(),
        );
        
        if self.globe_index_count > 0 {
            self.context.draw_elements_with_i32(
                WebGl2RenderingContext::TRIANGLES,
                self.globe_index_count,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                0,
            );
        }
        
        self.context.bind_vertex_array(None);
    }
    
    fn create_sphere_geometry(&mut self) {
        let lat_segments = 32;
        let lon_segments = 64;
        let radius = 1.0f32;
        
        let mut vertices: Vec<f32> = Vec::new();
        let mut indices: Vec<u16> = Vec::new();
        
        for lat in 0..=lat_segments {
            let theta = std::f32::consts::PI * (lat as f32) / (lat_segments as f32);
            let sin_theta = theta.sin();
            let cos_theta = theta.cos();
            
            for lon in 0..=lon_segments {
                let phi = 2.0 * std::f32::consts::PI * (lon as f32) / (lon_segments as f32);
                let sin_phi = phi.sin();
                let cos_phi = phi.cos();
                
                let x = radius * sin_theta * cos_phi;
                let y = radius * cos_theta;
                let z = radius * sin_theta * sin_phi;
                
                let nx = sin_theta * cos_phi;
                let ny = cos_theta;
                let nz = sin_theta * sin_phi;
                
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(nx);
                vertices.push(ny);
                vertices.push(nz);
            }
        }
        
        for lat in 0..lat_segments {
            for lon in 0..lon_segments {
                let first = (lat * (lon_segments + 1) + lon) as u16;
                let second = first + lon_segments as u16 + 1;
                
                indices.push(first);
                indices.push(second);
                indices.push(first + 1);
                
                indices.push(second);
                indices.push(second + 1);
                indices.push(first + 1);
            }
        }
        
        self.globe_index_count = indices.len() as i32;
        
        if let Some(vao) = self.context.create_vertex_array() {
            self.context.bind_vertex_array(Some(&vao));
            self.globe_vao = Some(vao);
        }
        
        if let Some(buffer) = self.context.create_buffer() {
            self.context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));
            
            let vertices_array = js_sys::Float32Array::from(&vertices[..]);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vertices_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
            
            self.globe_vertex_buffer = Some(buffer);
        }
        
        let position_loc = self.context.get_attrib_location(&self.globe_program, "a_position") as u32;
        let normal_loc = self.context.get_attrib_location(&self.globe_program, "a_normal") as u32;
        
        self.context.vertex_attrib_pointer_with_i32(
            position_loc,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            24,
            0,
        );
        self.context.enable_vertex_attrib_array(position_loc);
        
        self.context.vertex_attrib_pointer_with_i32(
            normal_loc,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            24,
            12,
        );
        self.context.enable_vertex_attrib_array(normal_loc);
        
        if let Some(buffer) = self.context.create_buffer() {
            self.context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
            
            let indices_array = js_sys::Uint16Array::from(&indices[..]);
            self.context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
            
            self.globe_index_buffer = Some(buffer);
        }
        
        self.context.bind_vertex_array(None);
    }
    
    pub fn draw_sensor(
        &self,
        position: &nalgebra::Vector3<f32>,
        value: f32,
        color: &[f32; 4],
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) {
        self.context.use_program(Some(&self.sensor_program));
        
        let view_loc = self.context.get_uniform_location(&self.sensor_program, "u_view");
        let proj_loc = self.context.get_uniform_location(&self.sensor_program, "u_projection");
        let pos_loc = self.context.get_uniform_location(&self.sensor_program, "u_position");
        let color_loc = self.context.get_uniform_location(&self.sensor_program, "u_color");
        let size_loc = self.context.get_uniform_location(&self.sensor_program, "u_size");
        
        self.context.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            view.as_slice(),
        );
        self.context.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            projection.as_slice(),
        );
        self.context.uniform3f(
            pos_loc.as_ref(),
            position.x,
            position.y,
            position.z,
        );
        self.context.uniform4f(
            color_loc.as_ref(),
            color[0],
            color[1],
            color[2],
            color[3],
        );
        self.context.uniform1f(size_loc.as_ref(), 0.02 + value * 0.01);
        
        self.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
    }
    
    pub fn draw_facility(
        &self,
        position: &nalgebra::Vector3<f32>,
        color: &[f32; 4],
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) {
        self.context.use_program(Some(&self.facility_program));
        
        let view_loc = self.context.get_uniform_location(&self.facility_program, "u_view");
        let proj_loc = self.context.get_uniform_location(&self.facility_program, "u_projection");
        let pos_loc = self.context.get_uniform_location(&self.facility_program, "u_position");
        let color_loc = self.context.get_uniform_location(&self.facility_program, "u_color");
        let size_loc = self.context.get_uniform_location(&self.facility_program, "u_size");
        
        self.context.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            view.as_slice(),
        );
        self.context.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            projection.as_slice(),
        );
        self.context.uniform3f(
            pos_loc.as_ref(),
            position.x,
            position.y,
            position.z,
        );
        self.context.uniform4f(
            color_loc.as_ref(),
            color[0],
            color[1],
            color[2],
            color[3],
        );
        // Facilities are larger than sensors (diamond shape)
        self.context.uniform1f(size_loc.as_ref(), 0.05);
        
        self.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
    }
    
    pub fn draw_plume_particle(
        &self,
        position: &nalgebra::Vector3<f32>,
        intensity: f32,
        view: &Matrix4<f32>,
        projection: &Matrix4<f32>,
    ) {
        self.context.use_program(Some(&self.sensor_program));
        
        let view_loc = self.context.get_uniform_location(&self.sensor_program, "u_view");
        let proj_loc = self.context.get_uniform_location(&self.sensor_program, "u_projection");
        let pos_loc = self.context.get_uniform_location(&self.sensor_program, "u_position");
        let color_loc = self.context.get_uniform_location(&self.sensor_program, "u_color");
        let size_loc = self.context.get_uniform_location(&self.sensor_program, "u_size");
        
        self.context.uniform_matrix4fv_with_f32_array(
            view_loc.as_ref(),
            false,
            view.as_slice(),
        );
        self.context.uniform_matrix4fv_with_f32_array(
            proj_loc.as_ref(),
            false,
            projection.as_slice(),
        );
        self.context.uniform3f(
            pos_loc.as_ref(),
            position.x,
            position.y,
            position.z,
        );
        self.context.uniform4f(
            color_loc.as_ref(),
            1.0,
            0.4,
            0.0,
            intensity * 0.5,
        );
        self.context.uniform1f(size_loc.as_ref(), 0.015);
        
        self.context.draw_arrays(WebGl2RenderingContext::POINTS, 0, 1);
    }
    
    fn create_program(
        context: &WebGl2RenderingContext,
        vert_source: &str,
        frag_source: &str,
    ) -> Result<WebGlProgram, wasm_bindgen::JsValue> {
        let vert_shader = Self::compile_shader(context, WebGl2RenderingContext::VERTEX_SHADER, vert_source)?;
        let frag_shader = Self::compile_shader(context, WebGl2RenderingContext::FRAGMENT_SHADER, frag_source)?;
        
        let program = context.create_program().ok_or("Failed to create program")?;
        context.attach_shader(&program, &vert_shader);
        context.attach_shader(&program, &frag_shader);
        context.link_program(&program);
        
        if !context.get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            let info = context.get_program_info_log(&program).unwrap_or_default();
            return Err(format!("Program link failed: {}", info).into());
        }
        
        Ok(program)
    }
    
    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, wasm_bindgen::JsValue> {
        let shader = context.create_shader(shader_type).ok_or("Failed to create shader")?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);
        
        if !context.get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            let info = context.get_shader_info_log(&shader).unwrap_or_default();
            return Err(format!("Shader compile failed: {}", info).into());
        }
        
        Ok(shader)
    }
}
