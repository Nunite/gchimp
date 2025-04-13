use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
};

use eframe::egui;

use gchimp::modules::blender_lightmap_baker_helper::{
    blender_lightmap_baker_helper, BLBHOptions, BLBH, BLBH_DEFAULT_UV_CLAMP_FACTOR,
};

use crate::{
    config::Config,
    gui::{constants::IMAGE_FORMATS, utils::preview_file_being_dropped, TabProgram},
    i18n::{Language, TextKey, get_text},
};

#[derive(Debug)]
pub struct BLBHGui {
    config: Config,
    smd_path: String,
    texture_path: String,
    options: BLBHOptions,
    clamp_value: String,
    check_clamp_value: bool,
    // origin: String,
    status: Arc<Mutex<String>>,
    current_language: Language,
}

impl BLBHGui {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            smd_path: Default::default(),
            texture_path: Default::default(),
            options: BLBHOptions::default(),
            clamp_value: BLBH_DEFAULT_UV_CLAMP_FACTOR.to_string(),
            check_clamp_value: false,
            // origin: "0 0 0".to_string(),
            status: Arc::new(Mutex::new("Idle".to_string())),
            current_language: Language::Chinese,
        }
    }

    pub fn run(&mut self) {
        let smd_path = self.smd_path.clone();
        let texture_path = self.texture_path.clone();
        let options = self.options.clone();

        let Config {
            studiomdl,
            #[cfg(target_os = "linux")]
            wineprefix,
            ..
        } = self.config.clone();

        let status = self.status.clone();

        // options.origin = parse_triplet(&self.origin)
        //     .map(|res| res.into())
        //     .unwrap_or(DVec3::ZERO);

        "Running".clone_into(&mut status.lock().unwrap());

        let _join_handle = thread::spawn(move || {
            let mut blbh = BLBH {
                smd_path: PathBuf::from(smd_path),
                texture_path: PathBuf::from(texture_path),
                options,
            };

            blbh.options.studiomdl = studiomdl;

            #[cfg(target_os = "linux")]
            {
                blbh.options.wineprefix = wineprefix.unwrap();
            }

            match blender_lightmap_baker_helper(&blbh) {
                Ok(_) => {
                    let mut status = status.lock().unwrap();

                    "Done".clone_into(&mut status);
                }
                Err(err) => {
                    let mut status = status.lock().unwrap();

                    err.to_string().clone_into(&mut status);
                }
            }
        });
    }
}

impl TabProgram for BLBHGui {
    fn tab_title(&self) -> eframe::egui::WidgetText {
        "BLBH".into()
    }

    fn tab_ui(&mut self, ui: &mut eframe::egui::Ui) -> egui_tiles::UiResponse {
        ui.separator();
        ui.hyperlink_to(
            "Youtube link on how to make use of this",
            "https://www.youtube.com/watch?v=OFKPLioaS3I",
        );

        ui.separator();
        egui::Grid::new("Input smd and texture grid")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label(get_text(TextKey::SMD, self.current_language));
                ui.add(
                    egui::TextEdit::singleline(&mut self.smd_path).hint_text("Choose .smd file"),
                );
                if ui.button(get_text(TextKey::Add, self.current_language)).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("SMD", &["smd"])
                        .pick_file()
                    {
                        if path.extension().is_some_and(|ext| ext == "smd") {
                            self.smd_path = path.display().to_string();
                        }
                    }
                }
                ui.end_row();

                ui.label(get_text(TextKey::Texture, self.current_language));
                ui.add(
                    egui::TextEdit::singleline(&mut self.texture_path)
                        .hint_text("Choose an image file"),
                );
                if ui.button(get_text(TextKey::Add, self.current_language)).clicked() {
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("Image", IMAGE_FORMATS)
                        .pick_file()
                    {
                        self.texture_path = path.display().to_string();
                    }
                }
                ui.end_row();
            });

        ui.separator();
        ui.label(get_text(TextKey::Options, self.current_language));

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.options.convert_texture, get_text(TextKey::ConvertTexture, self.current_language))
                .on_hover_text(get_text(TextKey::ConvertTextureHint, self.current_language));
            ui.checkbox(&mut self.options.convert_smd, get_text(TextKey::ConvertSMD, self.current_language))
                .on_hover_text(get_text(TextKey::ConvertSMDHint, self.current_language));
            ui.checkbox(&mut self.options.compile_model, get_text(TextKey::CompileMDL, self.current_language))
                .on_hover_text(get_text(TextKey::CompileMDLHint, self.current_language));
            ui.checkbox(&mut self.options.flat_shade, get_text(TextKey::Flatshade, self.current_language))
                .on_hover_text(get_text(TextKey::FlatshadeHint, self.current_language));
        });

        ui.horizontal(|ui| {
            ui.label("UV Clamp");
            // only check value if lost focus
            let text_editor = egui::TextEdit::singleline(&mut self.clamp_value).desired_width(80.);

            let text_editor_ui = ui.add(text_editor).on_hover_text(get_text(TextKey::UVClampHint, self.current_language));

            if text_editor_ui.has_focus() {
                self.check_clamp_value = true;
            }

            if text_editor_ui.lost_focus() && self.check_clamp_value {
                let shrink_value = self
                    .clamp_value
                    .parse::<f32>()
                    .unwrap_or(BLBH_DEFAULT_UV_CLAMP_FACTOR);
                self.clamp_value = shrink_value.to_string();
                self.options.uv_clamp_factor = shrink_value;
                self.check_clamp_value = false;
            }

            if ui.button("Default").clicked() {
                self.clamp_value = BLBH_DEFAULT_UV_CLAMP_FACTOR.to_string()
            }

            // origin
            // ui.label("Origin");

            // let text_editor = egui::TextEdit::singleline(&mut self.origin).desired_width(96.)   ;
            // ui.add(text_editor)
            //     .on_hover_text("Origin of the model. Space separated.");
        });

        ui.separator();

        if ui.button(get_text(TextKey::Run, self.current_language)).clicked() {
            self.run();
        }

        let binding = self.status.lock().unwrap();
        let mut status_text = binding.as_str();
        ui.text_edit_multiline(&mut status_text);

        let ctx = ui.ctx();
        preview_file_being_dropped(ctx);

        // Collect dropped files:
        ctx.input(|i| {
            for item in i.raw.dropped_files.clone() {
                if let Some(path) = item.path {
                    if path.is_file() {
                        if path.extension().is_some_and(|ext| ext == "smd") {
                            self.smd_path = path.to_str().unwrap().to_string();
                        } else if path.extension().is_some_and(|ext| ext == "wav") {
                            self.texture_path = path.to_str().unwrap().to_string();
                        }
                    }
                }
            }
        });

        // runs in continuous mode
        ctx.request_repaint();

        // Make it non drag-able
        egui_tiles::UiResponse::None
    }
}
