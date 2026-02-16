use nalgebra::Matrix4;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlShader, HtmlCanvasElement};

pub struct WebGLRenderer {
    context: WebGl2RenderingContext,
    canvas: HtmlCanvasElement,
    globe_program: WebGlProgram,
    sensor_program: WebGlProgram,
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
        
        context.enable(WebGl2RenderingContext::DEPTH_TEST);
        context.enable(WebGl2RenderingContext::BLEND);
        context.blend_func(WebGl2RenderingContext::SRC_ALPHA, WebGl2RenderingContext::ONE_MINUS_SRC_ALPHA);
        
        Ok(Self {
            context,
            canvas,
            globe_program,
            sensor_program,
        })
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
        
        // Set uniforms
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
        
        // Draw sphere
        self.context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 3600);
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
        // Facilities are larger than sensors
        self.context.uniform1f(size_loc.as_ref(), 0.04);
        
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
        // Plume particles are orange/red with varying alpha
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
