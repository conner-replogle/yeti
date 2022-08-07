use egui::widgets::Widget;
use egui_glium::egui_winit::egui::{self, Vec2, Ui};
use strum::IntoEnumIterator;
use yeti_lib::app_config::KeyBindType; // 0.17.1

use crate::glutin::event::VirtualKeyCode;
pub struct KeyBindSelector<'a>(pub &'a mut Option<KeyBindType>);

impl Widget for KeyBindSelector<'_>{
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let mut selected_key = self.0;
        ui.collapsing(format!("KeyBind: {:?}",selected_key.as_ref().unwrap_or_else(||{&KeyBindType::None})), |ui| {
            let a = ui.allocate_ui(Vec2::new(150.0,100.0),|ui|{
                let mut is_toggle = match selected_key{
                    Some(a) => {match a{
                        KeyBindType::Toggle(_) => {true},
                        KeyBindType::Hold(_) => {false},
                        KeyBindType::None => {false},
                    }},
                    None => {false},
                };
                if selected_key.is_some() && ui.checkbox(&mut is_toggle, "Toggle or Hold").changed(){
                    *selected_key = match selected_key{
                        Some(a) => {match a{
                            KeyBindType::Toggle(a) => {Some(KeyBindType::Hold(*a))},
                            KeyBindType::Hold(a) => {Some(KeyBindType::Toggle(*a))},
                            KeyBindType::None => {None},
                        }},
                        None => {None},
                    };
                };
                egui::ScrollArea::vertical().show(ui,|ui|{
                    for key in VirtualKeyCode::iter(){
                        if is_toggle{
                            ui.selectable_value(selected_key,Some(KeyBindType::Toggle(key)),key.to_string());
                        }else{
                            ui.selectable_value(selected_key,Some(KeyBindType::Hold(key)),key.to_string());
                        }
                    }
                })
            });
    
            a.response
        }).header_response
       
    }
}