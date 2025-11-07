use crate::{
    system::{
        create_desktop_file::create_desktop_file,
        create_protonfixes::create_proton_fixes,
        download_icon::download_icon,
        file_and_dirs::{create_dirs, remove_dirs, remove_files},
        github_api::check_if_github_api_is_available,
        hoyoplay::{check_if_hoyoplay_exist, check_if_hoyoplay_setup_exist, download_hoyoplay_setup, run_hoyoplay_setup},
        proton_ge::{check_if_proton_ge_exist, download_proton_ge},
        umu::{check_umu, create_umu_config}
    },
    ui::pages::{ButtonId::{self}, PageId}
};
use rust_page_system::system::{page_system::PageData, state::AppState};
use std::{thread, time::Duration};

pub static mut HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED: Option<bool> = None;
pub static mut HOYOPLAY_DOWNLOAD_SUCCEEDED: Option<bool> = None;
pub static mut PROTON_DOWNLOAD_SUCCEEDED: Option<(bool, bool)> = None;
pub static mut ALL_DOWNLOAD_SUCCEEDED: Option<bool> = None;

pub static mut DOWNLOADING_PROTON_GE: bool = false;
pub static mut DOWNLOADING_HOYOPLAY_SETUP: bool = false;
pub static mut DOWNLOADING_HOYOPLAY: bool = false;
pub static mut DOWNLOADING_OTHERS: bool = false;

pub static mut UMU_RUN_EXIST: Option<bool> = None;
pub static mut PROTON_EXIST: Option<bool> = None;
pub static mut PROTON_LATEST_EXIST: Option<bool> = None;
pub static mut HOYOPLAY_EXIST: Option<bool> = None;

pub static mut GITHUB_API_AVAILABLE: Option<bool> = None;

