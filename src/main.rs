use crate::{
    actions::buttons_actions::{button_action, INSTALL_FLAG},
    system::setup_rps::populate_page_data,
    ui::pages::PageId
};
use lazy_static::lazy_static;
use rust_page_system::{
    include_project_assets,
    system::{
        input_handler::InputHandler,
        page_system::PageData,
        renderer::RendererConfig,
        state::AppState,
        window::{create_window, get_monitor_refresh_rate, WindowConfig}
    },
    Renderer
};

use include_dir::Dir;
pub static ASSETS: Dir = include_project_assets!();

use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_DISABLED;
use std::{env, time::Duration};

lazy_static! 
{
    pub static ref HOME_DIR: String = env::home_dir().unwrap().display().to_string();
}

pub mod actions;
pub mod system;
pub mod ui;

fn main()
{
    //list_embedded(&ASSETS);
    let window_config = WindowConfig {
        window_title: "AdvancedExample".to_string(),
        icon: (Some("icons/hoyoumu_icon.bmp".to_string()), Some(&ASSETS)),
        // Recommended to start with 16:9 aspect ratio
        start_window_size: (350, 450),
        // Recommended to have minimum size with 16:9 aspect ratio
        window_minimum_size: (300, 450),
        resizable: false,
        centered: true,
        different_sdl_presentation_mode: Some(SDL_LOGICAL_PRESENTATION_DISABLED),
        font: ("Cantarell".to_string(), Some("Bold".to_string()))
    };
    let mut window_modules = create_window(window_config);
    let mut input_handler = InputHandler::new(false);
    let mut app_state = AppState::new(PageId::MainPage, window_modules.canvas.window().size(), window_modules.stretch_mode_status);
    let mut page_data = PageData::new(&app_state);
    let renderer_config = RendererConfig { canvas: window_modules.canvas, texture_creator: &window_modules.texture_creator, ttf_context: &window_modules.ttf_context, font_path: &window_modules.font_path, decrease_color_when_selected: Some((25, 25, 25)), selection_color: Some((0, 0, 200, 125)) };
    let mut renderer = Renderer::new(renderer_config);

    populate_page_data(&mut page_data);

    loop
    {
        //using (900 / your_refresh_rate) to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));
        unsafe { app_state.all_events_disable = INSTALL_FLAG };
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, &mut button_action);
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        page_data.create_current_page(&mut app_state);
        renderer.render(&page_data, &mut app_state, &input_handler);
    }
}
