pub const TEXT_VS: &str = r"
#version 140

in vec2 position;
in vec2 tex_coords;

uniform vec2 pos;

out vec2 v_tex_coords;

void main() {
    v_tex_coords = tex_coords;
    gl_Position = vec4(pos + position, 0.0, 1.0);
}
";

pub const TEXT_FS: &str = r"
#version 140

in vec2 v_tex_coords;

uniform sampler2D tex;

out vec4 color;

void main() {
    float alpha = texture(tex, v_tex_coords).r;
    color = vec4(1.0, 1.0, 1.0, alpha);
}
";

pub const FXAA_VS: &str = r"
#version 100

attribute vec2 position;
attribute vec2 i_tex_coords;

varying vec2 v_tex_coords;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_tex_coords = i_tex_coords;
}
";

pub const FXAA_FS: &str = r"
#version 100

precision mediump float;

uniform vec2 resolution;
uniform sampler2D tex;

varying vec2 v_tex_coords;

#define FXAA_REDUCE_MIN   (1.0/ 128.0)
#define FXAA_REDUCE_MUL   (1.0 / 8.0)
#define FXAA_SPAN_MAX     8.0

vec4 fxaa(sampler2D tex, vec2 fragCoord, vec2 resolution,
    vec2 v_rgbNW, vec2 v_rgbNE,
    vec2 v_rgbSW, vec2 v_rgbSE,
    vec2 v_rgbM) {
    vec4 color;
    mediump vec2 inverseVP = vec2(1.0 / resolution.x, 1.0 / resolution.y);
    vec3 rgbNW = texture2D(tex, v_rgbNW).xyz;
    vec3 rgbNE = texture2D(tex, v_rgbNE).xyz;
    vec3 rgbSW = texture2D(tex, v_rgbSW).xyz;
    vec3 rgbSE = texture2D(tex, v_rgbSE).xyz;
    vec4 texColor = texture2D(tex, v_rgbM);
    vec3 rgbM  = texColor.xyz;
    vec3 luma = vec3(0.299, 0.587, 0.114);
    float lumaNW = dot(rgbNW, luma);
    float lumaNE = dot(rgbNE, luma);
    float lumaSW = dot(rgbSW, luma);
    float lumaSE = dot(rgbSE, luma);
    float lumaM  = dot(rgbM,  luma);
    float lumaMin = min(lumaM, min(min(lumaNW, lumaNE), min(lumaSW, lumaSE)));
    float lumaMax = max(lumaM, max(max(lumaNW, lumaNE), max(lumaSW, lumaSE)));

    mediump vec2 dir;
    dir.x = -((lumaNW + lumaNE) - (lumaSW + lumaSE));
    dir.y =  ((lumaNW + lumaSW) - (lumaNE + lumaSE));

    float dirReduce = max((lumaNW + lumaNE + lumaSW + lumaSE) *
        (0.25 * FXAA_REDUCE_MUL), FXAA_REDUCE_MIN);

    float rcpDirMin = 1.0 / (min(abs(dir.x), abs(dir.y)) + dirReduce);
    dir = min(vec2(FXAA_SPAN_MAX, FXAA_SPAN_MAX),
        max(vec2(-FXAA_SPAN_MAX, -FXAA_SPAN_MAX),
        dir * rcpDirMin)) * inverseVP;

    vec3 rgbA = 0.5 * (
        texture2D(tex, fragCoord * inverseVP + dir * (1.0 / 3.0 - 0.5)).xyz +
        texture2D(tex, fragCoord * inverseVP + dir * (2.0 / 3.0 - 0.5)).xyz);
    vec3 rgbB = rgbA * 0.5 + 0.25 * (
        texture2D(tex, fragCoord * inverseVP + dir * -0.5).xyz +
        texture2D(tex, fragCoord * inverseVP + dir * 0.5).xyz);

    float lumaB = dot(rgbB, luma);
    if ((lumaB < lumaMin) || (lumaB > lumaMax))
        color = vec4(rgbA, texColor.a);
    else
        color = vec4(rgbB, texColor.a);
    return color;
}

void main() {
    vec2 fragCoord = v_tex_coords * resolution;
    vec4 color;
        vec2 inverseVP = 1.0 / resolution.xy;
        mediump vec2 v_rgbNW = (fragCoord + vec2(-1.0, -1.0)) * inverseVP;
        mediump vec2 v_rgbNE = (fragCoord + vec2(1.0, -1.0)) * inverseVP;
        mediump vec2 v_rgbSW = (fragCoord + vec2(-1.0, 1.0)) * inverseVP;
        mediump vec2 v_rgbSE = (fragCoord + vec2(1.0, 1.0)) * inverseVP;
        mediump vec2 v_rgbM = vec2(fragCoord * inverseVP);
        color = fxaa(tex, fragCoord, resolution, v_rgbNW, v_rgbNE, v_rgbSW,
        v_rgbSE, v_rgbM);
    gl_FragColor = color;
}
";

pub const SKYBOX_VS: &str = r"
#version 140

in vec3 position;

out vec3 v_tex_coords;

uniform mat4 persp;
uniform mat4 view;

void main() {
    v_tex_coords = position;
    mat4 view_no_translation = mat4(mat3(view));
    gl_Position = persp * view_no_translation * vec4(position, 1.0);
    gl_Position = gl_Position.xyzw; // Force w = 1 for depth correction
}
";

pub const SKYBOX_FS: &str = r"
#version 140

in vec3 v_tex_coords;

uniform samplerCube u_skybox;

out vec4 color;

void main() {
    color = texture(u_skybox, vec3(v_tex_coords.x, -v_tex_coords.y, v_tex_coords.z));
}
";