pub static mut STAGE: u8 = 1;
pub static mut LOADING: bool = false;

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::Install == button_id
        {
            unsafe 
            {
                LOADING = true;
                thread::spawn(move || 
                {
                    if STAGE == 1
                    {
                        println!("Stage 1 Runned.");
                        // ========= UMU Logic =========
                        let option_path_to_umu = check_umu();
                        if option_path_to_umu.is_some()
                        {
                            UMU_RUN_EXIST = Some(true);
                        }
                        else
                        {
                            LOADING = false;
                            UMU_RUN_EXIST = Some(false);
                            return 
                        };
                        STAGE = 2;
                    }



                    // ========= PROTON-GE Logic =========
                    if STAGE == 2
                    {
                        println!("Stage 2 Runned.");
                        if check_if_github_api_is_available()
                        {
                            GITHUB_API_AVAILABLE = Some(true);
                        }
                        STAGE = 3;
                    }
                    if STAGE == 3
                    {
                        println!("Stage 3 Runned.");
                        if GITHUB_API_AVAILABLE == Some(true)
                        {
                            println!("case 1");
                            if !check_if_proton_ge_exist(None, true)
                            {
                                println!("case 2");
                                LOADING = false;
                                DOWNLOADING_PROTON_GE = true;
                                download_proton_ge();
                                DOWNLOADING_PROTON_GE = false;
                                LOADING = true;

                                if !check_if_proton_ge_exist(None, true)
                                {
                                    println!("case 3");
                                    LOADING = false;
                                    PROTON_DOWNLOAD_SUCCEEDED = Some((false, true));
                                    return;
                                }
                            }
                        }
                        // needed this bc ".is_none()" creates an reference, and pub mut static reference is a big no no
                        #[allow(clippy::partialeq_to_none)]
                        if GITHUB_API_AVAILABLE == None
                        {
                            println!("case 4");
                            if check_if_proton_ge_exist(None, false)
                            {
                                println!("case 5");
                                LOADING = false;
                                PROTON_EXIST = Some(true);
                                thread::sleep(Duration::from_secs(5));
                                PROTON_EXIST = None;
                                LOADING = true;
                            }
                            else
                            {
                                println!("case 6");
                                LOADING = false;
                                GITHUB_API_AVAILABLE = Some(false);
                                return;
                            }
                        }
                        STAGE = 4;
                    }



                    // ========= HOYOPLAY Logic =========
                    if STAGE == 4
                    {
                        println!("Stage 4 Runned.");
                        if !check_if_hoyoplay_exist()
                        {
                            //Download HoyoPlay Setup
                            if !check_if_hoyoplay_setup_exist()
                            {
                                LOADING = false;
                                DOWNLOADING_HOYOPLAY_SETUP = true;
                                download_hoyoplay_setup();
                                DOWNLOADING_HOYOPLAY_SETUP = false;
                                LOADING = true;
                            }
                            //recheck if hoyoplay setup was truly installed
                            if check_if_hoyoplay_setup_exist()
                            {
                                HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED = Some(true);
                            }
                            else 
                            {
                                LOADING = false;
                                HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED = Some(false);
                                eprintln!("Hoyoplay setup download failed");
                                return;
                            };


                            // Download HoyoPlay
                            let path_to_umu = check_umu();
                            if !check_if_hoyoplay_exist()
                            {
                                LOADING = false;
                                DOWNLOADING_HOYOPLAY = true;
                                run_hoyoplay_setup(&path_to_umu.unwrap());
                                DOWNLOADING_HOYOPLAY = false;
                                LOADING = true;
                            }
                            //recheck if hoyoplay was truly installed
                            if check_if_hoyoplay_exist()
                            {
                                HOYOPLAY_DOWNLOAD_SUCCEEDED = Some(true);
                            }
                            else
                            {
                                LOADING = false;
                                HOYOPLAY_DOWNLOAD_SUCCEEDED = Some(false);
                                return;
                            }
                        }
                        STAGE = 5;
                    }



                    // ========= DOWNLOADING =========
                    if STAGE == 5
                    {
                        println!("Stage 5 Runned.");
                        if check_if_proton_ge_exist(None, false) && check_if_hoyoplay_exist()
                        {
                            LOADING = false;
                            DOWNLOADING_OTHERS = true;
                            thread::sleep(Duration::from_secs(2));
                            create_dirs();
                            create_umu_config();
                            create_proton_fixes();
                            download_icon();
                            let path_to_umu = check_umu();
                            create_desktop_file(&path_to_umu.unwrap());
                            DOWNLOADING_OTHERS = false;
                            ALL_DOWNLOAD_SUCCEEDED = Some(true);
                            STAGE = 1;
                        }
                        else
                        {
                            ALL_DOWNLOAD_SUCCEEDED = Some(false);
                        };
                    }
                });
            }
        };

        if &ButtonId::Update == button_id
        {
            unsafe
            {
                LOADING = true; 
                thread::spawn(move || 
                {
                    // ========= CHECKING =========
                    let result = check_if_github_api_is_available(); 
                    if !result { LOADING = false };
                    GITHUB_API_AVAILABLE = Some(result);

                    // ========= DOWNLOADING =========
                    println!("\n# ==== Update Button Clicked! ==== #");
                    if GITHUB_API_AVAILABLE == Some(true)
                    {
                        if check_if_proton_ge_exist(None, true) { LOADING = false; PROTON_LATEST_EXIST = Some(true); return };
                        LOADING = false;
                        DOWNLOADING_PROTON_GE = true;
                        download_proton_ge();
                        if check_if_proton_ge_exist(None, true)
                        {
                            DOWNLOADING_PROTON_GE = false;
                            PROTON_DOWNLOAD_SUCCEEDED = Some((true, false));
                        }
                        else
                        {
                            DOWNLOADING_PROTON_GE = false;
                            PROTON_DOWNLOAD_SUCCEEDED = Some((false, false));
                        };
                    }
                });
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
            unsafe 
            {
                HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED = None;
                HOYOPLAY_DOWNLOAD_SUCCEEDED = None;
                PROTON_DOWNLOAD_SUCCEEDED = None;
                ALL_DOWNLOAD_SUCCEEDED = None;
                
                DOWNLOADING_PROTON_GE = false;
                DOWNLOADING_HOYOPLAY_SETUP = false;
                DOWNLOADING_HOYOPLAY = false;
                DOWNLOADING_OTHERS = false;
                
                UMU_RUN_EXIST = None;
                PROTON_EXIST = None;
                PROTON_LATEST_EXIST = None;
                HOYOPLAY_EXIST = None;
                
                GITHUB_API_AVAILABLE = None;
                
                STAGE = 1;

                LOADING = false;
            };
        };
        if &ButtonId::RetryProton == button_id
        {
            unsafe 
            {
                STAGE = 1;
                button_action(app_state, &ButtonId::Install, page_data);
            }
        }
        if &ButtonId::RetryAll == button_id
        {
            unsafe 
            {
                STAGE = 2;
                button_action(app_state, &ButtonId::Install, page_data);
            }
        }
    }
}
