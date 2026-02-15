#version 300 es

uniform mat4 u_view;
uniform mat4 u_projection;
uniform vec3 u_position;
uniform float u_size;

out vec4 v_color;

void main() {
    vec4 world_pos = vec4(u_position, 1.0);
    vec4 view_pos = u_view * world_pos;
    
    // Billboard: always face camera
    vec2 offset = vec2(
        float(gl_VertexID % 2) * 2.0 - 1.0,  // x: -1 or 1
        float(gl_VertexID / 2) * 2.0 - 1.0   // y: -1 or 1
    ) * u_size;
    
    view_pos.xy += offset;
    
    gl_Position = u_projection * view_pos;
    gl_PointSize = u_size * 100.0;
}
