// palette.cs
using System;
using System.Collections.Generic;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Text.Json;

class Palette
{
    static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine("Использование: dotnet run <HEX-цвет> [--json] [--css] [--scss]");
            return;
        }
        string color = args[0];
        bool json = args.Contains("--json");
        bool css = args.Contains("--css");
        bool scss = args.Contains("--scss");

        try
        {
            var (shades, accents, contrast) = GeneratePalette(color);
            if (json) Console.WriteLine(ToJSON(shades, accents, contrast));
            else if (css) Console.WriteLine(ToCSS(shades, accents));
            else if (scss) Console.WriteLine(ToSCSS(shades, accents));
            else PrintPalette(shades, accents, contrast);
        }
        catch (Exception e)
        {
            Console.Error.WriteLine($"Ошибка: {e.Message}");
            Environment.Exit(1);
        }
    }

    static Color HexToColor(string hex)
    {
        hex = hex.TrimStart('#');
        if (hex.Length == 3)
            hex = string.Concat(hex.Select(c => c.ToString() + c));
        int val = Convert.ToInt32(hex, 16);
        return Color.FromArgb((val >> 16) & 0xFF, (val >> 8) & 0xFF, val & 0xFF);
    }

    static string ColorToHex(Color c) => $"#{c.R:X2}{c.G:X2}{c.B:X2}";

    static (float h, float s, float l) RGBToHSL(Color c)
    {
        float r = c.R / 255f, g = c.G / 255f, b = c.B / 255f;
        float max = Math.Max(r, Math.Max(g, b));
        float min = Math.Min(r, Math.Min(g, b));
        float l = (max + min) / 2;
        float h = 0, s = 0;
        if (max != min)
        {
            float d = max - min;
            s = l < 0.5 ? d / (max + min) : d / (2 - max - min);
            if (max == r) h = (g - b) / d + (g < b ? 6 : 0);
            else if (max == g) h = (b - r) / d + 2;
            else h = (r - g) / d + 4;
            h /= 6;
        }
        return (h, s, l);
    }

    static Color HSLToColor(float h, float s, float l)
    {
        float b = l + s * Math.Min(l, 1 - l);
        float newS = b == 0 ? 0 : 2 * (1 - l / b);
        float r, g, bl;
        // Используем стандартное преобразование HSB
        int rgb = Color.HSBtoRGB(h, newS, b);
        return Color.FromArgb((rgb >> 16) & 0xFF, (rgb >> 8) & 0xFF, rgb & 0xFF);
    }

    static (Dictionary<string, string> shades, Dictionary<string, string> accents, Dictionary<string, string> contrast) GeneratePalette(string hex)
    {
        Color baseColor = HexToColor(hex);
        var (h, s, l) = RGBToHSL(baseColor);

        var shades = new Dictionary<string, string>();
        float[] lightness = { 0.90f, 0.82f, 0.74f, 0.66f, 0.58f, 0.50f, 0.42f, 0.34f, 0.26f, 0.18f };
        int[] names = { 50, 100, 200, 300, 400, 500, 600, 700, 800, 900 };
        for (int i = 0; i < lightness.Length; i++)
        {
            Color c = HSLToColor(h, s, lightness[i]);
            shades[names[i].ToString()] = ColorToHex(c);
        }

        var accents = new Dictionary<string, string>();
        float[] accentL = { 0.60f, 0.70f, 0.75f, 0.80f };
        float[] accentS = { 0.95f, 0.90f, 0.85f, 0.80f };
        string[] accentNames = { "A100", "A200", "A400", "A700" };
        for (int i = 0; i < accentNames.Length; i++)
        {
            Color c = HSLToColor(h, accentS[i], accentL[i]);
            accents[accentNames[i]] = ColorToHex(c);
        }

        var contrast = new Dictionary<string, string>();
        foreach (var kv in shades)
        {
            Color c = HexToColor(kv.Value);
            double lum = (0.299 * c.R + 0.587 * c.G + 0.114 * c.B) / 255.0;
            contrast[kv.Key] = lum < 0.5 ? "#ffffff" : "#000000";
        }
        return (shades, accents, contrast);
    }

    static void PrintPalette(Dictionary<string, string> shades, Dictionary<string, string> accents, Dictionary<string, string> contrast)
    {
        Console.WriteLine("\n--- Material Palette ---");
        foreach (var kv in shades)
        {
            Color bg = HexToColor(kv.Value);
            Color fg = HexToColor(contrast[kv.Key]);
            Console.Write($"\x1b[48;2;{bg.R};{bg.G};{bg.B}m\x1b[38;2;{fg.R};{fg.G};{fg.B}m");
            Console.WriteLine($" {kv.Key,4} {kv.Value} \x1b[0m");
        }
        Console.WriteLine("\n--- Accents ---");
        foreach (var kv in accents)
        {
            Color bg = HexToColor(kv.Value);
            Console.Write($"\x1b[48;2;{bg.R};{bg.G};{bg.B}m");
            Console.WriteLine($" {kv.Key,4} {kv.Value} \x1b[0m");
        }
    }

    static string ToJSON(Dictionary<string, string> shades, Dictionary<string, string> accents, Dictionary<string, string> contrast)
    {
        var obj = new { shades, accents, contrast };
        return JsonSerializer.Serialize(obj, new JsonSerializerOptions { WriteIndented = true });
    }

    static string ToCSS(Dictionary<string, string> shades, Dictionary<string, string> accents)
    {
        var sb = new StringBuilder(":root {\n");
        foreach (var kv in shades) sb.Append($"  --md-{kv.Key}: {kv.Value};\n");
        foreach (var kv in accents) sb.Append($"  --md-{kv.Key}: {kv.Value};\n");
        sb.Append("}");
        return sb.ToString();
    }

    static string ToSCSS(Dictionary<string, string> shades, Dictionary<string, string> accents)
    {
        var sb = new StringBuilder();
        foreach (var kv in shades) sb.Append($"$md-{kv.Key}: {kv.Value};\n");
        foreach (var kv in accents) sb.Append($"$md-{kv.Key}: {kv.Value};\n");
        return sb.ToString();
    }
}
