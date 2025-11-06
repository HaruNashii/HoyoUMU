use crate::{
    system::{
        create_desktop_file::create_desktop_file, create_protonfixes::create_proton_fixes, download_icon::download_icon, file_and_dirs::{create_dirs, remove_dirs, remove_files}, github_api::check_if_github_api_is_available, hoyoplay::{check_if_hoyoplay_exist, download_hoyoplay, run_hoyoplay_setup}, proton_ge::{check_if_latest_proton_ge_exist, download_proton_ge}, umu::{check_umu, create_umu_config}
    },
    ui::pages::{
        already_installed_pe, downloading_pe, github_api_unavailabe_pe, ButtonId::{self}, PageId
    }
};
use rust_page_system::system::{page_system::PageData, state::AppState};
use std::thread;

pub static mut DOWNLOADING_FLAG: Option<bool> = None;

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::Install == button_id
        {
            if !check_if_github_api_is_available()
            {
                page_data.forced_persistent_elements = Some(vec![github_api_unavailabe_pe()]);
            }
            else 
            {
                if check_if_hoyoplay_exist()
                {
                    page_data.forced_persistent_elements = Some(vec![already_installed_pe(false)]);
                }
                else
                {
                    page_data.forced_persistent_elements = Some(vec![downloading_pe(false)]);
                    thread::spawn(move || {
                        println!("\n# ==== Install Button Clicked! ==== #");
                        unsafe { DOWNLOADING_FLAG = Some(true) };
                        let path_to_umu = check_umu();
                        create_dirs();
                        create_umu_config();
                        create_proton_fixes();
                        download_proton_ge();
                        download_hoyoplay();
                        download_icon();
                        create_desktop_file(&path_to_umu);
                        run_hoyoplay_setup(&path_to_umu);
                        unsafe { DOWNLOADING_FLAG = Some(false) };
                    });
                };
            };
        };
        if &ButtonId::Update == button_id
        {
            if check_if_github_api_is_available()
            {
                page_data.forced_persistent_elements = Some(vec![github_api_unavailabe_pe()]);
            }
            else 
            {
                if check_if_latest_proton_ge_exist(None)
                {
                    page_data.forced_persistent_elements = Some(vec![already_installed_pe(true)]);
                }
                else
                {
                    page_data.forced_persistent_elements = Some(vec![downloading_pe(true)]);
                    thread::spawn(move || {
                        println!("\n# ==== Update Button Clicked! ==== #");
                        unsafe { DOWNLOADING_FLAG = Some(true) };
                        download_proton_ge();
                        unsafe { DOWNLOADING_FLAG = Some(false) };
                    });
                }
            }
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
        if &ButtonId::ConfirmPopUP == button_id
        {
            page_data.forced_persistent_elements = None;
        };
    }
}
