import java.awt.*;

public class Converter {
    /** Helper method that converts hue to rgb */
    public static float hueToRgb(float p, float q, float t) {
        if (t < 0f)
            t += 1f;
        if (t > 1f)
            t -= 1f;
        if (t < 1f / 6f)
            return p + (q - p) * 6f * t;
        if (t < 1f / 2f)
            return q;
        if (t < 2f / 3f)
            return p + (q - p) * (2f / 3f - t) * 6f;
        return p;
    }

    public static int to255(float v) {
        return (int) Math.min(255, Math.max(0, Math.round(255 * v)));
    }

    public Color HSLToRGB(float h, float s, float l) {
        float r, g, b;

        if (s == 0f) {
            // Achromatic (gray)
            r = g = b = l;
        } else {
            float q = l < 0.5f ? l * (1 + s) : l + s - l * s;
            float p = 2 * l - q;
            r = hueToRgb(p, q, h + 1f / 3f);
            g = hueToRgb(p, q, h);
            b = hueToRgb(p, q, h - 1f / 3f);
        }

        int[] rgb = {to255(r), to255(g), to255(b)};
        return new Color(rgb[0], rgb[1], rgb[2]);
    }
}
