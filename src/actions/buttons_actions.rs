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
    ui::pages::{
        ButtonId::{self},
        PageId
    }
};
use rust_page_system::system::{page_system::PageData, state::AppState};
use std::{thread, time::Duration};

// Added for safe global state management
use lazy_static::lazy_static;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, AtomicU8, Ordering}
};

// Use Mutex<Option<T>> for Option types and AtomicBool/U8 for simple booleans.
lazy_static! {
    pub static ref HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref HOYOPLAY_DOWNLOAD_SUCCEEDED: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref PROTON_DOWNLOAD_SUCCEEDED: Mutex<Option<(bool, bool)>> = Mutex::new(None);
    pub static ref ALL_DOWNLOAD_SUCCEEDED: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref UMU_RUN_EXIST: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref PROTON_EXIST: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref PROTON_LATEST_EXIST: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref HOYOPLAY_EXIST: Mutex<Option<bool>> = Mutex::new(None);
    pub static ref GITHUB_API_AVAILABLE: Mutex<Option<bool>> = Mutex::new(None);
}

// atomic values for booleans and stage
pub static STAGE: AtomicU8 = AtomicU8::new(1);
pub static LOADING: AtomicBool = AtomicBool::new(false);
pub static DOWNLOADING_PROTON_GE: AtomicBool = AtomicBool::new(false);
pub static DOWNLOADING_HOYOPLAY_SETUP: AtomicBool = AtomicBool::new(false);
pub static DOWNLOADING_HOYOPLAY: AtomicBool = AtomicBool::new(false);
pub static DOWNLOADING_OTHERS: AtomicBool = AtomicBool::new(false);

