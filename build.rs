// Cargo автоматично запускає цей файл під час збірки проєкту.
// Cargo automatically runs this file while building the project.
fn main() {
    // Повторно запустити скрипт, якщо зміниться файл іконки Windows.
    // Rerun this script whenever the Windows icon file changes.
    println!("cargo:rerun-if-changed=assets/logo.ico");

    // Для Windows вбудовуємо логотип у .exe, щоб файл застосунку мав власну іконку.
    // Embed the logo in the Windows .exe so the application file has its own icon.
    // Для Linux цей крок не потрібен: іконка ярлика задається у .desktop-файлі.
    // Linux does not need this step; the launcher icon is specified in the .desktop file.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/logo.ico")
            .compile()
            .expect("failed to embed the VFile Windows icon");
    }
}
