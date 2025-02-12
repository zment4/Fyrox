uniform sampler2D diffuseTexture;
uniform vec4 color;

out vec4 FragColor;

in vec2 texCoord;

void main()
{
    FragColor = color * S_SRGBToLinear(texture(diffuseTexture, texCoord));
}