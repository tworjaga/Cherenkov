#version 300 es

in vec3 a_position;
in vec3 a_normal;

uniform mat4 u_view;
uniform mat4 u_projection;

out vec3 v_normal;
out vec3 v_position;

void main() {
    v_normal = mat3(u_view) * a_normal;
    v_position = a_position;
    gl_Position = u_projection * u_view * vec4(a_position, 1.0);
}
