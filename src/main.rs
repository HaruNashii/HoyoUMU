use rust_page_system::{include_project_assets, Renderer, system::{input_handler::InputHandler, page_system::PageData, renderer::RendererConfig, state::AppState, window::{WindowConfig, create_window, get_monitor_refresh_rate}}};
//use crate::ui::persistent_elements::{proton_already_installed_pe, download_not_succeed_pe, download_not_succeed_proton_pe, download_succeed, downloading_pe, loading, using_local_proton}; // -- Debug
use crate::{actions::buttons_actions::button_action, system::setup_rps::{populate_page_data, PageId}, ui::manage_pe::manage_pe};
use sdl3::sys::render::SDL_LOGICAL_PRESENTATION_DISABLED;
use std::{env, time::Duration};
use lazy_static::lazy_static;
use include_dir::Dir;



pub static ASSETS: Dir = include_project_assets!();



lazy_static! { pub static ref HOME_DIR: String = env::home_dir().unwrap().display().to_string(); }



pub mod actions;
pub mod system;
pub mod helpers;
pub mod ui;



fn main()
{
    let window_config = WindowConfig 
    {
        window_title: "HoyoUMU".to_string(),
        icon: (Some("icons/hutao.bmp".to_string()), Some(&ASSETS)),
        start_window_size: (350, 450),
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
    let renderer_config = RendererConfig { canvas: window_modules.canvas, texture_creator: &window_modules.texture_creator, ttf_context: &window_modules.ttf_context, font_path: &window_modules.font_path, decrease_color_when_selected: Some((25, 25, 25)), selection_color: Some((0, 0, 200, 125)), assets_dir: Some(&ASSETS) };
    let mut renderer = Renderer::new(renderer_config);

    populate_page_data(&mut page_data);

    loop
    {
        //using (900 / your_refresh_rate) to a very crispy experience
        std::thread::sleep(Duration::from_millis(900 / get_monitor_refresh_rate()));

        // === Debug ====
        //page_data.forced_persistent_elements = Some(vec![warning_update()]);
        //page_data.forced_persistent_elements = Some(vec![loading()]);
        //page_data.forced_persistent_elements = Some(vec![downloading_pe(false, true, false)]);
        //page_data.forced_persistent_elements = Some(vec![downloading_pe(true, false, false)]);
        //page_data.forced_persistent_elements = Some(vec![downloading_pe(false, false, true)]);
        //page_data.forced_persistent_elements = Some(vec![uninstall_succeeded()]);
        //page_data.forced_persistent_elements = Some(vec![using_local_proton()]);
        //page_data.forced_persistent_elements = Some(vec![proton_already_installed_pe()]);
        //page_data.forced_persistent_elements = Some(vec![download_not_succeed_pe()]);
        //page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe(true)]);
        //page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe(false)]);
        //page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe_but_has_local()]);
        //page_data.forced_persistent_elements = Some(vec![download_succeed(false)]);
        //page_data.forced_persistent_elements = Some(vec![download_succeed(true)]);
        // ==============

        manage_pe(&mut page_data, &mut app_state);
        input_handler.handle_input(&mut window_modules.event_pump, &mut window_modules.clipboard_system, &mut page_data, &mut app_state, &mut button_action);
        app_state.update_window_size(renderer.canvas.window().size().0, renderer.canvas.window().size().1);
        page_data.create_current_page(&mut app_state);
        renderer.render(&page_data, &mut app_state, &input_handler);
    }
}
