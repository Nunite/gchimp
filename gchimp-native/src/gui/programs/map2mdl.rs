use std::{path::PathBuf, thread};

use eframe::egui::{self, ScrollArea};

use gchimp::modules::map2mdl::{entity::MAP2MDL_ENTITY_NAME, Map2Mdl, Map2MdlOptions, Map2MdlSync};

use crate::{
    config::Config,
    gui::{
        constants::{PROGRAM_HEIGHT, PROGRAM_WIDTH},
        utils::preview_file_being_dropped,
        TabProgram,
    },
    i18n::{Language, TextKey, get_text},
};

pub struct Map2MdlGui {
    app_config: Config,
    map: String,
    entity: String,
    use_entity: bool,
    options: Map2MdlOptions,
    sync: Map2MdlSync,
    current_language: Language,
    extensions: String,
}

impl Map2MdlGui {
    pub fn new(app_config: Config) -> Self {
        Self {
            app_config,
            map: Default::default(),
            entity: Default::default(),
            use_entity: false,
            options: Map2MdlOptions::default(),
            sync: Map2MdlSync::default(),
            current_language: Language::Chinese,
            extensions: Default::default(),
        }
    }

    fn run(&mut self) {
        *self.sync.stdout().lock().unwrap() = "".to_string();

        let Config {
            studiomdl,
            crowbar: _,
            #[cfg(target_os = "linux")]
            wineprefix,
            ..
        } = self.app_config.clone();

        let Map2MdlOptions {
            auto_pickup_wad,
            export_texture,
            move_to_origin,
            marked_entity,
            flatshade,
            uppercase,
            reverse_normal,
            ..
        } = self.options;
        let entity = self.entity.clone();
        let map = self.map.clone();
        let use_entity = self.use_entity;

        let sync = self.sync.clone();

        thread::spawn(move || {
            let mut binding = Map2Mdl::default();
            binding
                .auto_pickup_wad(auto_pickup_wad)
                .move_to_origin(move_to_origin)
                .export_texture(export_texture)
                .studiomdl(PathBuf::from(&studiomdl).as_path())
                .marked_entity(marked_entity)
                .flatshade(flatshade)
                .uppercase(uppercase)
                .reverse_normal(reverse_normal)
                .sync(sync.clone());

            if use_entity {
                binding.entity(&entity);
            } else {
                binding.map(&map);
            };

            #[cfg(target_os = "linux")]
            binding.wineprefix(wineprefix.as_ref().unwrap());

            if let Err(err) = binding.work() {
                let mut lock = sync.stdout().lock().unwrap();
                *lock += "\n";
                *lock += err.to_string().as_str();
            } else {
                let mut ok_text = "OK".to_string();

                if use_entity {
                    ok_text += &("\n".to_owned()
                        + "Model is saved as map2mdl.mdl at "
                        + studiomdl.replace("studiomdl.exe", "").as_str());
                }

                *sync.stdout().lock().unwrap() += ok_text.as_str();
            }
        });
    }
}

impl TabProgram for Map2MdlGui {
    fn tab_title(&self) -> eframe::egui::WidgetText {
        "Map2Mdl".into()
    }

    fn tab_ui(&mut self, ui: &mut egui::Ui) -> egui_tiles::UiResponse {
        ui.separator();

        ui.add_enabled_ui(true, |ui| {
            egui::Grid::new("map2mdl grid")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label(get_text(TextKey::Map, self.current_language));
                    ui.add_enabled_ui(!self.use_entity, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.map).hint_text("Choose .map file"),
                        );
                    });
                    if ui.button(get_text(TextKey::Add, self.current_language)).clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("MAP", &["map"])
                            .pick_file()
                        {
                            if path.extension().is_some_and(|ext| ext == "map") {
                                self.map = path.display().to_string();
                                self.use_entity = false;
                            }
                        }
                    }

                    ui.end_row();
                    ui.checkbox(&mut self.use_entity, get_text(TextKey::Entity, self.current_language));
                    ui.add_enabled_ui(self.use_entity, |ui| {
                        ui.add(
                            egui::TextEdit::singleline(&mut self.entity)
                                .hint_text("Worldbrush entity copied from TrechBroom"),
                        );
                    });
                    if ui.button(get_text(TextKey::Clear, self.current_language)).clicked() {
                        self.entity.clear();
                    }
                })
        });
        ui.separator();
        ui.label(get_text(TextKey::Options, self.current_language));

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.options.auto_pickup_wad, get_text(TextKey::AutoPickupWads, self.current_language))
                .on_hover_text(get_text(TextKey::AutoPickupWadsHint, self.current_language));
            ui.checkbox(&mut self.options.export_texture, get_text(TextKey::ExportTextures, self.current_language))
                .on_hover_text(get_text(TextKey::ExportTexturesHint, self.current_language));
            ui.checkbox(&mut self.options.uppercase, get_text(TextKey::UppercaseTexture, self.current_language))
                .on_hover_text(get_text(TextKey::UppercaseTextureHint, self.current_language));
        });

        ui.horizontal(|ui| {
            ui.checkbox(
                &mut self.options.marked_entity,
                get_text(TextKey::OnlyConvertMarked, self.current_language),
            )
            .on_hover_text(get_text(TextKey::OnlyConvertMarkedHint, self.current_language));
            ui.checkbox(&mut self.options.move_to_origin, get_text(TextKey::CenterModel, self.current_language))
                .on_hover_text(get_text(TextKey::CenterModelHint, self.current_language));
            ui.checkbox(&mut self.options.flatshade, get_text(TextKey::Flatshade, self.current_language))
                .on_hover_text(get_text(TextKey::FlatshadeModelHint, self.current_language));
        });

        ui.horizontal(|ui| {
            ui.checkbox(&mut self.options.reverse_normal, get_text(TextKey::ReverseNormal, self.current_language))
                .on_hover_text(get_text(TextKey::ReverseNormalHint, self.current_language));
        });

        ui.separator();

        // ui.add_enabled_ui(true, |ui| {
        //     ui.label(get_text(TextKey::ImageExtensions, self.current_language));
        //     ui.text_edit_singleline(&mut self.extensions)
        //         .on_hover_text(get_text(TextKey::FileExtensionsHint, self.current_language));
        // });

        if ui.button(get_text(TextKey::Run, self.current_language)).clicked() {
            self.run();
        }

        ui.separator();

        let binding = self.sync.stdout().lock().unwrap();
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
                    if path.is_file() && path.extension().is_some_and(|ext| ext == "map") {
                        self.map = path.to_str().unwrap().to_string();
                        self.use_entity = false;
                    }
                }
            }
        });

        // Make it non drag-able
        egui_tiles::UiResponse::None
    }
}
