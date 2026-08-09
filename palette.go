// palette.go
package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"image/color"
	"strconv"
	"strings"
)

func hexToRGB(hex string) (uint8, uint8, uint8, error) {
	hex = strings.TrimPrefix(hex, "#")
	if len(hex) == 3 {
		hex = string([]byte{hex[0], hex[0], hex[1], hex[1], hex[2], hex[2]})
	}
	if len(hex) != 6 {
		return 0, 0, 0, fmt.Errorf("неверный HEX-формат")
	}
	r, _ := strconv.ParseUint(hex[0:2], 16, 8)
	g, _ := strconv.ParseUint(hex[2:4], 16, 8)
	b, _ := strconv.ParseUint(hex[4:6], 16, 8)
	return uint8(r), uint8(g), uint8(b), nil
}

func rgbToHex(r, g, b uint8) string {
	return fmt.Sprintf("#%02x%02x%02x", r, g, b)
}

func rgbToHSL(r, g, b uint8) (float64, float64, float64) {
	rf, gf, bf := float64(r)/255.0, float64(g)/255.0, float64(b)/255.0
	max := max(rf, gf, bf)
	min := min(rf, gf, bf)
	l := (max + min) / 2
	var h, s float64
	if max == min {
		h = 0
		s = 0
	} else {
		diff := max - min
		if l < 0.5 {
			s = diff / (max + min)
		} else {
			s = diff / (2.0 - max - min)
		}
		switch max {
		case rf:
			h = (gf - bf) / diff
			if gf < bf {
				h += 6
			}
		case gf:
			h = (bf-rf)/diff + 2
		case bf:
			h = (rf-gf)/diff + 4
		}
		h /= 6
	}
	return h, s, l
}

func hslToRGB(h, s, l float64) (uint8, uint8, uint8) {
	var r, g, b float64
	if s == 0 {
		r, g, b = l, l, l
	} else {
		var v2 float64
		if l < 0.5 {
			v2 = l * (1 + s)
		} else {
			v2 = l + s - l*s
		}
		v1 := 2*l - v2
		r = hueToRGB(v1, v2, h+1.0/3.0)
		g = hueToRGB(v1, v2, h)
		b = hueToRGB(v1, v2, h-1.0/3.0)
	}
	return uint8(r * 255), uint8(g * 255), uint8(b * 255)
}

func hueToRGB(v1, v2, h float64) float64 {
	if h < 0 {
		h += 1
	}
	if h > 1 {
		h -= 1
	}
	if 6*h < 1 {
		return v1 + (v2-v1)*6*h
	}
	if 2*h < 1 {
		return v2
	}
	if 3*h < 2 {
		return v1 + (v2-v1)*(2.0/3.0-h)*6
	}
	return v1
}

func generatePalette(hex string) (map[string]string, map[string]string, map[string]string, error) {
	r, g, b, err := hexToRGB(hex)
	if err != nil {
		return nil, nil, nil, err
	}
	h, s, l := rgbToHSL(r, g, b)

	shades := make(map[string]string)
	lightnessLevels := []float64{0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18}
	for i, lvl := range lightnessLevels {
		name := 50 + i*100
		rr, gg, bb := hslToRGB(h, s, lvl)
		shades[fmt.Sprintf("%d", name)] = rgbToHex(rr, gg, bb)
	}

	accents := make(map[string]string)
	accentLightness := []float64{0.60, 0.70, 0.75, 0.80}
	accentSaturation := []float64{0.95, 0.90, 0.85, 0.80}
	accentNames := []string{"A100", "A200", "A400", "A700"}
	for i, name := range accentNames {
		rr, gg, bb := hslToRGB(h, accentSaturation[i], accentLightness[i])
		accents[name] = rgbToHex(rr, gg, bb)
	}

	contrast := make(map[string]string)
	for name, hexC := range shades {
		rr, gg, bb, _ := hexToRGB(hexC)
		lum := (0.299*float64(rr) + 0.587*float64(gg) + 0.114*float64(bb)) / 255.0
		if lum < 0.5 {
			contrast[name] = "#ffffff"
		} else {
			contrast[name] = "#000000"
		}
	}
	return shades, accents, contrast, nil
}

func printPalette(shades, accents, contrast map[string]string) {
	fmt.Println("\n--- Material Palette ---")
	for name, hexC := range shades {
		r, g, b, _ := hexToRGB(hexC)
		txt := contrast[name]
		tr, tg, tb, _ := hexToRGB(txt)
		fmt.Printf("\033[48;2;%d;%d;%dm\033[38;2;%d;%d;%dm %4s %s \033[0m\n", r, g, b, tr, tg, tb, name, hexC)
	}
	fmt.Println("\n--- Accents ---")
	for name, hexC := range accents {
		r, g, b, _ := hexToRGB(hexC)
		fmt.Printf("\033[48;2;%d;%d;%dm %4s %s \033[0m\n", r, g, b, name, hexC)
	}
}

func main() {
	var jsonFlag bool
	var cssFlag bool
	var scssFlag bool
	flag.BoolVar(&jsonFlag, "json", false, "Вывод в JSON")
	flag.BoolVar(&cssFlag, "css", false, "Вывод CSS-переменных")
	flag.BoolVar(&scssFlag, "scss", false, "Вывод SCSS-переменных")
	flag.Parse()

	if flag.NArg() < 1 {
		fmt.Println("Использование: palette <HEX-цвет> [--json] [--css] [--scss]")
		return
	}
	color := flag.Arg(0)

	shades, accents, contrast, err := generatePalette(color)
	if err != nil {
		fmt.Println("Ошибка:", err)
		return
	}

	if jsonFlag {
		palette := map[string]interface{}{
			"shades":   shades,
			"accents":  accents,
			"contrast": contrast,
		}
		data, _ := json.MarshalIndent(palette, "", "  ")
		fmt.Println(string(data))
	} else if cssFlag {
		fmt.Println(":root {")
		for name, hexC := range shades {
			fmt.Printf("  --md-%s: %s;\n", name, hexC)
		}
		for name, hexC := range accents {
			fmt.Printf("  --md-%s: %s;\n", name, hexC)
		}
		fmt.Println("}")
	} else if scssFlag {
		for name, hexC := range shades {
			fmt.Printf("$md-%s: %s;\n", name, hexC)
		}
		for name, hexC := range accents {
			fmt.Printf("$md-%s: %s;\n", name, hexC)
		}
	} else {
		printPalette(shades, accents, contrast)
	}
}
