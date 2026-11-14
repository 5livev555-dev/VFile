#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::{collections::HashSet, fs, path::{Path, PathBuf}};

use eframe::egui;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Language { Ukrainian, English }

impl Language {
    fn text(self, ukrainian: &'static str, english: &'static str) -> &'static str {
        match self { Self::Ukrainian => ukrainian, Self::English => english }
    }
}

struct VFile {
    current: PathBuf,
    entries: Vec<Entry>,
    search: String,
    status: String,
    selected: HashSet<PathBuf>,
    confirm_delete: bool,
    rename_input: String,
    rename_dialog: bool,
    language: Option<Language>,
}

struct Entry { path: PathBuf, is_dir: bool, size: u64 }

impl VFile {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        let current = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut app = Self { current, entries: Vec::new(), search: String::new(), status: String::new(), selected: HashSet::new(), confirm_delete: false, rename_input: String::new(), rename_dialog: false, language: Self::load_language() };
        app.refresh();
        app
    }

    fn language_file() -> PathBuf {
        dirs::config_dir().unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")))
            .join("vfile").join("language")
    }

    fn load_language() -> Option<Language> {
        match fs::read_to_string(Self::language_file()).ok()?.trim() {
            "uk" => Some(Language::Ukrainian),
            "en" => Some(Language::English),
            _ => None,
        }
    }

    fn choose_language(&mut self, language: Language) {
        self.language = Some(language);
        let result = Self::language_file().parent().map(|dir| fs::create_dir_all(dir))
            .unwrap_or_else(|| Err(std::io::Error::other("could not find config directory")))
            .and_then(|()| fs::write(Self::language_file(), match language { Language::Ukrainian => "uk", Language::English => "en" }));
        if let Err(error) = result {
            self.status = format!("{}: {error}", language.text("Не вдалося зберегти мову", "Could not save language preference"));
        }
    }

    fn text(&self, ukrainian: &'static str, english: &'static str) -> &'static str {
        self.language.unwrap_or(Language::Ukrainian).text(ukrainian, english)
    }

    fn show_language_picker(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(egui::Color32::from_rgb(19, 22, 28)))
            .show(ui, |ui| {
                let top_space = ui.available_height() * 0.22;
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.add_space(top_space);
                    let title_width = ui.painter().layout_no_wrap(
                        "VFile".to_owned(),
                        egui::FontId::proportional(36.0),
                        egui::Color32::from_rgb(130, 170, 255),
                    ).size().x;
                    let row_width = 54.0 + 10.0 + title_width;
                    ui.allocate_ui_with_layout(
                        egui::vec2(row_width, 56.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            ui.add(egui::Image::from_bytes("bytes://assets/logo.svg", include_bytes!("../assets/logo.svg"))
                                .fit_to_exact_size(egui::vec2(54.0, 48.0)));
                            ui.add_space(10.0);
                            ui.heading(egui::RichText::new("VFile").size(36.0).color(egui::Color32::from_rgb(130, 170, 255)));
                        },
                    );
                    ui.add_space(40.0);
                    ui.label(egui::RichText::new("Виберіть мову / Select language").size(20.0));
                    ui.add_space(18.0);
                    let button_padding = ui.spacing().button_padding.x * 2.0;
                    let uk_width = ui.painter().layout_no_wrap(
                        "Українська".to_owned(), egui::FontId::proportional(17.0), ui.visuals().text_color(),
                    ).size().x + button_padding;
                    let en_width = ui.painter().layout_no_wrap(
                        "English".to_owned(), egui::FontId::proportional(17.0), ui.visuals().text_color(),
                    ).size().x + button_padding;
                    let button_row_width = uk_width + 12.0 + en_width;
                    ui.allocate_ui_with_layout(
                        egui::vec2(button_row_width, 44.0),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if ui.button(egui::RichText::new("Українська").size(17.0)).clicked() {
                                self.choose_language(Language::Ukrainian);
                            }
                            ui.add_space(12.0);
                            if ui.button(egui::RichText::new("English").size(17.0)).clicked() {
                                self.choose_language(Language::English);
                            }
                        },
                    );
                });
            });
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
        if path.is_dir() { self.current = path; self.selected.clear(); self.refresh(); self.status.clear(); }
        else { self.status = self.text("Не вдалося відкрити папку", "Could not open folder").into(); }
    }

    fn create_folder(&mut self) {
        let base = self.text("Нова папка", "New folder").to_string();
        let mut name = base.clone();
        let mut path = self.current.join(&name);
        let mut n = 2;
        while path.exists() { name = format!("{base} {n}"); path = self.current.join(&name); n += 1; }
        match fs::create_dir(&path) {
            Ok(()) => { self.status = format!("{}: {name}", self.text("Створено", "Created")); self.refresh(); }
            Err(e) => self.status = format!("{}: {e}", self.text("Не вдалося створити папку", "Could not create folder")),
        }
    }

    fn create_text_file(&mut self) {
        let base = self.text("Новий файл", "New file");
        let mut name = format!("{base}.txt");
        let mut path = self.current.join(&name);
        let mut n = 2;
        while path.exists() { name = format!("{base} {n}.txt"); path = self.current.join(&name); n += 1; }
        match fs::OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(_) => { self.status = format!("{}: {name}", self.text("Створено", "Created")); self.selected.clear(); self.selected.insert(path); self.refresh(); }
            Err(e) => self.status = format!("{}: {e}", self.text("Не вдалося створити файл", "Could not create file")),
        }
    }

    fn delete_selected(&mut self) {
        let mut paths: Vec<_> = self.selected.iter().cloned().collect();
        paths.sort();
        if paths.is_empty() { self.confirm_delete = false; return; }

        let mut removed = 0;
        let mut failures = Vec::new();
        for path in paths {
            if path.parent() != Some(self.current.as_path()) {
                failures.push(self.text("елемент поза поточною папкою", "item is outside the current folder").to_string());
                continue;
            }
            let result = match fs::symlink_metadata(&path) {
                Ok(metadata) if metadata.file_type().is_dir() => fs::remove_dir_all(&path),
                Ok(_) => fs::remove_file(&path),
                Err(e) => Err(e),
            };
            match result {
                Ok(()) => { self.selected.remove(&path); removed += 1; }
                Err(e) => failures.push(format!("{}: {e}", path.file_name().unwrap_or_default().to_string_lossy())),
            }
        }
        self.refresh();
        self.status = if failures.is_empty() { format!("{}: {removed}", self.text("Видалено елементів", "Items deleted")) }
            else { format!("{} {removed}; {}: {}", self.text("Видалено", "Deleted"), self.text("помилки", "errors"), failures.join("; ")) };
        self.confirm_delete = false;
    }

    fn open_selected(&mut self) {
        let mut paths: Vec<_> = self.selected.iter().cloned().collect();
        paths.sort();
        let mut failures = Vec::new();
        for path in &paths {
            if let Err(e) = opener::open(path) {
                failures.push(format!("{}: {e}", path.file_name().unwrap_or_default().to_string_lossy()));
            }
        }
        self.status = if failures.is_empty() {
            format!("{}: {}", self.text("Відкрито елементів", "Items opened"), paths.len())
        } else {
            format!("{}: {}", self.text("Не вдалося відкрити", "Could not open"), failures.join("; "))
        };
    }

    fn rename_selected(&mut self) {
        if self.selected.len() != 1 { return; }
        let old_path = self.selected.iter().next().expect("one selected item").clone();
        let name = self.rename_input.trim();
        if name.is_empty() || name == "." || name == ".." || name.chars().any(|c| matches!(c, '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*')) || name.ends_with([' ', '.']) {
            self.status = self.text("Вкажіть допустиму назву без символів шляху", "Enter a valid name without path separators").into();
            return;
        }
        if old_path.parent() != Some(self.current.as_path()) {
            self.status = self.text("Вибраний елемент не належить цій папці", "The selected item is not in this folder").into();
            self.rename_dialog = false;
            return;
        }
        let new_path = self.current.join(name);
        if new_path != old_path && new_path.exists() {
            self.status = self.text("Елемент із такою назвою вже існує", "An item with that name already exists").into();
            return;
        }
        match fs::rename(&old_path, &new_path) {
            Ok(()) => {
                self.status = format!("{} «{name}»", self.text("Перейменовано на", "Renamed to"));
                self.selected.clear();
                self.selected.insert(new_path);
                self.rename_dialog = false;
                self.refresh();
            }
            Err(e) => self.status = format!("{}: {e}", self.text("Не вдалося перейменувати", "Could not rename")),
        }
    }
}

