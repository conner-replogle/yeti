use std::{sync::{Arc, RwLock}, ops::Deref, path::PathBuf, ptr};

use egui_glium::egui_winit::egui::{Context, Frame, Color32, Ui, RichText, mutex::Mutex, Vec2, menu, Layout};
use glium::{Display, glutin};
use egui_glium::egui_winit::egui;
use log::debug;
use windows::{Win32::{UI::{Input::KeyboardAndMouse::{SetFocus, SetCapture}, WindowsAndMessaging::{FindWindowA, FindWindowW, GetWindowThreadProcessId}}, System::Threading::{AttachThreadInput, GetCurrentThreadId}, Foundation::GetLastError}, core::{PCSTR, PCWSTR, HSTRING}};
use yeti_lib::{app_config::{AppConfig, KeyBindType}, hack_config::YetiHackConfig, Saveable};

use crate::{yeti::Yeti, widgets::KeyBindSelector};
pub enum GuiMode{
    InGame,
    Settings,
    GuiEditing
}
pub enum SettingsTab{
    Glow,
    Bhops,
    Aim,
    Settings,

}

pub fn gui_settings(display: &Display,egui_ctx: &Context,app_config:&mut AppConfig,hack_config: &mut YetiHackConfig,tab: &mut SettingsTab,hittest: &mut bool,yeti: &mut Yeti){
    egui::Window::new("Settings").title_bar(true).collapsible(false).frame(Frame{
        fill: Color32::from_black_alpha(if *hittest{250}else{150}),
        ..Default::default()
        
    }).show(egui_ctx, |ui| {
        let mut changed = false;
        

        
        if !egui_ctx.is_pointer_over_area() && *hittest && !egui_ctx.is_using_pointer() {
            let clicked ={ egui_ctx.input().pointer.any_click()};
            if clicked {
                println!("Back too CS");
                *hittest = !*hittest;
                display.gl_window().window().set_cursor_hittest(false).unwrap();
                display.gl_window().window().set_always_on_top(true);
                unsafe{
                    let handle = FindWindowW(PCWSTR::null(),PCWSTR::from(&HSTRING::from("Counter-Strike: Global Offensive - Direct3D 9")));
                    println!("Window HWND {:?}",handle);
                    let remote_thread = GetWindowThreadProcessId(handle,ptr::null_mut());
                    AttachThreadInput(GetCurrentThreadId(), remote_thread, true);
                    if SetCapture(handle).0 == 0{
                        println!("SetCapture failed w error {:?}",GetLastError());
                    }
                    if SetFocus(handle).0 == 0{
                        println!("SetFocus failed w error {:?}",GetLastError());
                    }
                    AttachThreadInput(GetCurrentThreadId(), remote_thread, false);
                    
                    
                }
            }
        }
        
        

        
        menu::bar(ui, |ui| {
            if ui.button(RichText::new("Glow").color(if let SettingsTab::Glow = tab{Color32::WHITE}else{Color32::GRAY})).clicked() {
                *tab = SettingsTab::Glow;
            }
            if ui.button(RichText::new("Bhops").color(if let SettingsTab::Bhops = tab{Color32::WHITE}else{Color32::GRAY})).clicked() {
                *tab = SettingsTab::Bhops;
            }
            if ui.button(RichText::new("Settings").color(if let SettingsTab::Settings = tab{Color32::WHITE}else{Color32::GRAY})).clicked() {
                *tab = SettingsTab::Settings;
            }
            ui.with_layout(Layout::right_to_left(), |ui|{
                if ui.button(RichText::new(format!("Save {}",app_config.loaded_config_name)).color(if let SettingsTab::Settings = tab{Color32::WHITE}else{Color32::GRAY})).clicked() {
                    *tab = SettingsTab::Settings;
                    app_config.hack_config_path.set_file_name(format!("{}.{}",app_config.loaded_config_name.as_mut_str(),"json"));
                    hack_config.save(&app_config.hack_config_path);
                    debug!("Hack Config {} saved",app_config.loaded_config_name);
                }
            })
            
        });
        ui.separator();
        match tab{
            SettingsTab::Glow => {
                ui.add(KeyBindSelector(&mut app_config.key_binds.glow));

                if ui.checkbox(&mut hack_config.toggle.glow, "enabled").changed(){
                    changed = true;
                }
                ui.collapsing("Enemy Team", |ui|{
                    let glow_set = &mut hack_config.glow.enemy_glow;
                    if ui.checkbox(&mut glow_set.enabled, "enabled").changed(){
                        changed = true;
                    };
                    ui.label("Glow Color");
                    if ui.color_edit_button_rgba_unmultiplied(&mut glow_set.glow_rgba).changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.render_when_occluded, "Render When Occuluded").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.render_when_unoccluded, "Render When unOcculuded").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.full_bloom, "Full Bloom").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.bomb_defusal_affect, "Bomb Defusal").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.health_affect, "Health Affect").changed(){
                        changed = true;
                    }
    
                });
                ui.collapsing("My Team", |ui|{
                    let glow_set = &mut hack_config.glow.team_glow;
                    if ui.checkbox(&mut glow_set.enabled, "enabled").changed(){
                        changed = true;
                    };
                    ui.label("Glow Color");
                    if ui.color_edit_button_rgba_unmultiplied(&mut glow_set.glow_rgba).changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.render_when_occluded, "Render When Occuluded").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.render_when_unoccluded, "Render When unOcculuded").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.full_bloom, "Full Bloom").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.bomb_defusal_affect, "Bomb Defusal").changed(){
                        changed = true;
                    }
                    if ui.checkbox(&mut glow_set.health_affect, "Health Affect").changed(){
                        changed = true;
                    }

                });
            

            },
            // SettingsTab::Aim => todo!(),
            SettingsTab::Bhops => {
                if ui.checkbox(&mut hack_config.toggle.bhop, "enabled").changed(){
                    changed = true;
                }
            },
            SettingsTab::Settings => {
                ui.horizontal(|ui| {
                    if ui.button("Rescan Signatures").clicked(){
                        yeti.update_signatures(Yeti::signatures_scan(app_config.signatures_config_path.to_str().unwrap(),app_config.signatures_path.to_str().unwrap()));
                    }
                    ui.label(format!("Last scan was {} at {}",yeti.signatures.as_ref().unwrap().timestamp.naive_local().date(),yeti.signatures.as_ref().unwrap().timestamp.naive_local().time().format("%H:%M:%S")));

                });
                
                if !yeti.injected{
                    if ui.button("Inject Yeti").clicked(){
                        yeti.start(&PathBuf::from("target\\debug\\yeti_dll.dll"),&app_config.log_config_path).unwrap();
                        yeti.update_config(hack_config);
                    }
                }else{
                    if ui.button("Uninject Yeti").clicked(){
                        yeti.stop();
                    }
                }

                ui.add(egui::TextEdit::singleline(&mut app_config.loaded_config_name));



            }
            SettingsTab::Aim => {

            },
        }
        if changed{
            yeti.update_config(hack_config);
        }
        
    });

}
fn color_from_bool(a: bool) -> Color32{
    if a{
        Color32::GREEN
    }else {
        Color32::DARK_RED
    }
}

