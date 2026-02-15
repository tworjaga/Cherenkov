#version 300 es
precision highp float;

uniform vec4 u_color;

out vec4 fragColor;

void main() {
    // Create circular point with soft edges
    vec2 coord = gl_PointCoord - vec2(0.5);
    float dist = length(coord);
    
    if (dist > 0.5) {
        discard;
    }
    
    // Soft edge falloff
    float alpha = 1.0 - smoothstep(0.3, 0.5, dist);
    
    // Glow effect
    float glow = exp(-dist * 4.0) * 0.5;
    
    fragColor = vec4(u_color.rgb + glow, u_color.a * alpha);
}
