use crate::{
    system::github_api::GITHUB_API_TIME_RESET,
    ui::style::{BACKGROUND_COLOR, BLUE_COLOR, FOREGROUND_COLOR, GREEN_COLOR, RED_COLOR, TEXT_COLOR, YELLOW_COLOR}
};
use rust_page_system::{
    PersistentElements, get_center,
    system::page_system::{Button, Page}
};
use sdl3::{pixels::Color, rect::Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageId
{
    MainPage,
    AreYouSurePage,
    DownloadingPersistentElements
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum ButtonId
{
    Install,
    Update,
    Unistall,
    ConfirmUninstall,
    NoConfirmUninstall,
    ConfirmPopUP,
    RetryAll,
    RetryProton
}

pub fn umu_run_dont_exist() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    let centered_image_pos = get_center((85, 90), (350, 450));

    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== images =========================
    let all_images = vec![((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_umu.bmp".to_string())];

    //===================== texts =========================
    let all_texts = vec![(16.0, (85, 215), "'umu-run' Is Not Installed \n       Please Install It First".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: Some(all_images) }
}

pub fn download_not_succeed_proton_pe_but_has_local() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 375), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 140, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }, Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y - 20, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Update, has_transition: None }, Button { enabled: true, color: YELLOW_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 60, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== texts =========================
    let all_texts = vec![(18.0, (all_buttons[1].rect.x + 35, all_buttons[1].rect.y + 18), "Yes, Try Again!!".to_string(), TEXT_COLOR), (18.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR), (18.0, (all_buttons[2].rect.x + 15, all_buttons[2].rect.y + 18), "Continue With Local".to_string(), TEXT_COLOR), (16.0, (57, 65), "Installation of Proton-GE Latest \ndidn't work, You you like to retry \nor keep with your existing version?".to_string(), TEXT_COLOR)];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: None }
}

pub fn download_not_succeed_proton_pe(is_from_install: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let mut all_buttons = vec![Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];
    if is_from_install
    {
        all_buttons.push(Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y, centered_button.w, centered_button.h), radius: 10, id: ButtonId::RetryProton, has_transition: None });
    }
    else
    {
        all_buttons.push(Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Update, has_transition: None });
    };

    //===================== texts =========================
    let all_texts = vec![(25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR), (16.0, (34, 120), "Proton-GE installation didn't work \nWould you like to retry the installation?".to_string(), TEXT_COLOR)];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: None }
}

pub fn download_not_succeed_pe() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 375), (350, 450));
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image_pos = get_center((85, 100), (350, 450));

    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== images =========================
    let all_images = vec![((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_colapse.bmp".to_string())];

    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 150, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }, Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 60, centered_button.w, centered_button.h), radius: 10, id: ButtonId::RetryAll, has_transition: None }];

    //===================== texts =========================
    let all_texts = vec![(25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR), (16.0, (32, 125), "Couldn't find HoyoPlay,   please don't \n install the launcher on a custom folder,\n only games supports custom folders!!! \n \nWould you like to retry the installation?".to_string(), TEXT_COLOR)];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: Some(all_images) }
}

