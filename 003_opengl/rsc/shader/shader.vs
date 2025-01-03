#version 140

// 引数
in vec3 iPosition;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;

// 戻り値
out vec3 FragPosition;

void main()
{
    FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}
