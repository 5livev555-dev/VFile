#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{fs, path::{Path, PathBuf}};

use eframe::egui;

struct VFile {
    current: PathBuf,
    entries: Vec<Entry>,
    search: String,
    status: String,
    selected: Option<PathBuf>,
    confirm_delete: bool,
    rename_input: String,
    rename_dialog: bool,
}

struct Entry { path: PathBuf, is_dir: bool, size: u64 }

impl VFile {
    fn new() -> Self {
        let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = Self { current, entries: Vec::new(), search: String::new(), status: String::new(), selected: None, confirm_delete: false, rename_input: String::new(), rename_dialog: false };
        app.refresh();
        app
    }

    fn refresh(&mut self) {
        self.entries = fs::read_dir(&self.current).into_iter().flatten().filter_map(Result::ok)
            .filter_map(|item| {
                let path = item.path();
                let metadata = item.metadata().ok()?;
                Some(Entry { path, is_dir: metadata.is_dir(), size: metadata.len() })
            }).collect();
        self.entries.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.path.file_name().cmp(&b.path.file_name())));
    }

    fn navigate(&mut self, path: PathBuf) {
        if path.is_dir() { self.current = path; self.refresh(); self.status.clear(); }
        else { self.status = "Не вдалося відкрити папку".into(); }
    }

    fn create_folder(&mut self) {
        let mut name = "Нова папка".to_string();
        let mut path = self.current.join(&name);
        let mut n = 2;
        while path.exists() { name = format!("Нова папка {n}"); path = self.current.join(&name); n += 1; }
        match fs::create_dir(&path) {
            Ok(()) => { self.status = format!("Створено: {name}"); self.refresh(); }
            Err(e) => self.status = format!("Не вдалося створити папку: {e}"),
        }
    }

    fn create_text_file(&mut self) {
        let mut name = "Новий файл.txt".to_string();
        let mut path = self.current.join(&name);
        let mut n = 2;
        while path.exists() { name = format!("Новий файл {n}.txt"); path = self.current.join(&name); n += 1; }
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(_) => { self.status = format!("Створено: {name}"); self.selected = Some(path); self.refresh(); }
            Err(e) => self.status = format!("Не вдалося створити файл: {e}"),
        }
    }

    fn delete_selected(&mut self) {
        let Some(path) = self.selected.clone() else { return; };
        // Only allow deleting an item in the currently displayed directory.
        if path.parent() != Some(self.current.as_path()) {
            self.status = "Вибраний елемент не належить цій папці".into();
            self.confirm_delete = false;
            return;
        }
        let result = match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_dir() => fs::remove_dir_all(&path),
            Ok(_) => fs::remove_file(&path),
            Err(e) => Err(e),
        };
        match result {
            Ok(()) => { self.status = "Елемент видалено".into(); self.selected = None; self.refresh(); }
            Err(e) => self.status = format!("Не вдалося видалити елемент: {e}"),
        }
        self.confirm_delete = false;
    }

    fn rename_selected(&mut self) {
        let Some(old_path) = self.selected.clone() else { return; };
        let name = self.rename_input.trim();
        if name.is_empty() || name == "." || name == ".." || name.chars().any(|c| matches!(c, '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*')) || name.ends_with([' ', '.']) {
            self.status = "Вкажіть допустиму назву без символів шляху".into();
            return;
        }
        if old_path.parent() != Some(self.current.as_path()) {
            self.status = "Вибраний елемент не належить цій папці".into();
            self.rename_dialog = false;
            return;
        }
        let new_path = self.current.join(name);
        if new_path != old_path && new_path.exists() {
            self.status = "Елемент із такою назвою вже існує".into();
            return;
        }
        match fs::rename(&old_path, &new_path) {
            Ok(()) => {
                self.status = format!("Перейменовано на «{name}»");
                self.selected = Some(new_path);
                self.rename_dialog = false;
                self.refresh();
            }
            Err(e) => self.status = format!("Не вдалося перейменувати: {e}"),
        }
    }
}

