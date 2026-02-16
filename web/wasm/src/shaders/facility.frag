#version 300 es
precision highp float;

in vec2 v_texCoord;
in float v_distance;

uniform vec4 u_color;

out vec4 fragColor;

void main() {
    // Diamond shape using Manhattan distance
    vec2 coord = v_texCoord * 2.0 - 1.0;
    float diamond = abs(coord.x) + abs(coord.y);
    
    // Sharp diamond edge
    float edge = 0.9;
    float alpha = 1.0 - smoothstep(edge - 0.1, edge, diamond);
    
    // Discard pixels outside diamond
    if (diamond > edge) {
        discard;
    }
    
    // Inner glow effect
    float glow = 1.0 - smoothstep(0.0, 0.5, diamond);
    vec3 glowColor = u_color.rgb * (1.0 + glow * 0.5);
    
    // Pulsing effect for active facilities
    float pulse = 0.9 + 0.1 * sin(v_distance * 10.0);
    
    fragColor = vec4(glowColor * pulse, u_color.a * alpha);
}
