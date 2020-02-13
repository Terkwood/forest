pub const BEGINNING: &str = "shader_type canvas_item;

const vec4 BACKGROUND  = vec4(.0, .0, .0, .0);

// COLORS
const float _ = 0.;
const float B = 1.;
const float D = 2.;
const float O = 3.;

vec2 grid(vec2 p, vec2 size) { return floor(p * size); }

";
pub const DEFINE_BITMAP_FN: &str = "
vec4 bitmap(vec2 p, vec2 scale) {
	vec4 res = BACKGROUND;
	
	vec2 gv = grid(p, scale); // grid guide
";
pub const BITMAP_BEFORE_Q_CALLS: &str = "
// Indexing is upside down.
int y = int(scale.y - gv.y - 5.);
float m = 0.;
";
pub const BOGUS_END: &str = "
	
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
