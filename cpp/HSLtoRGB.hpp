#ifndef HSL_TO_RGB_HPP_
#define HSL_TO_RGB_HPP_

class ColorRGB {
	public:
		unsigned char R;
		unsigned char G;
		unsigned char B;
		ColorRGB (unsigned char r, unsigned char g, unsigned char b) {
			this->R = r;
			this->G = g;
			this->B = b;
		}
};

class ColorHSL {
	public:
		int H;
		float S;
		float L;

		ColorHSL (int h, float s, float l) {
			H = h;
			S = s;
			L = l;
		}
};

ColorRGB HSLToRGB(ColorHSL hsl);
float HueToRGB(float v1, float v2, float vH);

#endif
