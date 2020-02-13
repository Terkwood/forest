pub const BEGINNING: &str = "shader_type canvas_item;

const vec4 BACKGROUND  = vec4(.0, .0, .0, .0);

// COLORS
const float _ = 0.;
const float B = 1.;
const float D = 2.;
const float O = 3.;

vec2 grid(vec2 p, vec2 size) { return floor(p * size); }

";
pub const BOGUS_MID: &str = "
vec4 bitmap(vec2 p, vec2 scale) {
	vec4 res = BACKGROUND;
	
	vec2 gv = grid(p, scale); // grid guide
";
pub const BOGUS_END: &str = "
        // Indexing is upside down.
        int y = int(scale.y - gv.y - 5.);
		float m = 0.;
		m = Q(m,y,0,_,_,_,_,_,B,B,B);
		m = Q(m,y,1,_,_,_,B,B,B,D,O);
		m = Q(m,y,2, _,_,B,B,D,D,D,O);
		m = Q(m,y,3, _,B,B,O,D,D,O,O);
		m = Q(m,y,4, _,B,D,O,O,O,O,O);
		m = Q(m,y,5, B,B,D,D,O,O,D,D);
		m = Q(m,y,6, B,D,D,D,O,D,D,D);
		m = Q(m,y,7, B,D,D,D,O,D,D,D);
		m = Q(m,y,8, B,D,D,O,O,D,D,D);
		m = Q(m,y,9, B,O,O,O,O,O,D,D);
		m = Q(m,y,10,B,O,O,B,B,B,B,B);
        m = Q(m,y,11,B,B,B,B,D,D,B,D);
		m = Q(m,y,12,_,B,B,D,D,D,B,D);
		m = Q(m,y,13,_,_,B,D,D,D,D,D);
		m = Q(m,y,14,_,_,B,B,D,D,D,D);
		m = Q(m,y,15,_,_,_,B,B,B,B,B);
	
		float ldx = 15. - gv.x; // left bit index
		float rdx = gv.x;
		float bit = 0.;
		
		if (gv.x >= 8.) 
			bit = mod(m / pow(4., ldx), 4.); //decode
		else
			bit = mod(m / pow(4., rdx), 4.); // mirror
		bit = floor(bit);                    // sharpen
		
		// Add color
		if (bit > 2.)
			res = vec4(0.035,0.686,0.247,1.0);
		else if (bit > 1.)
			res = vec4(1);
		else if (bit > 0.)
		res = vec4(0,0,0,1);  
	}
	return res;
}

void fragment() {
	vec2 resolution = 1.0 / SCREEN_PIXEL_SIZE;
	vec2 uv = FRAGCOORD.xy / resolution.xy * vec2(resolution.x / resolution.y, 1.0);
	COLOR = bitmap(uv - vec2(.1, .1), vec2(20.));
}";
