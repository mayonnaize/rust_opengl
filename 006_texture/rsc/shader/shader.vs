#version 140

in vec3 iPosition;
in vec3 iNormal;
in vec2 iTexCoords;

uniform mat4 uModel;
uniform mat4 uView;
uniform mat4 uProjection;
uniform float uAlpha;

out float Alpha;
out vec3 FragPosition;
out vec3 Normal;
out vec2 TexCoords;

void main()
{
    Alpha = uAlpha;
    FragPosition = vec3(uModel * vec4(iPosition, 1.0));
    // 頂点座標に与えた変換と同じことを法線ベクトルに行う モデル行列の逆転置行列というらしい
    // 法線ベクトル=Normal Vector
    Normal = mat3(transpose(inverse(uModel))) * iNormal;
    TexCoords = iTexCoords;
    gl_Position = uProjection * uView * vec4(FragPosition, 1.0);
}
