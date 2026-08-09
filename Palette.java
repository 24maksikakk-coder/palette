// Palette.java
import java.awt.Color;
import java.util.*;
import java.util.stream.*;

public class Palette {
    public static void main(String[] args) {
        if (args.length < 1) {
            System.err.println("Использование: java Palette <HEX-цвет> [--json] [--css] [--scss]");
            System.exit(1);
        }
        String color = args[0];
        boolean json = false, css = false, scss = false;
        for (int i=1; i<args.length; i++) {
            if (args[i].equals("--json")) json = true;
            else if (args[i].equals("--css")) css = true;
            else if (args[i].equals("--scss")) scss = true;
        }

        try {
            Map<String, String> shades = new LinkedHashMap<>();
            Map<String, String> accents = new LinkedHashMap<>();
            Map<String, String> contrast = new LinkedHashMap<>();
            generatePalette(color, shades, accents, contrast);

            if (json) {
                System.out.println(toJSON(shades, accents, contrast));
            } else if (css) {
                System.out.println(toCSS(shades, accents));
            } else if (scss) {
                System.out.println(toSCSS(shades, accents));
            } else {
                printPalette(shades, accents, contrast);
            }
        } catch (Exception e) {
            System.err.println("Ошибка: " + e.getMessage());
            System.exit(1);
        }
    }

    private static Color hexToColor(String hex) {
        hex = hex.replaceFirst("^#", "");
        if (hex.length() == 3) {
            hex = hex.replaceAll("(.)", "$1$1");
        }
        return new Color(Integer.parseInt(hex, 16));
    }

    private static String colorToHex(Color c) {
        return String.format("#%02x%02x%02x", c.getRed(), c.getGreen(), c.getBlue());
    }

    private static float[] rgbToHSL(Color c) {
        float[] hsl = new float[3];
        Color.RGBtoHSB(c.getRed(), c.getGreen(), c.getBlue(), hsl);
        return hsl; // [hue, saturation, brightness] (0..1)
    }

    private static Color hslToColor(float h, float s, float l) {
        // Преобразуем lightness в brightness для HSB
        float b = l + s * Math.min(l, 1 - l);
        float newS = (b == 0) ? 0 : 2 * (1 - l / b);
        int rgb = Color.HSBtoRGB(h, newS, b);
        return new Color(rgb);
    }

    private static void generatePalette(String hex, Map<String, String> shades, Map<String, String> accents, Map<String, String> contrast) {
        Color base = hexToColor(hex);
        float[] hsl = rgbToHSL(base);
        float h = hsl[0], s = hsl[1], l = hsl[2];

        float[] lightness = {0.90f, 0.82f, 0.74f, 0.66f, 0.58f, 0.50f, 0.42f, 0.34f, 0.26f, 0.18f};
        int[] names = {50, 100, 200, 300, 400, 500, 600, 700, 800, 900};
        for (int i=0; i<lightness.length; i++) {
            Color c = hslToColor(h, s, lightness[i]);
            shades.put(String.valueOf(names[i]), colorToHex(c));
        }

        float[] accentL = {0.60f, 0.70f, 0.75f, 0.80f};
        float[] accentS = {0.95f, 0.90f, 0.85f, 0.80f};
        String[] accentNames = {"A100", "A200", "A400", "A700"};
        for (int i=0; i<accentNames.length; i++) {
            Color c = hslToColor(h, accentS[i], accentL[i]);
            accents.put(accentNames[i], colorToHex(c));
        }

        for (Map.Entry<String, String> entry : shades.entrySet()) {
            Color c = hexToColor(entry.getValue());
            double lum = (0.299 * c.getRed() + 0.587 * c.getGreen() + 0.114 * c.getBlue()) / 255.0;
            contrast.put(entry.getKey(), lum < 0.5 ? "#ffffff" : "#000000");
        }
    }

    private static void printPalette(Map<String, String> shades, Map<String, String> accents, Map<String, String> contrast) {
        System.out.println("\n--- Material Palette ---");
        for (Map.Entry<String, String> entry : shades.entrySet()) {
            Color bg = hexToColor(entry.getValue());
            Color fg = hexToColor(contrast.get(entry.getKey()));
            System.out.printf("\033[48;2;%d;%d;%dm\033[38;2;%d;%d;%dm %4s %s \033[0m%n",
                bg.getRed(), bg.getGreen(), bg.getBlue(),
                fg.getRed(), fg.getGreen(), fg.getBlue(),
                entry.getKey(), entry.getValue());
        }
        System.out.println("\n--- Accents ---");
        for (Map.Entry<String, String> entry : accents.entrySet()) {
            Color bg = hexToColor(entry.getValue());
            System.out.printf("\033[48;2;%d;%d;%dm %4s %s \033[0m%n",
                bg.getRed(), bg.getGreen(), bg.getBlue(),
                entry.getKey(), entry.getValue());
        }
    }

    private static String toJSON(Map<String, String> shades, Map<String, String> accents, Map<String, String> contrast) {
        return String.format(
            "{\"shades\":%s,\"accents\":%s,\"contrast\":%s}",
            mapToJSON(shades), mapToJSON(accents), mapToJSON(contrast)
        );
    }

    private static String mapToJSON(Map<String, String> map) {
        return map.entrySet().stream()
            .map(e -> String.format("\"%s\":\"%s\"", e.getKey(), e.getValue()))
            .collect(Collectors.joining(",", "{", "}"));
    }

    private static String toCSS(Map<String, String> shades, Map<String, String> accents) {
        StringBuilder sb = new StringBuilder(":root {\n");
        for (Map.Entry<String, String> e : shades.entrySet())
            sb.append("  --md-").append(e.getKey()).append(": ").append(e.getValue()).append(";\n");
        for (Map.Entry<String, String> e : accents.entrySet())
            sb.append("  --md-").append(e.getKey()).append(": ").append(e.getValue()).append(";\n");
        sb.append("}");
        return sb.toString();
    }

    private static String toSCSS(Map<String, String> shades, Map<String, String> accents) {
        StringBuilder sb = new StringBuilder();
        for (Map.Entry<String, String> e : shades.entrySet())
            sb.append("$md-").append(e.getKey()).append(": ").append(e.getValue()).append(";\n");
        for (Map.Entry<String, String> e : accents.entrySet())
            sb.append("$md-").append(e.getKey()).append(": ").append(e.getValue()).append(";\n");
        return sb.toString();
    }
}
