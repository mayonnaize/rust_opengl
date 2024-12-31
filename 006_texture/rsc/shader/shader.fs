#version 140

// 3Dオブジェクトのデータ
struct Material {
    // 鏡面反射の強さ
    vec3 specular;
    // 発光の強さ
    float shininess;
};

// 照明のデータ
struct Light {
    // 照明の光が指すベクトル
    vec3 direction;
    // 環境光の強さ
    vec3 ambient;
    // 拡散光の強さ
    vec3 diffuse;
    // 鏡面反射の強さ
    vec3 specular;
};

// shader.vsのout → shader.fsのinで値を渡している
in float Alpha;
in vec3 FragPosition;
in vec3 Normal;
in vec2 TexCoords;

// 描画するテクスチャデータ
uniform sampler2D uScreenTexture;
// カメラの座標データ
uniform vec3 uViewPosition;
// 3Dオブジェクトの照明の属性データ
uniform Material uMaterial;
// 照明のデータ
uniform Light uLight;

void main()
{
    // ambient 環境光
    // texture関数: (テクスチャデータ, テクスチャ座標)=>その座標の位置のテクスチャの色
    // テクスチャの色データ * 環境光の値
    vec3 ambient = uLight.ambient * texture(uScreenTexture, TexCoords).rgb;

    // diffuse 拡散光 = 法線ベクトル方向の拡散光の強さ
    // 特定の方向からの光で面にあたってあらゆる方向に均等に反射
    vec3 norm = normalize(Normal);
    // 光の進行方向とは逆を向く単位ベクトル
    vec3 lightDir = normalize(-uLight.direction);
    float diff = max(dot(norm, lightDir), 0.0);
    // 光の進行方向とは逆を向く単位ベクトルと、法線の単位ベクトルの内積
    vec3 diffuse = uLight.diffuse * diff * texture(uScreenTexture, TexCoords).rgb;

    // specular 鏡面反射 特定の方向からの光が物体に反射しカメラの方向に差し込む光
    vec3 viewDir = normalize(uViewPosition - FragPosition);
    // 反射方向ベクトル = 光の進行方向ベクトルと法線ベクトルで計算
    vec3 reflectDir = reflect(-lightDir, norm);
    // オブジェクトの鏡面反射率 = 反射光とカメラ方向のベクトルの内積とuMaterial.shininessのべき乗 よくわからん
    float spec = pow(max(dot(viewDir, reflectDir), 0.0), uMaterial.shininess);
    // 鏡面反射の光の強さ = 光自体の鏡面反射率 * オブジェクトの鏡面反射率 * 反射光の強さ
    vec3 specular = uLight.specular * spec * uMaterial.specular;

    // オブジェクトの色 = 環境光 + 拡散光 + 鏡面反射
    vec3 result = ambient + diffuse + specular;

    gl_FragColor = vec4(result, Alpha);
}
