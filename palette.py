# palette.py
import sys
import argparse
import colorsys
import json
import re

def hex_to_rgb(hex_color):
    hex_color = hex_color.lstrip('#')
    if len(hex_color) == 3:
        hex_color = ''.join(c*2 for c in hex_color)
    return tuple(int(hex_color[i:i+2], 16) for i in (0, 2, 4))

def rgb_to_hex(rgb):
    return '#{:02x}{:02x}{:02x}'.format(*rgb)

def rgb_to_hsl(rgb):
    r, g, b = [x/255.0 for x in rgb]
    h, l, s = colorsys.rgb_to_hls(r, g, b)
    return (h, s, l)  # h, saturation, lightness

def hsl_to_rgb(h, s, l):
    r, g, b = colorsys.hls_to_rgb(h, l, s)
    return tuple(int(x*255) for x in (r, g, b))

def generate_palette(hex_color):
    rgb = hex_to_rgb(hex_color)
    h, s, l = rgb_to_hsl(rgb)

    # Оттенки 50-900
    lightness_levels = [0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18]
    shades = {}
    for i, lvl in enumerate(lightness_levels):
        name = 50 + i*100
        r, g, b = hsl_to_rgb(h, s, lvl)
        shades[name] = rgb_to_hex((r, g, b))

    # Акцентные цвета
    accent_lightness = [0.60, 0.70, 0.75, 0.80]
    accent_saturation = [0.95, 0.90, 0.85, 0.80]
    accent_names = ['A100', 'A200', 'A400', 'A700']
    accents = {}
    for i, name in enumerate(accent_names):
        r, g, b = hsl_to_rgb(h, accent_saturation[i], accent_lightness[i])
        accents[name] = rgb_to_hex((r, g, b))

    # Контрастный текст для каждого оттенка (белый или чёрный)
    contrast = {}
    for name, hex_c in shades.items():
        r, g, b = hex_to_rgb(hex_c)
        luminance = (0.299*r + 0.587*g + 0.114*b) / 255
        contrast[name] = '#ffffff' if luminance < 0.5 else '#000000'

    return shades, accents, contrast

def print_palette(shades, accents, contrast):
    # Заголовок
    print("\n--- Material Palette ---")
    # Оттенки
    for name, hex_c in shades.items():
        r, g, b = hex_to_rgb(hex_c)
        # Цветной блок с текстом
        text_color = contrast[name]
        # Используем ANSI True Color для фона и текста
        bg = f"\033[48;2;{r};{g};{b}m"
        fg = f"\033[38;2;{int(text_color[1:3],16)};{int(text_color[3:5],16)};{int(text_color[5:7],16)}m"
        reset = "\033[0m"
        print(f"{bg}{fg} {name:4} {hex_c} {reset}")

    # Акценты
    print("\n--- Accents ---")
    for name, hex_c in accents.items():
        r, g, b = hex_to_rgb(hex_c)
        bg = f"\033[48;2;{r};{g};{b}m"
        print(f"{bg} {name:4} {hex_c} {reset}")

def main():
    parser = argparse.ArgumentParser(description='Material Palette Generator')
    parser.add_argument('color', help='Основной цвет (HEX, например #6200EE)')
    parser.add_argument('--json', action='store_true', help='Вывод в JSON')
    parser.add_argument('--css', action='store_true', help='Вывод CSS-переменных')
    parser.add_argument('--scss', action='store_true', help='Вывод SCSS-переменных')
    args = parser.parse_args()

    try:
        shades, accents, contrast = generate_palette(args.color)
    except Exception as e:
        print(f"Ошибка: {e}", file=sys.stderr)
        sys.exit(1)

    if args.json:
        palette = {'shades': shades, 'accents': accents, 'contrast': contrast}
        print(json.dumps(palette, indent=2))
    elif args.css:
        print(":root {")
        for name, hex_c in shades.items():
            print(f"  --md-{name}: {hex_c};")
        for name, hex_c in accents.items():
            print(f"  --md-{name}: {hex_c};")
        print("}")
    elif args.scss:
        for name, hex_c in shades.items():
            print(f"$md-{name}: {hex_c};")
        for name, hex_c in accents.items():
            print(f"$md-{name}: {hex_c};")
    else:
        print_palette(shades, accents, contrast)

if __name__ == '__main__':
    main()
