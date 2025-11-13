use crate::{system::setup_rps::{ButtonId, PageId}, ui::style::{FOREGROUND_COLOR, GREEN_COLOR, RED_COLOR, TEXT_COLOR}, helpers::github_api::GITHUB_API_TIME_RESET };
use rust_page_system::{get_center, PersistentElements, system::page_system::Button};
use sdl3::{pixels::Color, rect::Rect};



pub fn umu_run_dont_exist() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((85, 90), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 95), (centered_image.w, centered_image.h), "icons/hutao_umu.bmp".to_string())
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (16.0, (85, 215), "'umu-run' Is Not Installed \n       Please Install It First".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), 
        images: Some(all_images) 
    }
}

pub fn download_not_succeed_proton_pe(is_from_install: bool) -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== buttons =========================
    let mut all_buttons = vec!
    [
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }
    ];
    if is_from_install
    {
        all_buttons.push(Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y, centered_button.w, centered_button.h), radius: 10, id: ButtonId::RetryProton, has_transition: None });
    }
    else
    {
        all_buttons.push(Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Update, has_transition: None });
    };

    //===================== texts =========================
    let all_texts = vec!
    [
        (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR), 
        (16.0, (34, 120), "Proton-GE installation didn't work \nWould you like to retry the installation?".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), images: None 
    }
}

pub fn download_not_succeed_pe() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 375), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image_pos = get_center((85, 100), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_colapse.bmp".to_string())
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 150, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }, 
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 60, centered_button.w, centered_button.h), radius: 10, id: ButtonId::RetryAll, has_transition: None }
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR), 
        (16.0, (32, 125), "Couldn't find HoyoPlay,   please don't \n install the launcher on a custom folder,\n only games supports custom folders!!! \n \nWould you like to retry the installation?".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), 
        images: Some(all_images) 
    }
}

pub fn warning_update() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 375), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((85, 85), (350, 450));

    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: GREEN_COLOR, rect:  Rect::new(centered_button.pos_x, centered_button.pos_y + 65, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ContinueProtonDownload, has_transition: None },
        Button { enabled: true, color: RED_COLOR,   rect:  Rect::new(centered_button.pos_x, centered_button.pos_y + 145, centered_button.w, centered_button.h), radius: 10, id: ButtonId::CancelProtonDownload, has_transition: None }
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 50), (centered_image.w, centered_image.h), "icons/warning.bmp".to_string())
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (15.0, (55, 146), "Launcher is not installed but update \nbutton was clicked, this button just \nupdate or install the Proton-GE, if \nyou do not install HoyoPlay this \ninstallation will serve no porpuse".to_string(), TEXT_COLOR), 
        (18.0, (all_buttons[0].rect.x + 23, all_buttons[0].rect.y + 18), "Download Anyway".to_string(), TEXT_COLOR),
        (18.0, (all_buttons[1].rect.x + 73, all_buttons[1].rect.y + 18), "Cancel".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), images: Some(all_images) 
    }
}

pub fn uninstall_succeeded() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((85, 100), (350, 450));

    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 75), (centered_image.w, centered_image.h), "icons/hutao_colapse.bmp".to_string())
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (15.0, (30, 180), "                     Everything Uninstalled!!! \n Sorry for not meeting your expectations :( \n".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), images: Some(all_images) 
    }
}

pub fn github_api_unavailabe_pe() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((85, 100), (350, 450));

    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 75), (centered_image.w, centered_image.h), "icons/hutao_colapse.bmp".to_string())
    ];

    //===================== texts =========================
    let mut all_texts = vec!
    [
        (16.0, (55, 180), "Sorry, Github API Is Not Available \n            Please Try Again Later!!!".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay :(".to_string(), TEXT_COLOR)
    ];

    if let Some(github_api_time_reset) = GITHUB_API_TIME_RESET.lock().unwrap().clone()
    {
        all_texts.push((16.0, (90, 225), format!("Github API will reset in: \n   {}", github_api_time_reset), TEXT_COLOR))
    };

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_texts), images: Some(all_images) 
    }
}

pub fn downloading_pe(downloading_hoyoplay: bool, downloading_proton: bool, setuping_final_tweaks: bool) -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_image = get_center((50, 50), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== texts =========================
    let mut all_texts = Vec::new();
    if setuping_final_tweaks
    {
        all_texts.push((16.0, (63, 180), "Running Final Tweaking For An \n              Amazing Adventure \n\n                Please Wait <3!!!".to_string(), TEXT_COLOR))
    }
    if downloading_proton
    {
        all_texts.push((16.0, (45, 210), "Downloading The Latest Proton-GE \n                       Please Wait <3!!!".to_string(), TEXT_COLOR))
    }
    if downloading_hoyoplay
    {
        all_texts = vec![(16.0, (30, 100), "          Downloading Hoyoplay For You \n                           Please Wait <3!!! \n\n  (Please don't install the launcher with \n  an custom installation, only games \n  supports custom installation/folders!!!) \n\n (also, make sure to close the launcher \n after you finish setting it up!!!)".to_string(), TEXT_COLOR)];
    };

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: None, 
        texts: Some(all_texts), 
        images: Some(all_images) 
    }
}

pub fn using_local_proton() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_image = get_center((50, 50), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (30, 180), "         Github API Not Available!!!\n Using Your Local Proton Version...".to_string(), TEXT_COLOR)
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: None, 
        texts: Some(all_text), 
        images: Some(all_images) 
    }
}

pub fn loading() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_image = get_center((50, 50), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (18.0, (135, 210), "Loading...".to_string(), TEXT_COLOR)
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: None, 
        texts: Some(all_text), 
        images: Some(all_images) 
    }
}

pub fn download_succeed(is_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image_pos = get_center((85, 90), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }
    ];

    //===================== images =========================
    let mut all_images = Vec::new();
    if is_proton
    {
        all_images.push(((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_proton.bmp".to_string()));
    }
    else
    {
        all_images.push(((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_all.bmp".to_string()));
    }

    //===================== texts =========================
    let mut all_text = vec![(25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];
    if is_proton
    {
        all_text.push((18.0, (75, 180), "Downloaded The Latest \n   Proton-GE Version!!!".to_string(), TEXT_COLOR));
    }
    else
    {
        all_text.push((18.0, (40, 215), "       Everything Downloaded!!! \n    Thanks for using this app <3".to_string(), TEXT_COLOR));
    };

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_text), 
        images: Some(all_images) 
    }
}

pub fn proton_already_installed_pe() -> PersistentElements<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((85, 90), (350, 450));

    //===================== rects =========================
    let all_rects = vec!
    [
        (FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))
    ];

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 95), (centered_image.w, centered_image.h), "icons/hutao_proton.bmp".to_string())
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR),
        (18.0, (32, 210), "Your Proton-GE Version Is Already\n                        The Latest!!!".to_string(), TEXT_COLOR)
    ];

    PersistentElements 
    { 
        id: PageId::DownloadingPersistentElements, 
        background_color: Some(Color::RGBA(0, 0, 0, 155)), 
        rects: Some(all_rects), 
        buttons: Some(all_buttons), 
        texts: Some(all_text), 
        images: Some(all_images) 
    }
}
