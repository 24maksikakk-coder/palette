// palette.js
const yargs = require('yargs/yargs');
const { hideBin } = require('yargs/helpers');

function hexToRgb(hex) {
    hex = hex.replace(/^#/, '');
    if (hex.length === 3) {
        hex = hex.split('').map(c => c+c).join('');
    }
    const intVal = parseInt(hex, 16);
    return [(intVal >> 16) & 0xFF, (intVal >> 8) & 0xFF, intVal & 0xFF];
}

function rgbToHex(r, g, b) {
    return '#' + [r, g, b].map(c => c.toString(16).padStart(2, '0')).join('');
}

function rgbToHsl(r, g, b) {
    const rf = r / 255, gf = g / 255, bf = b / 255;
    const max = Math.max(rf, gf, bf);
    const min = Math.min(rf, gf, bf);
    let h, s, l = (max + min) / 2;
    if (max === min) {
        h = s = 0;
    } else {
        const d = max - min;
        s = l < 0.5 ? d / (max + min) : d / (2 - max - min);
        if (max === rf) {
            h = (gf - bf) / d + (gf < bf ? 6 : 0);
        } else if (max === gf) {
            h = (bf - rf) / d + 2;
        } else {
            h = (rf - gf) / d + 4;
        }
        h /= 6;
    }
    return [h, s, l];
}

function hslToRgb(h, s, l) {
    let r, g, b;
    if (s === 0) {
        r = g = b = l;
    } else {
        const v2 = l < 0.5 ? l * (1 + s) : l + s - l * s;
        const v1 = 2 * l - v2;
        const hueToRgb = (hh) => {
            if (hh < 0) hh += 1;
            if (hh > 1) hh -= 1;
            if (6 * hh < 1) return v1 + (v2 - v1) * 6 * hh;
            if (2 * hh < 1) return v2;
            if (3 * hh < 2) return v1 + (v2 - v1) * (2/3 - hh) * 6;
            return v1;
        };
        r = hueToRgb(h + 1/3);
        g = hueToRgb(h);
        b = hueToRgb(h - 1/3);
    }
    return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
}

function generatePalette(hex) {
    const [r, g, b] = hexToRgb(hex);
    const [h, s, l] = rgbToHsl(r, g, b);

    const lightness = [0.90, 0.82, 0.74, 0.66, 0.58, 0.50, 0.42, 0.34, 0.26, 0.18];
    const names = [50, 100, 200, 300, 400, 500, 600, 700, 800, 900];
    const shades = {};
    lightness.forEach((lvl, i) => {
        const [rr, gg, bb] = hslToRgb(h, s, lvl);
        shades[names[i]] = rgbToHex(rr, gg, bb);
    });

    const accentL = [0.60, 0.70, 0.75, 0.80];
    const accentS = [0.95, 0.90, 0.85, 0.80];
    const accentNames = ['A100', 'A200', 'A400', 'A700'];
    const accents = {};
    accentNames.forEach((name, i) => {
        const [rr, gg, bb] = hslToRgb(h, accentS[i], accentL[i]);
        accents[name] = rgbToHex(rr, gg, bb);
    });

    const contrast = {};
    Object.entries(shades).forEach(([name, hexC]) => {
        const [rr, gg, bb] = hexToRgb(hexC);
        const lum = (0.299 * rr + 0.587 * gg + 0.114 * bb) / 255;
        contrast[name] = lum < 0.5 ? '#ffffff' : '#000000';
    });
    return { shades, accents, contrast };
}

function printPalette(shades, accents, contrast) {
    console.log('\n--- Material Palette ---');
    Object.entries(shades).forEach(([name, hexC]) => {
        const [r, g, b] = hexToRgb(hexC);
        const [tr, tg, tb] = hexToRgb(contrast[name]);
        console.log(`\x1b[48;2;${r};${g};${b}m\x1b[38;2;${tr};${tg};${tb}m ${name.padStart(4)} ${hexC} \x1b[0m`);
    });
    console.log('\n--- Accents ---');
    Object.entries(accents).forEach(([name, hexC]) => {
        const [r, g, b] = hexToRgb(hexC);
        console.log(`\x1b[48;2;${r};${g};${b}m ${name.padStart(4)} ${hexC} \x1b[0m`);
    });
}

async function main() {
    const argv = yargs(hideBin(process.argv))
        .usage('Использование: $0 <HEX-цвет> [--json] [--css] [--scss]')
        .option('json', { type: 'boolean', description: 'Вывод в JSON' })
        .option('css', { type: 'boolean', description: 'Вывод CSS-переменных' })
        .option('scss', { type: 'boolean', description: 'Вывод SCSS-переменных' })
        .help()
        .parse();

    const color = argv._[0];
    if (!color) {
        console.error('Укажите HEX-цвет, например #6200EE');
        process.exit(1);
    }

    try {
        const { shades, accents, contrast } = generatePalette(color);
        if (argv.json) {
            console.log(JSON.stringify({ shades, accents, contrast }, null, 2));
        } else if (argv.css) {
            console.log(':root {');
            Object.entries(shades).forEach(([k, v]) => console.log(`  --md-${k}: ${v};`));
            Object.entries(accents).forEach(([k, v]) => console.log(`  --md-${k}: ${v};`));
            console.log('}');
        } else if (argv.scss) {
            Object.entries(shades).forEach(([k, v]) => console.log(`$md-${k}: ${v};`));
            Object.entries(accents).forEach(([k, v]) => console.log(`$md-${k}: ${v};`));
        } else {
            printPalette(shades, accents, contrast);
        }
    } catch (err) {
        console.error(`Ошибка: ${err.message}`);
        process.exit(1);
    }
}

main();
