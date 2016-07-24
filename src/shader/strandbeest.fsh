#version 330
in vec3 vNormal;

const vec3 L = normalize(vec3(-1.0, 1.0, 1.0));
vec3 N = normalize(vNormal);

out vec3 fColor;
void main() {
    fColor = vec3(1,0,1) * dot(N, L);
    fColor = vec3(1,0,1);
}
