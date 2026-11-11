# VFile

> A modern, cross-platform file manager built with Rust and egui.
>
> Сучасний кросплатформний файловий менеджер, створений на Rust та egui.

## English

VFile is a lightweight desktop file manager for Linux and Windows. It provides a clean graphical interface for browsing folders and managing everyday files.

### Features

- Browse folders and navigate using the address bar or quick locations.
- Search the current folder by file or folder name.
- Create folders and empty `.txt` files.
- Rename selected files and folders.
- Select and delete files or folders with a confirmation step.
- Open files with the operating system's default application.
- Build a standalone Windows executable for launching from File Explorer.

### Run from source

Install Rust and the platform dependencies listed in [INSTALL.md](INSTALL.md), then run:

```sh
cargo run
```

Build an optimized release:

```sh
cargo build --release
```

On Windows, the executable is `target/release/vfile.exe` and can be launched by double-clicking it. The **Windows release** GitHub Actions workflow also builds a downloadable executable artifact. Pushing a version tag such as `v1.1` publishes the executable as a GitHub Release asset.

#### Cross-compile a Windows executable from Linux

On Ubuntu/Debian, install MinGW and the Rust Windows GNU target, then build:

```sh
sudo apt update
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be at `target/x86_64-pc-windows-gnu/release/vfile.exe`.

## Українською

VFile — легкий файловий менеджер для Linux і Windows. Він має простий графічний інтерфейс для перегляду папок і роботи з файлами.

### Можливості

- Перегляд папок і навігація через адресний рядок та швидкі посилання.
- Пошук файлів і папок у поточному каталозі.
- Створення папок і порожніх файлів `.txt`.
- Перейменування вибраних файлів і папок.
- Вибір і видалення файлів або папок із підтвердженням.
- Відкриття файлів у стандартній програмі операційної системи.
- Збірка окремого Windows `.exe`, який можна запускати з Провідника.

### Запуск із вихідного коду

Встановіть Rust і системні залежності, наведені в [INSTALL.md](INSTALL.md), а потім виконайте:

```sh
cargo run
```

Зібрати оптимізовану версію можна командою:

```sh
cargo build --release
```

У Windows виконуваний файл буде в `target/release/vfile.exe`; його можна запустити подвійним клацанням. Workflow **Windows release** у GitHub Actions також створює завантажуваний `.exe`. Якщо відправити тег версії, наприклад `v1.1`, виконуваний файл буде опублікований як вкладення GitHub Release.

#### Збірка Windows `.exe` з Linux

В Ubuntu/Debian встановіть MinGW та Windows-ціль Rust і зберіть застосунок:

```sh
sudo apt update
sudo apt install gcc-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Файл буде тут: `target/x86_64-pc-windows-gnu/release/vfile.exe`.

---

## GitHub About

**English:** A modern Rust file manager for Linux and Windows, built with egui.

**Українською:** Сучасний файловий менеджер на Rust для Linux і Windows, створений з egui.
# VFIle
