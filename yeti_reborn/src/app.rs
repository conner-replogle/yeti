use eframe::{egui, epi, epaint::Color32};
use yeti::Yeti;
use std::thread::{self, JoinHandle};
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct YetiReborn {
    // Example stuff:+
    yeti: Yeti,
    label: String,
    value: f32,
    overlay:bool
}

impl Default for YetiReborn {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            yeti: Yeti::new(),
            overlay:false

        }
    }
}


impl epi::App for YetiReborn {
    fn clear_color(&self) -> eframe::epaint::Rgba{
        if self.overlay{
            return egui::Color32::from_rgba_unmultiplied(12, 12, 12, 180).into()
        }else{
            
            return egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0).into()
        }
        
    }
    fn name(&self) -> &str {
        "Yeti Reborn"
    }

    /// Called once before the first frame.
    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    // #[cfg(feature = "persistence")]
    // fn save(&mut self, storage: &mut dyn epi::Storage) {
    //     epi::set_value(storage, epi::APP_KEY, self);
    // }
    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        
        let Self { label, value, yeti, overlay } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        
        if *overlay{

            egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                // The top panel is often a good place for a menu bar:
                egui::menu::bar(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.quit();
                        }
                    });
                });
            });

            egui::SidePanel::left("side_panel").show(ctx, |ui| {
                ui.heading("Settings");
                if !yeti.running{
                    if ui.button("Start").clicked() {
                        yeti.start()
                    }
                }else{
                    if ui.button("Stop").clicked() {
                        yeti.stop()

                    }

                }
                

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(label);
                });

                ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    *value += 1.0;
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        ui.label("powered by Yeti");
                        
                    });
                });
            });

            egui::CentralPanel::default().show(ctx, |ui| {
                // The central panel the region left after adding TopPanel's and SidePanel's

                ui.heading("eframe template");
                ui.hyperlink("https://github.com/emilk/eframe_template");
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/master/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });

            if false {
                egui::Window::new("Window").show(ctx, |ui| {
                    ui.label("Windows can be moved by dragging them.");
                    ui.label("They are automatically sized based on contents.");
                    ui.label("You can turn on resizing and scrolling if you like.");
                    ui.label("You would normally chose either panels OR windows.");
                });
            }
        }
    }
}