impl eframe::App for VFile {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.language.is_none() {
            self.show_language_picker(ui);
            return;
        }

        egui::Panel::top("toolbar").frame(egui::Frame::new().fill(egui::Color32::from_rgb(24, 27, 34)).inner_margin(egui::Margin::symmetric(18, 14))).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Image::from_bytes("bytes://assets/logo.svg", include_bytes!("../assets/logo.svg"))
                    .fit_to_exact_size(egui::vec2(36.0, 32.0)));
                ui.heading(egui::RichText::new("VFile").color(egui::Color32::from_rgb(130, 170, 255)));
                ui.add_space(16.0);
                if ui.button("←").on_hover_text(self.text("На рівень вище", "Go to parent folder")).clicked() {
                    if let Some(parent) = self.current.parent() { self.current = parent.to_path_buf(); self.refresh(); }
                }
                if ui.button("⟳").on_hover_text(self.text("Оновити", "Refresh")).clicked() { self.refresh(); }
                let mut address = self.current.display().to_string();
                let address_hint = self.text("Шлях до папки", "Folder path");
                let response = ui.add(egui::TextEdit::singleline(&mut address).desired_width(ui.available_width() * 0.55).hint_text(address_hint));
                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { self.navigate(PathBuf::from(address)); }
                ui.add_space(8.0);
                let search_hint = self.text("⌕  Пошук у папці", "⌕  Search this folder");
                ui.add(egui::TextEdit::singleline(&mut self.search).desired_width(190.0).hint_text(search_hint));
            });
        });

        egui::Panel::left("sidebar").resizable(false).default_size(185.0).frame(egui::Frame::new().fill(egui::Color32::from_rgb(27, 30, 38)).inner_margin(egui::Margin::same(14))).show(ui, |ui| {
            ui.label(egui::RichText::new(self.text("РОЗТАШУВАННЯ", "LOCATIONS")).small().weak());
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
            for (label, icon, path) in [
                (self.text("Домівка", "Home"), "⌂", home),
                (self.text("Робочий стіл", "Desktop"), "▦", desktop),
                (self.text("Завантаження", "Downloads"), "↓", downloads),
                (self.text("Корінь диска", "Drive root"), "▣", root),
            ] {
                if ui.selectable_label(self.current == path, format!("{icon}  {label}")).clicked() { self.navigate(path); }
            }
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| { ui.label(egui::RichText::new("VFile • 1.2").small().weak()); });
        });

        egui::CentralPanel::default().frame(egui::Frame::new().fill(egui::Color32::from_rgb(19, 22, 28)).inner_margin(egui::Margin::symmetric(24, 20))).show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading(self.text("Файли", "Files"));
                ui.add_space(12.0);
                ui.label(egui::RichText::new(self.current.display().to_string()).weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_enabled(!self.selected.is_empty(), egui::Button::new(self.text("⌫ Видалити", "⌫ Delete"))).clicked() { self.confirm_delete = true; }
                    if ui.add_enabled(self.selected.len() == 1, egui::Button::new(self.text("Перейменувати", "Rename"))).clicked() {
                        self.rename_input = self.selected.iter().next().and_then(|p| p.file_name()).unwrap_or_default().to_string_lossy().to_string();
                        self.rename_dialog = true;
                    }
                    if ui.button(self.text("＋ .txt файл", "＋ .txt file")).clicked() { self.create_text_file(); }
                    if ui.button(self.text("＋ Нова папка", "＋ New folder")).clicked() { self.create_folder(); }
                    if self.selected.len() >= 2 {
                        let count = self.selected.len();
                        if ui.button(format!("{} {count}", self.text("Відкрити", "Open"))).clicked() { self.open_selected(); }
                    }
                });
            });
            ui.add_space(14.0);
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(self.text("ІМ’Я", "NAME")).small().weak());
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| { ui.label(egui::RichText::new(self.text("РОЗМІР", "SIZE")).small().weak()); });
            });
            ui.separator();
            let query = self.search.to_lowercase();
            let mut open_dir: Option<PathBuf> = None;
            let mut open_file: Option<PathBuf> = None;
            let mut selection_action: Option<(PathBuf, bool)> = None;
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.spacing_mut().interact_size.y = 30.0;
                ui.spacing_mut().button_padding = egui::vec2(8.0, 5.0);
                for entry in &self.entries {
                    let name = entry.path.file_name().unwrap_or_default().to_string_lossy().to_string();
                    if !name.to_lowercase().contains(&query) { continue; }
                    ui.horizontal(|ui| {
                        let icon = if entry.is_dir { "📁" } else { file_icon(&name) };
                        let response = ui.selectable_label(self.selected.contains(&entry.path), format!("{icon}  {name}"));
                        if response.clicked() {
                            selection_action = Some((entry.path.clone(), ui.input(|i| i.modifiers.ctrl)));
                        }
                        if response.double_clicked() {
                            if entry.is_dir { open_dir = Some(entry.path.clone()); } else { open_file = Some(entry.path.clone()); }
                        }
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(egui::RichText::new(if entry.is_dir { "—".into() } else { format_size(entry.size, self.language == Some(Language::English)) }).weak());
                        });
                    });
                    ui.separator();
                }
            });
            if let Some((path, ctrl_held)) = selection_action {
                if ctrl_held {
                    if !self.selected.remove(&path) { self.selected.insert(path); }
                } else {
                    self.selected.clear();
                    self.selected.insert(path);
                }
            }
            if let Some(path) = open_dir { self.navigate(path); }
            if let Some(path) = open_file { if let Err(e) = opener::open(&path) { self.status = format!("{}: {e}", self.text("Не вдалося відкрити файл", "Could not open file")); } }
            if !self.status.is_empty() { ui.colored_label(egui::Color32::LIGHT_RED, &self.status); }
        });

        if self.confirm_delete {
            let count = self.selected.len();
            let title = self.text("Підтвердити видалення", "Confirm deletion");
            egui::Window::new(title).collapsible(false).resizable(false).show(ui.ctx(), |ui| {
                if count == 1 {
                    let name = self.selected.iter().next().and_then(|p| p.file_name()).unwrap_or_default().to_string_lossy();
                    ui.label(format!("{} «{name}»? {}", self.text("Видалити", "Delete"), self.text("Вміст папки також буде видалено.", "The folder contents will also be deleted.")));
                } else {
                    ui.label(format!("{} {count} {}? {}", self.text("Видалити", "Delete"), self.text("вибраних елементів", "selected items"), self.text("Вміст папок також буде видалено.", "Folder contents will also be deleted.")));
                }
                ui.horizontal(|ui| {
                    if ui.button(self.text("Видалити", "Delete")).clicked() { self.delete_selected(); }
                    if ui.button(self.text("Скасувати", "Cancel")).clicked() { self.confirm_delete = false; }
                });
            });
        }

        if self.rename_dialog {
            let title = self.text("Перейменувати", "Rename");
            egui::Window::new(title).collapsible(false).resizable(false).show(ui.ctx(), |ui| {
                ui.label(self.text("Нова назва:", "New name:"));
                let input = ui.text_edit_singleline(&mut self.rename_input);
                let enter = input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
                ui.horizontal(|ui| {
                    let rename = ui.button(self.text("Зберегти", "Save")).clicked() || enter;
                    if rename { self.rename_selected(); }
                    if ui.button(self.text("Скасувати", "Cancel")).clicked() { self.rename_dialog = false; }
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

fn format_size(bytes: u64, english: bool) -> String {
    let units = if english { ["B", "KB", "MB", "GB"] } else { ["Б", "КБ", "МБ", "ГБ"] };
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < units.len() - 1 { size /= 1024.0; unit += 1; }
    format!("{size:.1} {}", units[unit])
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 720.0])
            .with_min_inner_size([760.0, 500.0])
            .with_icon(eframe::icon_data::from_png_bytes(include_bytes!("../assets/logo.png")).expect("valid VFile icon")),
        ..Default::default()
    };
    eframe::run_native("VFile", options, Box::new(|cc| Ok(Box::new(VFile::new(cc)))))
}
