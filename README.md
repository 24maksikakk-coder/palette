🎨 Генератор цветовых палитр (Material)
Версия: 1.0.0 | Лицензия: MIT | Статус: ✅ Активная разработка

https://img.shields.io/github/repo-size/yourusername/material-palette https://img.shields.io/github/last-commit/yourusername/material-palette https://img.shields.io/github/languages/count/yourusername/material-palette

🎨 Описание
Генератор цветовых палитр (Material) — это консольная утилита для создания полных палитр в стиле Material Design на основе одного основного цвета. Программа генерирует:

10 оттенков (50, 100, 200, 300, 400, 500, 600, 700, 800, 900)

4 акцентных цвета (A100, A200, A400, A700)

Контрастный цвет текста (белый или чёрный) для каждого оттенка

Палитры выводятся в консоль с цветными блоками (True Color) и HEX-кодами. Поддерживается экспорт в JSON, CSS-переменные и SCSS.

Проект содержит 8 полноценных реализаций на разных языках программирования — выберите свой любимый язык и генерируйте красивые палитры для своих проектов!

✨ Возможности
Функция	Описание
Генерация оттенков	50, 100, 200, 300, 400, 500, 600, 700, 800, 900
Акцентные цвета	A100, A200, A400, A700 (более насыщенные)
Контрастный текст	Автоматический выбор белого или чёрного текста
Цветной вывод	Отображение цветовых блоков в терминале (True Color)
Экспорт	JSON, CSS-переменные, SCSS
Кроссплатформенность	Работает на Linux, macOS, Windows
📦 Установка и запуск
Каждая реализация находится в отдельной папке. Для запуска требуется соответствующий компилятор/интерпретатор.

Язык	Файл	Команда запуска
Python	palette.py	python3 palette.py #6200EE
Go	palette.go	go run palette.go #6200EE
Rust	palette.rs	cargo run -- #6200EE
C++	palette.cpp	g++ -std=c++17 -o palette palette.cpp && ./palette #6200EE
Java	Palette.java	javac Palette.java && java Palette #6200EE
C#	palette.cs	dotnet run #6200EE
Ruby	palette.rb	ruby palette.rb #6200EE
Node.js	palette.js	node palette.js #6200EE
Примечание: Все версии поддерживают опции: --json, --css, --scss для экспорта.

📂 Структура репозитория
text
.
├── README.md
├── python/
│   └── palette.py
├── go/
│   └── palette.go
├── rust/
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
├── cpp/
│   └── palette.cpp
├── java/
│   └── Palette.java
├── csharp/
│   └── palette.cs
├── ruby/
│   └── palette.rb
└── javascript/
    ├── package.json
    └── palette.js
🎮 Использование
bash
# Базовый вывод палитры
palette #6200EE

# Экспорт в JSON
palette #6200EE --json

# Экспорт в CSS-переменные
palette #6200EE --css

# Экспорт в SCSS
palette #6200EE --scss

# Помощь
palette --help
🛠️ Особенности реализаций
Python – использует colorsys для преобразования цветов, argparse для опций.

Go – встроенные image/color и strconv, флаги из flag.

Rust – clap для парсинга, colored для цвета (опционально).

C++ – ручное преобразование HSV/HSL, стандартный вывод.

Java – java.awt.Color для работы с цветами.

C# – System.Drawing.Color (или System.Windows.Media в WPF).

Ruby – встроенный модуль color (или Color gem).

Node.js – yargs для опций, chalk для цвета (опционально).

Для консольного вывода используются ANSI-коды с поддержкой True Color (24-bit) для отображения цветных блоков.

🤝 Вклад
PR и issues приветствуются. Добавляйте поддержку новых форматов экспорта, улучшайте алгоритмы.

📄 Лицензия
MIT License.
