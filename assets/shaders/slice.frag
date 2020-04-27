#version 150 core
in vec3 v_gpos;
out vec4 o_Color;
uniform sampler2D u_trans_phase;
uniform vec3 u_xdir;
uniform vec3 u_ydir;
uniform vec3 t_trans_pos;
const float PI = 3.141592653589793;
const float WAVE_LENGTH = 8.5;
const float WAVE_NUM = 2.0*PI/WAVE_LENGTH;
void main() {
    // float t = mod(time, 1.0);
    
    float re = 0.0;
    float im = 0.0;
    for(float ty = 0.0; ty < 14.0; ty++){
        for(float tx = 0.0; tx < 18.0; tx++){
            vec4 p = 2.0*PI*texture(u_trans_phase, vec2(tx/18.0, ty/14.0));
            vec3 tp = t_trans_pos + tx * u_xdir + ty * u_ydir;
            float d = length(v_gpos - tp);
            im += cos(p[0] - WAVE_NUM*d) / d;
            re += sin(p[0] - WAVE_NUM*d) / d;
        }
    }
    float c = re*re + im*im;
    o_Color = vec4(mod(c, 1.0), 0.0, 0.0, 0.0); //mix(tex, vec4(t, 0., 0., 0.0), blend*1.0);
}
