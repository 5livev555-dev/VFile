<div align="center">
  <img src="assets/logo.png" alt="VFile logo" width="160">
  <h1>VFile</h1>
  <p>A modern, cross-platform file manager built with Rust and egui.<br>
  Сучасний кросплатформний файловий менеджер, створений на Rust та egui.</p>
</div>

## English

VFile is a lightweight desktop file manager for Linux and Windows. It provides a clean graphical interface for browsing folders and managing everyday files.

The VFile logo is shown in the app toolbar and window. Windows release builds embed it as the executable icon.

On first launch, choose Ukrainian or English. VFile saves the choice in your user configuration and uses it on later launches.

### Features

- Browse folders and navigate using the address bar or quick locations.
- Select multiple files or folders with Ctrl-click and delete them together.
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

On Windows, the executable is `target/release/vfile.exe` and can be launched by double-clicking it. The **Windows release** GitHub Actions workflow also builds a downloadable executable artifact. Pushing a version tag such as `v1.2` publishes the executable as a GitHub Release asset.

#### Cross-compile a Windows executable from Linux

On Ubuntu/Debian, install MinGW and the Rust Windows GNU target, then build:

```sh
sudo apt update
sudo apt install gcc-mingw-w64-x86-64 binutils-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

The executable will be at `target/x86_64-pc-windows-gnu/release/vfile.exe`.

On Linux, install a desktop launcher and icon with the commands below (after building the release):

```sh
mkdir -p ~/.local/bin ~/.local/share/applications ~/.local/share/icons/hicolor/scalable/apps
install -Dm755 target/release/vfile ~/.local/bin/vfile
install -Dm644 assets/logo.svg ~/.local/share/icons/hicolor/scalable/apps/vfile.svg
install -Dm644 packaging/linux/vfile.desktop ~/.local/share/applications/vfile.desktop
```

## Українською

VFile — легкий файловий менеджер для Linux і Windows. Він має простий графічний інтерфейс для перегляду папок і роботи з файлами.

Логотип VFile показується у панелі застосунку та у заголовку вікна. У збірках для Windows він також вбудовується як піктограма `.exe`.

Під час першого запуску виберіть українську або англійську мову. VFile збереже вибір у конфігурації користувача й застосовуватиме його під час наступних запусків.

### Можливості

- Перегляд папок і навігація через адресний рядок та швидкі посилання.
- Вибір кількох файлів чи папок за допомогою Ctrl+клацання та спільне видалення.
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

У Windows виконуваний файл буде в `target/release/vfile.exe`; його можна запустити подвійним клацанням. Workflow **Windows release** у GitHub Actions також створює завантажуваний `.exe`. Якщо відправити тег версії, наприклад `v1.2`, виконуваний файл буде опублікований як вкладення GitHub Release.

#### Збірка Windows `.exe` з Linux

В Ubuntu/Debian встановіть MinGW та Windows-ціль Rust і зберіть застосунок:

```sh
sudo apt update
sudo apt install gcc-mingw-w64-x86-64 binutils-mingw-w64-x86-64
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Файл буде тут: `target/x86_64-pc-windows-gnu/release/vfile.exe`.

Щоб додати ярлик VFile у меню програм Linux та встановити іконку, після release-збірки виконайте:

```sh
mkdir -p ~/.local/bin ~/.local/share/applications ~/.local/share/icons/hicolor/scalable/apps
install -Dm755 target/release/vfile ~/.local/bin/vfile
install -Dm644 assets/logo.svg ~/.local/share/icons/hicolor/scalable/apps/vfile.svg
install -Dm644 packaging/linux/vfile.desktop ~/.local/share/applications/vfile.desktop
```

---

## GitHub About

**English:** A modern Rust file manager for Linux and Windows, built with egui.

**Українською:** Сучасний файловий менеджер на Rust для Linux і Windows, створений з egui.
# VFIle
