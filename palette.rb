# palette.rb
require 'color'
require 'json'
require 'optparse'

def hex_to_rgb(hex)
  hex = hex.gsub('#', '')
  hex = hex.chars.map { |c| c*2 }.join if hex.length == 3
  r, g, b = hex.scan(/../).map { |x| x.to_i(16) }
  [r, g, b]
end

def rgb_to_hex(r, g, b)
  "##{r.to_s(16).rjust(2, '0')}#{g.to_s(16).rjust(2, '0')}#{b.to_s(16).rjust(2, '0')}"
end

def rgb_to_hsl(r, g, b)
  rf, gf, bf = r/255.0, g/255.0, b/255.0
  max = [rf, gf, bf].max
  min = [rf, gf, bf].min
  l = (max + min) / 2.0
  if max == min
    return [0, 0, l]
  end
  diff = max - min
  s = l < 0.5 ? diff / (max + min) : diff / (2.0 - max - min)
  h = if max == rf
    (gf - bf) / diff + (gf < bf ? 6 : 0)
  elsif max == gf
    (bf - rf) / diff + 2
  else
    (rf - gf) / diff + 4
  end
  h /= 6.0
  [h, s, l]
end

def hsl_to_rgb(h, s, l)
  if s == 0
    r = g = b = l
  else
    v2 = l < 0.5 ? l * (1 + s) : l + s - l * s
    v1 = 2 * l - v2
    hue_to_rgb = ->(hh) {
      hh += 1 if hh < 0
      hh -= 1 if hh > 1
      if 6 * hh < 1
        v1 + (v2 - v1) * 6 * hh
      elsif 2 * hh < 1
        v2
      elsif 3 * hh < 2
        v1 + (v2 - v1) * (2.0/3.0 - hh) * 6
      else
        v1
      end
    }
    r = hue_to_rgb.call(h + 1.0/3.0)
    g = hue_to_rgb.call(h)
    b = hue_to_rgb.call(h - 1.0/3.0)
  end
  [(r * 255).round, (g * 255).round, (b * 255).round]
end

def generate_palette(hex)
  r, g, b = hex_to_rgb(hex)
  h, s, l = rgb_to_hsl(r, g, b)

  lightness = [0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18]
  names = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900]
  shades = {}
  lightness.each_with_index do |lvl, i|
    rr, gg, bb = hsl_to_rgb(h, s, lvl)
    shades[names[i].to_s] = rgb_to_hex(rr, gg, bb)
  end

  accent_l = [0.60, 0.70, 0.75, 0.80]
  accent_s = [0.95, 0.90, 0.85, 0.80]
  accent_names = ['A100', 'A200', 'A400', 'A700']
  accents = {}
  accent_names.each_with_index do |name, i|
    rr, gg, bb = hsl_to_rgb(h, accent_s[i], accent_l[i])
    accents[name] = rgb_to_hex(rr, gg, bb)
  end

  contrast = {}
  shades.each do |name, hex_c|
    rr, gg, bb = hex_to_rgb(hex_c)
    lum = (0.299 * rr + 0.587 * gg + 0.114 * bb) / 255.0
    contrast[name] = lum < 0.5 ? '#ffffff' : '#000000'
  end
  [shades, accents, contrast]
end

def print_palette(shades, accents, contrast)
  puts "\n--- Material Palette ---"
  shades.each do |name, hex_c|
    r, g, b = hex_to_rgb(hex_c)
    tr, tg, tb = hex_to_rgb(contrast[name])
    print "\033[48;2;#{r};#{g};#{b}m\033[38;2;#{tr};#{tg};#{tb}m"
    puts " #{name.rjust(4)} #{hex_c} \033[0m"
  end
  puts "\n--- Accents ---"
  accents.each do |name, hex_c|
    r, g, b = hex_to_rgb(hex_c)
    print "\033[48;2;#{r};#{g};#{b}m"
    puts " #{name.rjust(4)} #{hex_c} \033[0m"
  end
end

options = {}
OptionParser.new do |opts|
  opts.banner = "Использование: ruby palette.rb <HEX-цвет> [--json] [--css] [--scss]"
  opts.on("--json", "Вывод в JSON") { options[:json] = true }
  opts.on("--css", "Вывод CSS-переменных") { options[:css] = true }
  opts.on("--scss", "Вывод SCSS-переменных") { options[:scss] = true }
end.parse!

color = ARGV[0]
unless color
  puts "Укажите HEX-цвет, например #6200EE"
  exit 1
end

begin
  shades, accents, contrast = generate_palette(color)
rescue => e
  puts "Ошибка: #{e.message}"
  exit 1
end

if options[:json]
  puts JSON.pretty_generate({shades: shades, accents: accents, contrast: contrast})
elsif options[:css]
  puts ":root {"
  shades.each { |k, v| puts "  --md-#{k}: #{v};" }
  accents.each { |k, v| puts "  --md-#{k}: #{v};" }
  puts "}"
elsif options[:scss]
  shades.each { |k, v| puts "$md-#{k}: #{v};" }
  accents.each { |k, v| puts "$md-#{k}: #{v};" }
else
  print_palette(shades, accents, contrast)
end