pub fn button_action(app_state: &mut AppState<PageId, ButtonId>, button_id: &ButtonId, page_data: &mut PageData<PageId, ButtonId>)
{
    if !app_state.capturing_input.0
    {
        if &ButtonId::Install == button_id
        {
            LOADING.store(true, Ordering::SeqCst);
            thread::spawn(move || {
                // Stage 1: UMU Logic
                if STAGE.load(Ordering::SeqCst) == 1
                {
                    println!("Stage 1 Runned.");
                    let option_path_to_umu = check_umu();
                    if option_path_to_umu.is_some()
                    {
                        *UMU_RUN_EXIST.lock().unwrap() = Some(true);
                    }
                    else
                    {
                        LOADING.store(false, Ordering::SeqCst);
                        *UMU_RUN_EXIST.lock().unwrap() = Some(false);
                        return;
                    };
                    STAGE.store(2, Ordering::SeqCst);
                }

                // Stage 2: PROTON-GE Logic: check GitHub API
                if STAGE.load(Ordering::SeqCst) == 2
                {
                    println!("Stage 2 Runned.");
                    if check_if_github_api_is_available()
                    {
                        *GITHUB_API_AVAILABLE.lock().unwrap() = Some(true);
                    }
                    STAGE.store(3, Ordering::SeqCst);
                }

                if STAGE.load(Ordering::SeqCst) == 3
                {
                    println!("Stage 3 Runned.");
                    let github_api_available = *GITHUB_API_AVAILABLE.lock().unwrap();
                    if github_api_available == Some(true)
                    {
                        println!("case 1");
                        if !check_if_proton_ge_exist(None, true)
                        {
                            println!("case 2");
                            LOADING.store(false, Ordering::SeqCst);
                            DOWNLOADING_PROTON_GE.store(true, Ordering::SeqCst);
                            download_proton_ge();
                            DOWNLOADING_PROTON_GE.store(false, Ordering::SeqCst);
                            LOADING.store(true, Ordering::SeqCst);

                            if !check_if_proton_ge_exist(None, true)
                            {
                                println!("case 3");
                                LOADING.store(false, Ordering::SeqCst);
                                *PROTON_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some((false, true));
                                return;
                            }
                        }
                    }
                    // needed this because ".is_none()" creates a reference, but with mutex we can check Option directly
                    if github_api_available.is_none()
                    {
                        println!("case 4");
                        if check_if_proton_ge_exist(None, false)
                        {
                            println!("case 5");
                            LOADING.store(false, Ordering::SeqCst);
                            *PROTON_EXIST.lock().unwrap() = Some(true);
                            thread::sleep(Duration::from_secs(5));
                            *PROTON_EXIST.lock().unwrap() = None;
                            LOADING.store(true, Ordering::SeqCst);
                        }
                        else
                        {
                            println!("case 6");
                            LOADING.store(false, Ordering::SeqCst);
                            *GITHUB_API_AVAILABLE.lock().unwrap() = Some(false);
                            return;
                        }
                    }
                    STAGE.store(4, Ordering::SeqCst);
                }

                // Stage 4: HOYOPLAY Logic
                if STAGE.load(Ordering::SeqCst) == 4
                {
                    println!("Stage 4 Runned.");
                    if !check_if_hoyoplay_exist()
                    {
                        //Download HoyoPlay Setup
                        if !check_if_hoyoplay_setup_exist()
                        {
                            LOADING.store(false, Ordering::SeqCst);
                            DOWNLOADING_HOYOPLAY_SETUP.store(true, Ordering::SeqCst);
                            download_hoyoplay_setup();
                            DOWNLOADING_HOYOPLAY_SETUP.store(false, Ordering::SeqCst);
                            LOADING.store(true, Ordering::SeqCst);
                        }
                        //recheck if hoyoplay setup was truly installed
                        if check_if_hoyoplay_setup_exist()
                        {
                            *HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(true);
                        }
                        else
                        {
                            LOADING.store(false, Ordering::SeqCst);
                            *HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(false);
                            eprintln!("Hoyoplay setup download failed");
                            return;
                        };

                        // Download HoyoPlay
                        let path_to_umu = check_umu();
                        if !check_if_hoyoplay_exist()
                        {
                            LOADING.store(false, Ordering::SeqCst);
                            DOWNLOADING_HOYOPLAY.store(true, Ordering::SeqCst);
                            run_hoyoplay_setup(&path_to_umu.unwrap());
                            DOWNLOADING_HOYOPLAY.store(false, Ordering::SeqCst);
                            LOADING.store(true, Ordering::SeqCst);
                        }
                        //recheck if hoyoplay was truly installed
                        if check_if_hoyoplay_exist()
                        {
                            *HOYOPLAY_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(true);
                        }
                        else
                        {
                            LOADING.store(false, Ordering::SeqCst);
                            *HOYOPLAY_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(false);
                            return;
                        }
                    }
                    STAGE.store(5, Ordering::SeqCst);
                }

                // Stage 5: DOWNLOADING final tasks
                if STAGE.load(Ordering::SeqCst) == 5
                {
                    println!("Stage 5 Runned.");
                    if check_if_proton_ge_exist(None, false) && check_if_hoyoplay_exist()
                    {
                        LOADING.store(false, Ordering::SeqCst);
                        DOWNLOADING_OTHERS.store(true, Ordering::SeqCst);
                        thread::sleep(Duration::from_secs(2));
                        create_dirs();
                        create_umu_config();
                        create_proton_fixes();
                        download_icon();
                        let path_to_umu = check_umu();
                        create_desktop_file(&path_to_umu.unwrap());
                        DOWNLOADING_OTHERS.store(false, Ordering::SeqCst);
                        *ALL_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(true);
                        STAGE.store(1, Ordering::SeqCst);
                    }
                    else
                    {
                        *ALL_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some(false);
                    };
                }
            });
        };

        if &ButtonId::Update == button_id
        {
            LOADING.store(true, Ordering::SeqCst);
            thread::spawn(move || {
                // ========= CHECKING =========
                let result = check_if_github_api_is_available();
                if !result
                {
                    LOADING.store(false, Ordering::SeqCst);
                }
                *GITHUB_API_AVAILABLE.lock().unwrap() = Some(result);

                // ========= DOWNLOADING =========
                println!("\n# ==== Update Button Clicked! ==== #");
                if result
                {
                    if check_if_proton_ge_exist(None, true)
                    {
                        LOADING.store(false, Ordering::SeqCst);
                        *PROTON_LATEST_EXIST.lock().unwrap() = Some(true);
                        return;
                    }
                    LOADING.store(false, Ordering::SeqCst);
                    DOWNLOADING_PROTON_GE.store(true, Ordering::SeqCst);
                    download_proton_ge();
                    if check_if_proton_ge_exist(None, true)
                    {
                        DOWNLOADING_PROTON_GE.store(false, Ordering::SeqCst);
                        *PROTON_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some((true, false));
                    }
                    else
                    {
                        DOWNLOADING_PROTON_GE.store(false, Ordering::SeqCst);
                        *PROTON_DOWNLOAD_SUCCEEDED.lock().unwrap() = Some((false, false));
                    };
                }
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
        if &ButtonId::ConfirmPopUP == button_id
        {
            page_data.forced_persistent_elements = None;
            {
                *HOYOPLAY_SETUP_DOWNLOAD_SUCCEEDED.lock().unwrap() = None;
                *HOYOPLAY_DOWNLOAD_SUCCEEDED.lock().unwrap() = None;
                *PROTON_DOWNLOAD_SUCCEEDED.lock().unwrap() = None;
                *ALL_DOWNLOAD_SUCCEEDED.lock().unwrap() = None;

                DOWNLOADING_PROTON_GE.store(false, Ordering::SeqCst);
                DOWNLOADING_HOYOPLAY_SETUP.store(false, Ordering::SeqCst);
                DOWNLOADING_HOYOPLAY.store(false, Ordering::SeqCst);
                DOWNLOADING_OTHERS.store(false, Ordering::SeqCst);

                *UMU_RUN_EXIST.lock().unwrap() = None;
                *PROTON_EXIST.lock().unwrap() = None;
                *PROTON_LATEST_EXIST.lock().unwrap() = None;
                *HOYOPLAY_EXIST.lock().unwrap() = None;

                *GITHUB_API_AVAILABLE.lock().unwrap() = None;

                STAGE.store(1, Ordering::SeqCst);

                LOADING.store(false, Ordering::SeqCst);
            };
        };
        if &ButtonId::RetryProton == button_id
        {
            STAGE.store(1, Ordering::SeqCst);
            button_action(app_state, &ButtonId::Install, page_data);
        }
        if &ButtonId::RetryAll == button_id
        {
            STAGE.store(2, Ordering::SeqCst);
            button_action(app_state, &ButtonId::Install, page_data);
        }
    }
}
