use crate::{
    system::{
        create_desktop_file::create_desktop_file,
        create_protonfixes::create_proton_fixes,
        download_hoyoplay::download_hoyoplay,
        download_icon::download_icon,
        download_proton_ge::download_proton_ge,
        file_and_dirs::{create_dirs, remove_dirs, remove_files},
        run_hoyoplay_setup::run_hoyoplay_setup,
        umu::{check_umu, create_umu_config}
    },
    ui::pages::{
        ButtonId::{self},
        PageId
    }
};
use rust_page_system::system::{page_system::PageData, state::AppState};
use std::thread;

pub static mut INSTALL_FLAG: bool = false;

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::Install == button_id
        {
            thread::spawn(move || {
                println!("\n# ==== Install Button Clicked! ==== #");
                unsafe { INSTALL_FLAG = true };
                let path_to_umu = check_umu();
                create_dirs();
                create_umu_config();
                create_proton_fixes();
                download_proton_ge();
                download_hoyoplay();
                download_icon();
                create_desktop_file(&path_to_umu);
                run_hoyoplay_setup(&path_to_umu);
                unsafe { INSTALL_FLAG = false };
            });
        };
        if &ButtonId::Update == button_id
        {
            thread::spawn(move || {
                println!("\n# ==== Update Button Clicked! ==== #");
                unsafe { INSTALL_FLAG = true };
                download_proton_ge();
                unsafe { INSTALL_FLAG = false };
            });
        };
        if &ButtonId::Unistall == button_id
        {
            println!("\n# ==== Uninstall Button Clicked! ==== #");
            app_state.change_current_page(page_data, PageId::AreYouSurePage, button_id);
        };
        if &ButtonId::ConfirmUninstall == button_id
        {
            println!("\n# ==== Confirm Uninstall Button Clicked! ==== #");
            remove_files();
            remove_dirs();
        };
        if &ButtonId::NoConfirmUninstall == button_id
        {
            println!("\n# ==== No Confirm Uninstall Button Clicked! ==== #");
            app_state.change_current_page(page_data, PageId::MainPage, button_id);
        };
    }
}