pub fn github_api_unavailabe_pe() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== images =========================
    let centered_image_pos = get_center((85, 100), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, 75), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_colapse.bmp".to_string())];

    //===================== texts =========================
    let mut all_texts = vec![(16.0, (55, 180), "Sorry, Github API Is Not Available \n            Please Try Again Later!!!".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay :(".to_string(), TEXT_COLOR)];
    // Lock the mutex to check and clone the reset time value
    if let Some(github_api_time_reset) = GITHUB_API_TIME_RESET.lock().unwrap().clone()
    {
        all_texts.push((16.0, (90, 225), format!("Github API will reset in: \n   {}", github_api_time_reset), TEXT_COLOR))
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: Some(all_images) }
}

pub fn downloading_pe(downloading_hoyoplay: bool, downloading_proton: bool, setuping_final_tweaks: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

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
        all_texts = vec![(16.0, (55, 210), "Downloading Hoyoplay For You \n                  Please Wait <3!!!".to_string(), TEXT_COLOR)];
    };

    let centered_image = get_center((50, 50), (350, 450));
    //===================== images =========================
    let all_images = vec![((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: None, texts: Some(all_texts), images: Some(all_images) }
}

pub fn using_local_proton() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== texts =========================
    let all_text = vec![(18.0, (30, 180), "         Github API Not Available!!!\n Using Your Local Proton Version...".to_string(), TEXT_COLOR)];

    let centered_image = get_center((50, 50), (350, 450));
    //===================== images =========================
    let all_images = vec![((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn loading() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== texts =========================
    let all_text = vec![(18.0, (135, 210), "Loading...".to_string(), TEXT_COLOR)];

    let centered_image = get_center((50, 50), (350, 450));
    //===================== images =========================
    let all_images = vec![((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/hutao_spinning.gif".to_string())];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: None, texts: Some(all_text), images: Some(all_images) }
}

pub fn download_succeed(is_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    let centered_image_pos = get_center((85, 90), (350, 450));
    //===================== images =========================
    let mut all_images = Vec::new();

    //===================== texts =========================
    let mut all_text = vec![(25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];
    if is_proton
    {
        all_images.push(((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_proton.bmp".to_string()));
        all_text.push((18.0, (75, 180), "Downloaded The Latest \n   Proton-GE Version!!!".to_string(), TEXT_COLOR));
    }
    else
    {
        all_images.push(((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_all.bmp".to_string()));
        all_text.push((18.0, (40, 215), "       Everything Downloaded!!! \n    Thanks for using this app <3".to_string(), TEXT_COLOR));
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}

pub fn already_installed_pe(is_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== images =========================
    let mut all_images = Vec::new();

    //===================== texts =========================
    let mut all_text = vec![(25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];
    if is_proton
    {
        all_text.push((18.0, (32, 210), "Your Proton-GE Version Is Already\n                        The Latest!!!".to_string(), TEXT_COLOR));
        let centered_image_pos = get_center((85, 90), (350, 450));
        all_images.push(((centered_image_pos.pos_x, 95), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_proton.bmp".to_string()));
    }
    else
    {
        all_text.push((18.0, (32, 180), "Everything Already Downloaded!!! \n     Thanks for using this app <3".to_string(), TEXT_COLOR));
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}

pub fn main_page() -> Page<PageId, ButtonId>
{
    let centered_button_pos = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: BLUE_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y - 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Install, has_transition: None }, Button { enabled: true, color: YELLOW_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Update, has_transition: None }, Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 150, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Unistall, has_transition: None }];

    //===================== texts =========================
    let all_text = vec![(25.0, (all_buttons[0].rect.x + 60, all_buttons[0].rect.y + 15), "Install".to_string(), TEXT_COLOR), (25.0, (all_buttons[1].rect.x + 60, all_buttons[1].rect.y + 15), "Update".to_string(), TEXT_COLOR), (25.0, (all_buttons[2].rect.x + 55, all_buttons[2].rect.y + 15), "Unistall".to_string(), TEXT_COLOR)];

    //===================== images =========================
    let centered_image_pos = get_center((100, 150), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, -5), (centered_image_pos.w, centered_image_pos.h), "icons/hutao.bmp".to_string())];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::MainPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}

pub fn are_you_sure_page() -> Page<PageId, ButtonId>
{
    let centered_button_pos = get_center((220, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::ConfirmUninstall, has_transition: None }, Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 150, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::NoConfirmUninstall, has_transition: None }];

    //===================== texts =========================
    let all_text = vec![(13.0, (65, 120), "This Action Will Remove Everything \nRelated To HoyoPlay And This Project \nLike: (Games, LauncherData, LoginData, \nWinePrefix, Desktop Files, Icons, Etc...)\n \nAre You Sure You Want To Continue?".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 33, all_buttons[0].rect.y + 15), "Yes, I'm Sure!".to_string(), TEXT_COLOR), (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 15), "No, Please Don't!".to_string(), TEXT_COLOR)];

    let centered_image_pos = get_center((90, 140), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, -10), (centered_image_pos.w, centered_image_pos.h), "icons/hutao_colapse.bmp".to_string())];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::AreYouSurePage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}
