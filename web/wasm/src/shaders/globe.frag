#version 300 es
precision highp float;

in vec3 v_normal;
in vec3 v_position;

out vec4 fragColor;

uniform vec3 u_light_direction;

void main() {
    vec3 base_color = vec3(0.05, 0.1, 0.2);
    vec3 ocean_color = vec3(0.0, 0.15, 0.3);
    
    // Simple lighting
    float light = max(dot(normalize(v_normal), -u_light_direction), 0.0);
    vec3 color = mix(base_color, ocean_color, 0.5) * (0.3 + 0.7 * light);
    
    // Add atmosphere glow at edges
    float fresnel = 1.0 - abs(dot(normalize(v_normal), vec3(0.0, 0.0, 1.0)));
    vec3 atmosphere = vec3(0.3, 0.6, 1.0) * pow(fresnel, 3.0) * 0.5;
    
    fragColor = vec4(color + atmosphere, 1.0);
}
