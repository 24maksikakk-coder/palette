// palette.cpp
#include <iostream>
#include <string>
#include <map>
#include <vector>
#include <sstream>
#include <iomanip>
#include <cmath>
#include <cstdlib>

using namespace std;

struct RGB { int r, g, b; };
struct HSL { double h, s, l; };

RGB hexToRGB(const string& hex) {
    string s = hex;
    if (s[0] == '#') s = s.substr(1);
    if (s.length() == 3) {
        string t = "";
        for (char c : s) t += c + c;
        s = t;
    }
    int r, g, b;
    stringstream ss;
    ss << hex << s.substr(0,2);
    ss >> r;
    ss.clear();
    ss << hex << s.substr(2,2);
    ss >> g;
    ss.clear();
    ss << hex << s.substr(4,2);
    ss >> b;
    return {r, g, b};
}

string rgbToHex(int r, int g, int b) {
    stringstream ss;
    ss << "#" << setfill('0') << setw(2) << hex << r
       << setw(2) << hex << g << setw(2) << hex << b;
    return ss.str();
}

HSL rgbToHSL(int r, int g, int b) {
    double rf = r/255.0, gf = g/255.0, bf = b/255.0;
    double max = std::max({rf, gf, bf});
    double min = std::min({rf, gf, bf});
    double l = (max + min) / 2.0;
    double h, s;
    if (max == min) {
        h = 0; s = 0;
    } else {
        double diff = max - min;
        s = (l < 0.5) ? diff / (max + min) : diff / (2.0 - max - min);
        if (max == rf) {
            h = (gf - bf) / diff + (gf < bf ? 6.0 : 0.0);
        } else if (max == gf) {
            h = (bf - rf) / diff + 2.0;
        } else {
            h = (rf - gf) / diff + 4.0;
        }
        h /= 6.0;
    }
    return {h, s, l};
}

RGB hslToRGB(double h, double s, double l) {
    double r, g, b;
    if (s == 0) {
        r = g = b = l;
    } else {
        double v2 = (l < 0.5) ? l * (1 + s) : l + s - l*s;
        double v1 = 2*l - v2;
        auto hueToRGB = [&](double hh) {
            if (hh < 0) hh += 1;
            if (hh > 1) hh -= 1;
            if (6*hh < 1) return v1 + (v2-v1)*6*hh;
            if (2*hh < 1) return v2;
            if (3*hh < 2) return v1 + (v2-v1)*(2.0/3.0 - hh)*6;
            return v1;
        };
        r = hueToRGB(h + 1.0/3.0);
        g = hueToRGB(h);
        b = hueToRGB(h - 1.0/3.0);
    }
    return {(int)(r*255), (int)(g*255), (int)(b*255)};
}

void generatePalette(const string& hex, map<string, string>& shades, map<string, string>& accents, map<string, string>& contrast) {
    RGB rgb = hexToRGB(hex);
    HSL hsl = rgbToHSL(rgb.r, rgb.g, rgb.b);

    vector<double> lightness = {0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18};
    vector<int> names = {50, 100, 200, 300, 400, 500, 600, 700, 800, 900};
    for (size_t i=0; i<lightness.size(); ++i) {
        RGB c = hslToRGB(hsl.h, hsl.s, lightness[i]);
        shades[to_string(names[i])] = rgbToHex(c.r, c.g, c.b);
    }

    vector<double> accentL = {0.60, 0.70, 0.75, 0.80};
    vector<double> accentS = {0.95, 0.90, 0.85, 0.80};
    vector<string> accentNames = {"A100", "A200", "A400", "A700"};
    for (size_t i=0; i<accentNames.size(); ++i) {
        RGB c = hslToRGB(hsl.h, accentS[i], accentL[i]);
        accents[accentNames[i]] = rgbToHex(c.r, c.g, c.b);
    }

    for (auto& p : shades) {
        RGB c = hexToRGB(p.second);
        double lum = (0.299*c.r + 0.587*c.g + 0.114*c.b) / 255.0;
        contrast[p.first] = (lum < 0.5) ? "#ffffff" : "#000000";
    }
}

void printPalette(const map<string, string>& shades, const map<string, string>& accents, const map<string, string>& contrast) {
    cout << "\n--- Material Palette ---" << endl;
    for (auto& p : shades) {
        RGB bg = hexToRGB(p.second);
        RGB fg = hexToRGB(contrast.at(p.first));
        cout << "\033[48;2;" << bg.r << ";" << bg.g << ";" << bg.b << "m"
             << "\033[38;2;" << fg.r << ";" << fg.g << ";" << fg.b << "m"
             << " " << setw(4) << p.first << " " << p.second << " \033[0m" << endl;
    }
    cout << "\n--- Accents ---" << endl;
    for (auto& p : accents) {
        RGB bg = hexToRGB(p.second);
        cout << "\033[48;2;" << bg.r << ";" << bg.g << ";" << bg.b << "m"
             << " " << setw(4) << p.first << " " << p.second << " \033[0m" << endl;
    }
}

int main(int argc, char* argv[]) {
    if (argc < 2) {
        cerr << "Использование: palette <HEX-цвет> [--json] [--css] [--scss]" << endl;
        return 1;
    }
    string color = argv[1];
    bool jsonFlag = false, cssFlag = false, scssFlag = false;
    for (int i=2; i<argc; ++i) {
        string arg = argv[i];
        if (arg == "--json") jsonFlag = true;
        else if (arg == "--css") cssFlag = true;
        else if (arg == "--scss") scssFlag = true;
    }

    map<string, string> shades, accents, contrast;
    try {
        generatePalette(color, shades, accents, contrast);
    } catch (...) {
        cerr << "Ошибка генерации палитры" << endl;
        return 1;
    }

    if (jsonFlag) {
        cout << "{\"shades\":{";
        int i=0;
        for (auto& p : shades) {
            if (i++) cout << ",";
            cout << "\"" << p.first << "\":\"" << p.second << "\"";
        }
        cout << "},\"accents\":{";
        i=0;
        for (auto& p : accents) {
            if (i++) cout << ",";
            cout << "\"" << p.first << "\":\"" << p.second << "\"";
        }
        cout << "},\"contrast\":{";
        i=0;
        for (auto& p : contrast) {
            if (i++) cout << ",";
            cout << "\"" << p.first << "\":\"" << p.second << "\"";
        }
        cout << "}}\n";
    } else if (cssFlag) {
        cout << ":root {" << endl;
        for (auto& p : shades) cout << "  --md-" << p.first << ": " << p.second << ";" << endl;
        for (auto& p : accents) cout << "  --md-" << p.first << ": " << p.second << ";" << endl;
        cout << "}" << endl;
    } else if (scssFlag) {
        for (auto& p : shades) cout << "$md-" << p.first << ": " << p.second << ";" << endl;
        for (auto& p : accents) cout << "$md-" << p.first << ": " << p.second << ";" << endl;
    } else {
        printPalette(shades, accents, contrast);
    }
    return 0;
}
