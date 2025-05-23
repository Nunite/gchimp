use std::{
    path::PathBuf,
    thread::{self, JoinHandle},
};

use eframe::egui::{self, ScrollArea};

use gchimp::modules::s2g::{S2GOptions, S2GSteps, S2GSync, S2G};

use crate::{
    config::Config,
    gui::{
        constants::{PROGRAM_HEIGHT, PROGRAM_WIDTH},
        utils::preview_file_being_dropped,
        TabProgram,
    },
    i18n::{Language, TextKey, get_text},
};

struct DragAndDrop {
    file_path: String,
    use_file: bool,
    folder_path: String,
    use_folder: bool,
}

impl Default for DragAndDrop {
    fn default() -> Self {
        // use_file and use_path are both true so the users can choose either at the beginning.
        Self {
            file_path: Default::default(),
            use_file: true,
            folder_path: Default::default(),
            use_folder: true,
        }
    }
}

pub struct S2GGui {
    app_config: Config,
    s2g_sync: S2GSync,
    drag_and_drop: DragAndDrop,
    steps: S2GSteps,
    options: S2GOptions,
    is_idle: bool,
    current_language: Language,
}

impl S2GGui {
    // runs in a different thread to avoid blocking
    fn run(&self) -> eyre::Result<JoinHandle<eyre::Result<Vec<PathBuf>>>> {
        let path = if self.drag_and_drop.use_file {
            self.drag_and_drop.file_path.clone()
        } else {
            self.drag_and_drop.folder_path.clone()
        };

        let steps = self.steps.clone();
        let options = self.options.clone();

        let Config {
            studiomdl,
            crowbar,
            #[cfg(target_os = "linux")]
            wineprefix,
            ..
        } = self.app_config.clone();

        let sync = self.s2g_sync.clone();

        let handle = thread::spawn(move || {
            *sync.is_done().lock().unwrap() = false;

            // TODO fix, this is not respecting config.toml
            let mut s2g = S2G::new(path.as_str());

            let S2GSteps {
                decompile,
                vtf,
                bmp,
                smd_and_qc,
                compile,
            } = steps;

            let S2GOptions {
                force,
                add_suffix,
                ignore_converted,
                flatshade,
                ..
            } = options;

            s2g.studiomdl(PathBuf::from(studiomdl).as_path())
                .crowbar(PathBuf::from(crowbar).as_path());

            #[cfg(target_os = "linux")]
            s2g.wineprefix(wineprefix.as_ref().unwrap());

            s2g.decompile(decompile)
                .vtf(vtf)
                .bmp(bmp)
                .smd_and_qc(smd_and_qc)
                .compile(compile)
                .sync(sync.clone())
                .force(force)
                .add_suffix(add_suffix)
                .ignore_converted(ignore_converted)
                .flatshade(flatshade);

            let res = s2g.work();

            *sync.is_done().lock().unwrap() = true;

            res
        });

        Ok(handle)
    }

    pub fn new(app_config: Config) -> Self {
        Self {
            app_config,
            s2g_sync: S2GSync::default(),
            drag_and_drop: DragAndDrop::default(),
            steps: S2GSteps::default(),
            options: S2GOptions::default(),
            is_idle: true,
            current_language: Language::Chinese,
        }
    }
}

impl TabProgram for S2GGui {
    fn tab_title(&self) -> eframe::egui::WidgetText {
        "S2G".into()
    }

