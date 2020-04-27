#version 150 core
in vec3 v_gpos;
out vec4 o_Color;
uniform float u_trans_size;
uniform float u_trans_num;
uniform sampler1D u_trans_pos_x;
uniform sampler1D u_trans_pos_y;
uniform sampler1D u_trans_pos_z;
uniform sampler1D u_trans_phase;
const float PI = 3.141592653589793;
const float WAVE_LENGTH = 8.5;
const float WAVE_NUM = 2.0*PI/WAVE_LENGTH;
void main() {
    float re = 0.0;
    float im = 0.0;
    for(float idx = 0.0; idx < 65536.0; idx++){
        if (idx >= u_trans_num) break;
        vec4 tr_x = texture(u_trans_pos_x, (idx+0.5) / u_trans_num);
        vec4 tr_y = texture(u_trans_pos_y, (idx+0.5) / u_trans_num);
        vec4 tr_z = texture(u_trans_pos_z, (idx+0.5) / u_trans_num);
        float tx = floor(255.0 * (tr_x.x + 255.0 * tr_x.y));
        float ty = floor(255.0 * (tr_y.x + 255.0 * tr_y.y));
        float tz = floor(255.0 * (tr_z.x + 255.0 * tr_z.y));
        vec3 tp = vec3(tx * u_trans_size, ty * u_trans_size, tz * u_trans_size);
        float p = 2.0*PI*texture(u_trans_phase, (idx+0.5) / u_trans_num).x;
        float d = length(v_gpos - tp);
        im += cos(p - WAVE_NUM*d) / d;
        re += sin(p - WAVE_NUM*d) / d;
    }
    float c = re*re + im*im;
    o_Color = vec4(c, 0.0, 0.0, 0.0);
}