impl eframe::App for VFile {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::top("toolbar").frame(egui::Frame::new().fill(egui::Color32::from_rgb(24, 27, 34)).inner_margin(egui::Margin::symmetric(18, 14))).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(egui::RichText::new("VFile").color(egui::Color32::from_rgb(130, 170, 255)));
                ui.add_space(16.0);
                if ui.button("←").on_hover_text("На рівень вище").clicked() {
                    if let Some(parent) = self.current.parent() { self.current = parent.to_path_buf(); self.refresh(); }
                }
                if ui.button("⟳").on_hover_text("Оновити").clicked() { self.refresh(); }
                let mut address = self.current.display().to_string();
                let response = ui.add(egui::TextEdit::singleline(&mut address).desired_width(ui.available_width() * 0.55).hint_text("Шлях до папки"));
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { self.navigate(PathBuf::from(address)); }
                ui.add_space(8.0);
                ui.add(egui::TextEdit::singleline(&mut self.search).desired_width(190.0).hint_text("⌕  Пошук у папці"));
            });
        });

        egui::Panel::left("sidebar").resizable(false).default_size(185.0).frame(egui::Frame::new().fill(egui::Color32::from_rgb(27, 30, 38)).inner_margin(egui::Margin::same(14))).show(ui, |ui| {
            ui.label(egui::RichText::new("РОЗТАШУВАННЯ").small().weak());
            ui.add_space(10.0);
            let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
            let desktop = dirs::desktop_dir().unwrap_or_else(|| home.join("Desktop"));
            let downloads = dirs::download_dir().unwrap_or_else(|| home.join("Downloads"));
            // `C:` is drive-relative on Windows; `C:\` is the drive root.
            let root = if cfg!(windows) {
                let drive = std::env::var("SystemDrive").unwrap_or_else(|_| "C:".into());
                PathBuf::from(format!("{}\\", drive.trim_end_matches(['\\', '/'])))
            } else {
                PathBuf::from("/")
            };
            for (label, icon, path) in [("Домівка", "⌂", home), ("Робочий стіл", "▦", desktop), ("Завантаження", "↓", downloads), ("Корінь диска", "▣", root)] {
                if ui.selectable_label(self.current == path, format!("{icon}  {label}")).clicked() { self.navigate(path); }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| { ui.label(egui::RichText::new("VFile • 1.1").small().weak()); });
        });

        egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::from_rgb(19, 22, 28)).inner_margin(egui::Margin::symmetric(24, 20))).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Файли");
                ui.add_space(12.0);
                ui.label(egui::RichText::new(self.current.display().to_string()).weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_enabled(self.selected.is_some(), egui::Button::new("⌫ Видалити")).clicked() { self.confirm_delete = true; }
                    if ui.add_enabled(self.selected.is_some(), egui::Button::new("Перейменувати")).clicked() {
                        self.rename_input = self.selected.as_ref().and_then(|p| p.file_name()).unwrap_or_default().to_string_lossy().to_string();
                        self.rename_dialog = true;
                    }
                    if ui.button("＋ .txt файл").clicked() { self.create_text_file(); }
                    if ui.button("＋ Нова папка").clicked() { self.create_folder(); }
                });
            });
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("ІМ’Я").small().weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(egui::RichText::new("РОЗМІР").small().weak()); });
            });
            ui.separator();
            let query = self.search.to_lowercase();
            let mut open_dir: Option<PathBuf> = None;
            let mut open_file: Option<PathBuf> = None;
            let mut selected: Option<PathBuf> = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                for entry in &self.entries {
                    let name = entry.path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if !name.to_lowercase().contains(&query) { continue; }
                    ui.horizontal(|ui| {
                        let icon = if entry.is_dir { "📁" } else { file_icon(&name) };
                        let response = ui.selectable_label(self.selected.as_ref() == Some(&entry.path), format!("{icon}  {name}"));
                        if response.clicked() { selected = Some(entry.path.clone()); }
                        if response.double_clicked() {
                            if entry.is_dir { open_dir = Some(entry.path.clone()); } else { open_file = Some(entry.path.clone()); }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(if entry.is_dir { "—".into() } else { format_size(entry.size) }).weak());
                        });
                    });
                    ui.separator();
                }
            });
            if let Some(path) = selected { self.selected = Some(path); }
            if let Some(path) = open_dir { self.navigate(path); }
            if let Some(path) = open_file { if let Err(e) = opener::open(&path) { self.status = format!("Не вдалося відкрити файл: {e}"); } }
            if !self.status.is_empty() { ui.colored_label(egui::Color32::LIGHT_RED, &self.status); }
        });

        if self.confirm_delete {
            let name = self.selected.as_ref().and_then(|p| p.file_name()).unwrap_or_default().to_string_lossy().to_string();
            egui::Window::new("Підтвердити видалення").collapsible(false).resizable(false).show(ui.ctx(), |ui| {
                ui.label(format!("Видалити «{name}»? Папка та її вміст будуть видалені."));
                ui.horizontal(|ui| {
                    if ui.button("Видалити").clicked() { self.delete_selected(); }
                    if ui.button("Скасувати").clicked() { self.confirm_delete = false; }
                });
            });
        }

        if self.rename_dialog {
            egui::Window::new("Перейменувати").collapsible(false).resizable(false).show(ui.ctx(), |ui| {
                ui.label("Нова назва:");
                let input = ui.text_edit_singleline(&mut self.rename_input);
                let enter = input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                ui.horizontal(|ui| {
                    let rename = ui.button("Зберегти").clicked() || enter;
                    if rename { self.rename_selected(); }
                    if ui.button("Скасувати").clicked() { self.rename_dialog = false; }
                });
            });
        }
    }
}

fn file_icon(name: &str) -> &'static str {
    match Path::new(name).extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "webp" | "svg" => "🖼️",
        "mp4" | "mkv" | "mov" | "avi" => "🎬",
        "mp3" | "wav" | "flac" | "ogg" => "🎵",
        "zip" | "gz" | "7z" | "tar" | "rar" => "📦",
        "rs" | "toml" | "py" | "js" | "ts" | "html" | "css" => "💻",
        "pdf" | "doc" | "docx" | "txt" | "md" => "📄",
        _ => "📃",
    }
}

fn format_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["Б", "КБ", "МБ", "ГБ"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 { size /= 1024.0; unit += 1; }
    format!("{size:.1} {}", UNITS[unit])
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1100.0, 720.0]).with_min_inner_size([760.0, 500.0]),
        ..Default::default()
    };
    eframe::run_native("VFile", options, Box::new(|_| Ok(Box::new(VFile::new()))))
}
