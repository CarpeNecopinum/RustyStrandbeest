#version 330
layout(lines) in;
layout(triangle_strip, max_vertices = 8) out;

uniform mat4 projection;

in vec2 vPosition[];
out vec3 vNormal;

void main() {
    vec2 base = vPosition[0];
    vec2 fwd = vPosition[1] - vPosition[0];
    vec2 rgt = normalize(vec2(fwd.y, -fwd.x));

    vec2 pos;
    vNormal = vec3(rgt, 0.0);
    pos = base + rgt;
    gl_Position = projection * vec4(pos, -0.1, 1.0); EmitVertex();
    pos = base + rgt + fwd;
    gl_Position = projection * vec4(pos, -0.1, 1.0); EmitVertex();
    pos = base + rgt;
    gl_Position = projection * vec4(pos, 0.0, 1.0); EmitVertex();
    pos = base + rgt + fwd;
    vNormal = vec3(0.0, 0.0, 1.0);
    gl_Position = projection * vec4(pos, 0.0, 1.0); EmitVertex();
    pos = base - rgt;
    gl_Position = projection * vec4(pos, 0.0, 1.0); EmitVertex();
    pos = base - rgt + fwd;
    gl_Position = projection * vec4(pos, 0.0, 1.0); EmitVertex();
    pos = base - rgt;
    gl_Position = projection * vec4(pos, -0.1, 1.0); EmitVertex();
    pos = base - rgt + fwd;
    gl_Position = projection * vec4(pos, -0.1, 1.0); EmitVertex();
}
