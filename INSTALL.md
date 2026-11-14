# Встановлення інструментів

Cargo автоматично завантажить Rust-залежності з `Cargo.toml`.

## Linux

Встановіть Rust офіційним rustup інсталятором:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustup update stable
```

Ubuntu/Debian:

```sh
sudo apt update
sudo apt install build-essential pkg-config libx11-dev libxkbcommon-dev libwayland-dev libegl1-mesa-dev libgl1-mesa-dev
```

Fedora:

```sh
sudo dnf install gcc gcc-c++ make pkgconf-pkg-config libX11-devel libxkbcommon-devel wayland-devel mesa-libEGL-devel mesa-libGL-devel
```

## Windows

PowerShell: встановіть Rust і Visual Studio Build Tools з підтримкою C++:

```powershell
winget install --id Rustlang.Rustup --exact
winget install --id Microsoft.VisualStudio.2022.BuildTools --exact
```

У Visual Studio Installer виберіть workload **Desktop development with C++** (включно з Windows SDK), а потім перезапустіть термінал.

## Запуск застосунку

```sh
rustc --version
cargo --version
cargo run
```

На Windows ті самі команди працюють у PowerShell.

Збірка Windows з GitHub Actions вбудовує логотип `assets/logo.ico` у `vfile.exe`. Для Linux ярлик у меню програм можна встановити командами з README; вони копіюють застосунок у `~/.local/bin`, файл ярлика `.desktop` та SVG-іконку у каталоги поточного користувача.
