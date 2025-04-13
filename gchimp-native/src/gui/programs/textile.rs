use std::{
    path::{Path, PathBuf},
    thread,
};

use eframe::egui;
use egui_extras::{Column, TableBuilder};

use gchimp::modules::textile::{TexTileBuilder, TexTileOptions, TexTileSync};

use crate::{
    gui::{utils::preview_file_being_dropped, TabProgram},
    i18n::{Language, TextKey, get_text},
};

pub struct TexTileGui {
    items: Vec<PathBuf>,
    options: TexTileOptions,
    extensions: String,
    sync: TexTileSync,
    current_language: Language,
}

impl Default for TexTileGui {
    fn default() -> Self {
        let options = TexTileOptions::default();
        let extensions = options.extensions.join(" ");

        Self {
            items: vec![],
            options,
            extensions,
            sync: TexTileSync::default(),
            current_language: Language::Chinese,
        }
    }
}

impl TexTileGui {
    fn add_item(&mut self, path: &Path) {
        self.update_extensions();

        if self.options.check_item(path).is_ok() {
            self.items.push(path.to_path_buf())
        }
    }

    fn update_extensions(&mut self) {
        let extensions = self
            .extensions
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        self.options.extensions = extensions;
    }

    fn run(&mut self) {
        self.update_extensions();

        let TexTileOptions {
            extensions: _,
            is_tiling,
            tiling_scalar,
            is_transparent,
            transparent_threshold,
            change_name,
        } = self.options;

        let items = self.items.clone();
        let sync = self.sync.clone();
        let extensions = self.options.extensions.clone();

        let _ = thread::spawn(move || {
            let mut binding = TexTileBuilder::new(items);

            let textile = binding
                .extension(&extensions)
                .change_name(change_name)
                .tiling(is_tiling)
                .tiling_scalar(tiling_scalar)
                .transparent(is_transparent)
                .transparent_threshold(transparent_threshold)
                .sync(sync.clone());

            *sync.done().lock().unwrap() = false;

            let res = textile.work();

            *sync.done().lock().unwrap() = true;

            res
        });
    }
}

impl TabProgram for TexTileGui {
    fn tab_title(&self) -> eframe::egui::WidgetText {
        "TexTile".into()
    }

    fn tab_ui(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse {
        ui.separator();
        ui.label(get_text(TextKey::Options, self.current_language));

        ui.horizontal(|ui| {
            ui.label(get_text(TextKey::ImageExtensions, self.current_language));
            ui.text_edit_singleline(&mut self.extensions).on_hover_text(
                "\
Converts only textures with specified file extension(s)
Space seperated",
            );
        });

        egui::Grid::new("TexTile option grid")
            .num_columns(6)
            .show(ui, |ui| {
                ui.checkbox(&mut self.options.is_tiling, get_text(TextKey::Tiling, self.current_language));
                ui.add_enabled(
                    self.options.is_tiling,
                    egui::DragValue::new(&mut self.options.tiling_scalar).range(0.0..=100.0),
                )
                .on_hover_text("The dimensions of a texture will multiply by this number.");

                ui.checkbox(&mut self.options.is_transparent, get_text(TextKey::Transparent, self.current_language));
                ui.add_enabled(
                    self.options.is_transparent,
                    egui::DragValue::new(&mut self.options.transparent_threshold)
                        .range(0.0..=1.0)
                        .speed(0.01),
                )
                .on_hover_text(
                    "\
The threshold to decide whether a texture is transparent. \n
If the dominant color of an image exceeds this threshold, 
it will be chosen as transparent mask.",
                );

                ui.checkbox(&mut self.options.change_name, get_text(TextKey::ChangeFileName, self.current_language))
                    .on_hover_text(
                        "\
Prepend \"{\" if transparent
Append \"_<scalar>\" if tiling",
                    );
            });

        ui.separator();
        ui.horizontal(|ui| {
            let is_done = *self.sync.done().lock().unwrap();
            ui.add_enabled_ui(is_done, |ui| {
                if ui.button(get_text(TextKey::Run, self.current_language)).clicked() {
                    self.run();
                }
            });
            ui.add_enabled_ui(!is_done, |ui| if ui.button(get_text(TextKey::Cancel, self.current_language)).clicked() {});

            let readonly_buffer = self.sync.status().lock().unwrap();
            ui.text_edit_singleline(&mut readonly_buffer.as_str())
        });

        ui.separator();

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        ui.horizontal(|ui| {
            if ui.button(get_text(TextKey::AddFiles, self.current_language)).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_files() {
                    for path in paths {
                        self.add_item(path.as_path());
                    }
                }
            }

            if ui.button(get_text(TextKey::AddFolders, self.current_language)).clicked() {
                if let Some(paths) = rfd::FileDialog::new().pick_folders() {
                    for path in paths {
                        self.add_item(path.as_path());
                    }
                }
            }

            if ui.button(get_text(TextKey::Clear, self.current_language)).clicked() {
                self.items.clear();
            }
        });

        let mut remove_index: Option<usize> = None;

        ui.label(format!("{} ({}):", get_text(TextKey::ListOfItems, self.current_language), self.items.len()));

        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(false)
            .column(Column::remainder());

        table.body(|body| {
            body.rows(text_height, self.items.len(), |mut row| {
                let row_index = row.index();

                row.col(|ui| {
                    let curr_item = &self.items[row_index];
                    let display_text = curr_item.display().to_string();

                    let label = ui
                        .selectable_label(
                            false,
                            if curr_item.is_dir() {
                                format!("{} (folder)", display_text)
                            } else {
                                display_text
                            },
                        )
                        .on_hover_text("Right click to remove");

                    if label.clicked_by(egui::PointerButton::Secondary) {
                        remove_index = Some(row_index);
                    }
                });
            });
        });

        let ctx = ui.ctx();

        if let Some(remove_index) = remove_index {
            self.items.remove(remove_index);
        }

        preview_file_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            for item in &i.raw.dropped_files {
                if let Some(path) = &item.path {
                    self.add_item(path);
                }
            }
        });

        // Make it non drag-able
        egui_tiles::UiResponse::None
    }
}