    fn tab_ui(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse {
        ui.separator();

        ui.add_enabled_ui(true, |ui| {
            egui::Grid::new("S2G Layout").num_columns(2).show(ui, |ui| {
                ui.label(get_text(TextKey::File, self.current_language));
                ui.add_enabled_ui(self.drag_and_drop.use_file, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.drag_and_drop.file_path)
                            .hint_text("Choose .mdl file"),
                    );
                });
                if ui.button("+").clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Model files", &["mdl"])
                        .pick_file()
                    {
                        self.drag_and_drop.file_path = path.display().to_string();
                        self.drag_and_drop.use_file = true;
                        self.drag_and_drop.use_folder = false;
                    }
                }
                ui.end_row();

                ui.label(get_text(TextKey::Folder, self.current_language));
                ui.add_enabled_ui(self.drag_and_drop.use_folder, |ui| {
                    ui.add(
                        egui::TextEdit::singleline(&mut self.drag_and_drop.folder_path)
                            .hint_text("Choose folder containing .mdl"),
                    );
                });
                if ui.button("+").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.drag_and_drop.folder_path = path.display().to_string();
                        self.drag_and_drop.use_folder = true;
                        self.drag_and_drop.use_file = false;
                    }
                }
            })
        });

        // if compile is ticked then always do the smd and qc step
        if self.steps.compile {
            self.steps.smd_and_qc = true;
        }

        ui.separator();
        ui.label(get_text(TextKey::Steps, self.current_language));
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.steps.decompile, get_text(TextKey::Decompile, self.current_language));
            ui.checkbox(&mut self.steps.vtf, get_text(TextKey::VTF, self.current_language))
                .on_hover_text(get_text(TextKey::VTFHint, self.current_language));
            ui.checkbox(&mut self.steps.bmp, get_text(TextKey::BMP, self.current_language))
                .on_hover_text(get_text(TextKey::BMPHint, self.current_language));
            ui.checkbox(&mut self.steps.smd_and_qc, get_text(TextKey::SmdQc, self.current_language))
                .on_hover_text(get_text(TextKey::SmdQcHint, self.current_language));
            ui.checkbox(&mut self.steps.compile, get_text(TextKey::GoldSrcCompile, self.current_language))
                .on_hover_text(get_text(TextKey::GoldSrcCompileHint, self.current_language));
        });

        ui.separator();
        ui.label(get_text(TextKey::Options, self.current_language));
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.options.force, get_text(TextKey::Force, self.current_language))
                .on_hover_text(get_text(TextKey::ForceHint, self.current_language));
            ui.checkbox(&mut self.options.add_suffix, get_text(TextKey::AddSuffix, self.current_language))
                .on_hover_text(get_text(TextKey::AddSuffixHint, self.current_language));
            ui.checkbox(&mut self.options.ignore_converted, get_text(TextKey::IgnoreConverted, self.current_language))
                .on_hover_text(get_text(TextKey::IgnoreConvertedHint, self.current_language));
            ui.checkbox(&mut self.options.flatshade, get_text(TextKey::Flatshade, self.current_language))
                .on_hover_text(get_text(TextKey::FlatshadeHint, self.current_language));
        });

        let is_done = *self.s2g_sync.is_done().lock().unwrap();

        ui.separator();
        ui.horizontal(|ui| {
            ui.add_enabled_ui(is_done, |ui| {
                if ui.button(get_text(TextKey::Run, self.current_language)).clicked() {
                    self.is_idle = false;
                    let _ = self.run();
                }
            });
            ui.add_enabled_ui(!is_done, |ui| {
                if ui.button(get_text(TextKey::Cancel, self.current_language)).clicked() {
                    self.is_idle = true;
                }
            });
            if ui
                .button(get_text(TextKey::Clear, self.current_language))
                .on_hover_text("Click to clear output text")
                .clicked()
            {
                *self.s2g_sync.stdout().lock().unwrap() = "".to_string();
            }
        });

        ui.separator();

        let binding = self.s2g_sync.stdout().lock().unwrap();
        let mut readonly_buffer = binding.as_str();

        ScrollArea::vertical().show(ui, |ui| {
            ui.add_sized(
                egui::vec2(PROGRAM_WIDTH, PROGRAM_HEIGHT / 3.),
                egui::TextEdit::multiline(&mut readonly_buffer),
            );
        });

        let ctx = ui.ctx();
        preview_file_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            if i.raw.dropped_files.len() == 1 {
                let item = i.raw.dropped_files[0].clone();
                if let Some(path) = item.path {
                    if path.is_file() {
                        self.drag_and_drop.file_path = path.display().to_string();
                        self.drag_and_drop.use_file = true;
                        self.drag_and_drop.use_folder = false;
                    } else if path.is_dir() {
                        self.drag_and_drop.folder_path = path.display().to_string();
                        self.drag_and_drop.use_folder = true;
                        self.drag_and_drop.use_file = false;
                    }
                }
            }
        });

        // Force continuous mode
        ctx.request_repaint();

        // Make it non drag-able
        egui_tiles::UiResponse::None
    }
}