pub fn gui_in_game(display: &Display,egui_ctx: &Context,config: &mut YetiHackConfig,yeti: &mut Yeti,hittest:&mut bool){
    egui::Window::new("").title_bar(false).collapsible(false).frame(Frame{
        fill: Color32::from_black_alpha(if *hittest{250}else{100}),
        ..Default::default()
        
    }).show(egui_ctx, |ui| {
        if !egui_ctx.is_pointer_over_area() && *hittest && !egui_ctx.is_using_pointer() {
            let clicked ={ egui_ctx.input().pointer.any_click()};
            if clicked {
                println!("Back too CS");
                *hittest = !*hittest;
                display.gl_window().window().set_cursor_hittest(false).unwrap();
                display.gl_window().window().set_always_on_top(true);
                unsafe{
                    let handle = FindWindowW(PCWSTR::null(),PCWSTR::from(&HSTRING::from("Counter-Strike: Global Offensive - Direct3D 9")));
                    println!("Window HWND {:?}",handle);
                    let remote_thread = GetWindowThreadProcessId(handle,ptr::null_mut());
                    AttachThreadInput(GetCurrentThreadId(), remote_thread, true);
                    if SetCapture(handle).0 == 0{
                        println!("SetCapture failed w error {:?}",GetLastError());
                    }
                    if SetFocus(handle).0 == 0{
                        println!("SetFocus failed w error {:?}",GetLastError());
                    }
                    AttachThreadInput(GetCurrentThreadId(), remote_thread, false);
                    
                    
                }
            }
        }
        let mut changed = false;
        if ui.button(RichText::new("GLOW").color(color_from_bool(config.toggle.glow))).clicked(){
            config.toggle.glow = !config.toggle.glow;
            changed = true;
        }
        if ui.button(RichText::new("AIM").color(color_from_bool(config.toggle.aim))).clicked(){
            config.toggle.aim = !config.toggle.aim;
            changed = true;
        }

        if ui.button(RichText::new("BHOP").color(color_from_bool(config.toggle.bhop))).clicked(){
            config.toggle.bhop = !config.toggle.bhop;
            changed = true;
        }
        if changed{
            yeti.update_config(config);
        }
        
    });
    
}