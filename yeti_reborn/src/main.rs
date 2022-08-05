use std::{fs::{File, OpenOptions}, path::PathBuf, io::{Write, Read}, ptr};

use dll_needle::target_process::TargetProcess;
use log::{LevelFilter, debug};
use log4rs::{append::{console::{ConsoleAppender, Target}, file::FileAppender}, encode::pattern::PatternEncoder, config::{Appender, Root}, filter::threshold::ThresholdFilter};
use windows::{Win32::{UI::{Input::KeyboardAndMouse::{SetFocus, SetCapture}, WindowsAndMessaging::{FindWindowA, GetWindowThreadProcessId, FindWindowW}}, Foundation::GetLastError, System::Threading::{AttachThreadInput, GetCurrentThreadId}}, core::{PCSTR, PCWSTR, HSTRING}};
use yeti::Yeti;
use yeti_lib::{hack_config::YetiHackConfig, app_config::AppConfig, Saveable, signatures::Signatures};
use std::{sync::{Mutex, Arc, RwLock}, time::Duration, thread};
use egui_glium::egui_winit::egui::Color32;
use glium::glutin;
use crate::gui::GuiMode;
mod gui;
mod yeti;
mod error;


// Begin Logging
// let path_buf= PathBuf::from(LOG_PATH).canonicalize().unwrap();
// let log_path = path_buf.to_str().unwrap();
// log4rs::init_file(log_path,Default::default()).unwrap();


//yeti.start("./target/debug/yeti_dll.dll",log_path).unwrap();

fn main() {
    let event_loop = glutin::event_loop::EventLoop::with_user_event();
    let display = create_display(&event_loop);
    let mut cursor_hittest_toggle = false;
    display.gl_window().window().set_cursor_hittest(cursor_hittest_toggle).unwrap();
    display.gl_window().window().set_always_on_top(!cursor_hittest_toggle);
    display.gl_window().window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(event_loop.primary_monitor())));
    display.gl_window().window().set_decorations(false);
    let mut egui_glium = egui_glium::EguiGlium::new(&display);

    let mut app_config = AppConfig::open_file_or_create(&PathBuf::from("./yeti.json"), &mut Vec::new());
    let mut hack_config = YetiHackConfig::open_file_or_create(&app_config.hack_config_path, &mut Vec::new());
    let path_buf= PathBuf::from("./log-config.yaml").canonicalize().unwrap();
    let log_path = path_buf.to_str().unwrap();
    log4rs::init_file(log_path,Default::default()).unwrap();

    
    let mut yeti = Yeti::new().unwrap();
    let sigs = Signatures::load(app_config.signatures_path.to_str().unwrap());
    if let Some(sig) = sigs{
        yeti.update_signatures(sig);
    }else{
        
        yeti.update_signatures(Yeti::signatures_scan( app_config.signatures_config_path.to_str().unwrap(),app_config.signatures_path.to_str().unwrap()));
    }
    // ctrlc::set_handler(move || {
    //     drop(yeti)
    // }).expect("Error setting Ctrl-C handler");
    
    
    // //varsx
    let mut mode = gui::GuiMode::InGame;
    let mut tab = gui::SettingsTab::Bhops;

    event_loop.run(move |event, _, control_flow| {
        let mut redraw = || {
            
            let mut quit = false;

            let needs_repaint = egui_glium.run(&display, |egui_ctx| {
                match mode{
                    gui::GuiMode::InGame => {gui::gui_in_game(&display,egui_ctx, &mut hack_config,&mut yeti,&mut cursor_hittest_toggle)},
                    gui::GuiMode::Settings => {gui::gui_settings(&display, egui_ctx, &mut app_config,&mut hack_config,&mut tab,&mut cursor_hittest_toggle,&mut yeti)},
                    gui::GuiMode::GuiEditing => {}
                }
                
            });

            *control_flow = if quit {
                glutin::event_loop::ControlFlow::Exit
            } else if needs_repaint {
                display.gl_window().window().request_redraw();
                glutin::event_loop::ControlFlow::Poll
            } else {
                glutin::event_loop::ControlFlow::Wait
            };

            {
                use glium::Surface as _;
                let mut target = display.draw();

                let color = egui_glium::egui_winit::egui::Rgba::from_rgb(0.0, 0.0, 0.0);
                target.clear_color(color[0], color[1], color[2], 0.0);

                // draw things behind egui here

                egui_glium.paint(&display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };

        match event {
            // Platform-dependent event handlers to workaround a winit bug
            // See: https://github.com/rust-windowing/winit/issues/987
            // See: https://github.com/rust-windowing/winit/issues/1619
            glutin::event::Event::RedrawEventsCleared if cfg!(windows) => redraw(),
            glutin::event::Event::RedrawRequested(_) if !cfg!(windows) => redraw(),
            glutin::event::Event::DeviceEvent {event,..} => {
                match event{
                    glutin::event::DeviceEvent::Key(key) => {
                        if !cursor_hittest_toggle &&key.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::F1 && key.state == glutin::event::ElementState::Released{
                            cursor_hittest_toggle = !cursor_hittest_toggle;
                            println!("Into Menu");
                            display.gl_window().window().set_cursor_hittest(cursor_hittest_toggle).unwrap();
                            display.gl_window().window().set_always_on_top(!cursor_hittest_toggle);
                            
                            
                        }
                        if key.virtual_keycode.unwrap() == glutin::event::VirtualKeyCode::F2 && key.state == glutin::event::ElementState::Released{
                            println!("Settings");
                            // cursor_hittest_toggle = !cursor_hittest_toggle;
                            // display.gl_window().window().set_cursor_hittest(cursor_hittest_toggle).unwrap();
                            mode = if let GuiMode::InGame = mode {GuiMode::Settings}else{GuiMode::InGame};
                            
                        }

                    }
                    _ => {}
                }
            },
            glutin::event::Event::WindowEvent { event, .. } => {
                use glutin::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                }

                egui_glium.on_event(&event);

                display.gl_window().window().request_redraw(); // TODO: ask egui if the events warrants a repaint instead
            }

            _ => (),
        }
    });
}

fn create_display(event_loop: &glutin::event_loop::EventLoop<()>) -> glium::Display {
    let window_builder = glutin::window::WindowBuilder::new()
        .with_resizable(true)
        .with_transparent(true)
        .with_inner_size(glutin::dpi::LogicalSize {
            width: 800.0,
            height: 600.0,
        })
        .with_title("YetiReborn");

    let context_builder = glutin::ContextBuilder::new()
        .with_depth_buffer(0)
        .with_srgb(true)
        .with_stencil_buffer(0)
        .with_vsync(true);

    glium::Display::new(window_builder, context_builder, event_loop).unwrap()
}