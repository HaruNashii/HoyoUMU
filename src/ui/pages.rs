use crate::{
    system::{file_and_dirs::HOYOUMU_FILES, github_api::GITHUB_API_TIME_RESET},
    ui::style::{BACKGROUND_COLOR, BLUE_COLOR, GREEN_COLOR, ORANGE_COLOR, RED_COLOR, FOREGROUND_COLOR, TEXT_COLOR, YELLOW_COLOR}
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
    ConfirmPopUP
}

pub fn download_not_succeed_proton_pe() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None },
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Update, has_transition: None }
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR),
        (16.0, (34, 120), "Proton-GE installation didn't work \nWould you like to retry the installation?".to_string(), TEXT_COLOR)
    ]; 

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: None }
}

pub fn download_not_succeed_pe() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 375), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: ORANGE_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 150, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None },
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 50, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Install, has_transition: None }
    ];

    //===================== texts =========================
    let all_texts = vec!
    [
        (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 13), "Yes, Try Again!!".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 70, all_buttons[0].rect.y + 13), "No :(".to_string(), TEXT_COLOR),
        (16.0, (32, 50), "Hoyoplay installation didn't work \nor was installed in one custom folder, \nplease don't install the launcher on \nan custom folder, only games \n installation supports custom folders. \n \nWould you like to retry the installation?".to_string(), TEXT_COLOR)
    ]; 

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: None }
}

pub fn github_api_unavailabe_pe() -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== texts =========================
    let mut all_texts = vec![(16.0, (55, 180), "Sorry, Github API Is Not Available \n            Please Try Again Later!!!".to_string(), TEXT_COLOR), (25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay :(".to_string(), TEXT_COLOR)];
    #[allow(static_mut_refs)]
    if unsafe{GITHUB_API_TIME_RESET.is_some()}
    {
        let github_api_time_reset = unsafe{GITHUB_API_TIME_RESET.clone().unwrap()};
        all_texts.push((16.0, (90, 225), format!("Github API will reset in: \n   {}", github_api_time_reset), TEXT_COLOR))
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_texts), images: None }
}

pub fn downloading_pe(downloading_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    //===================== texts =========================
    let mut all_texts = Vec::new(); 
    if downloading_proton
    {
        all_texts.push((16.0, (45, 210),   "Downloading The Latest Proton-GE \n                       Please Wait <3!!!".to_string(), TEXT_COLOR))
    }
    else 
    {
        all_texts = vec![(16.0, (55, 210), "Downloading Everything For You \n                  Please Wait <3!!!".to_string(), TEXT_COLOR)];
    };

    let centered_image = get_center((50, 50), (350, 450));
    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, 300), (centered_image.w, centered_image.h), "gifs/herta-kurukuru.gif".to_string())
    ];

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: None, texts: Some(all_texts), images: Some(all_images) }
}

pub fn download_succeed(is_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== texts =========================
    let mut all_text = vec![(25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];
    if is_proton
    {
        all_text.push((18.0, (75, 180), "Downloaded The Latest \n   Proton-GE Version!!!".to_string(), TEXT_COLOR));
    }
    else
    {
        all_text.push((18.0, (40, 180), "       Everything Downloaded!!! \n\n    Thanks for using this app <3".to_string(), TEXT_COLOR));
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn already_installed_pe(is_proton: bool) -> PersistentElements<PageId, ButtonId>
{
    let centered_rect = get_center((300, 300), (350, 450));
    //===================== rects =========================
    let all_rects = vec![(FOREGROUND_COLOR, (Rect::new(centered_rect.pos_x, centered_rect.pos_y, centered_rect.w, centered_rect.h), 5))];

    let centered_button = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 100, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmPopUP, has_transition: None }];

    //===================== texts =========================
    let mut all_text = vec![(25.0, (all_buttons[0].rect.x + 61, all_buttons[0].rect.y + 13), "Okay!!!".to_string(), TEXT_COLOR)];
    if is_proton
    {
        all_text.push((18.0, (32, 180), "Your Proton-GE Version Is Already\n                        The Latest!!!".to_string(), TEXT_COLOR));
    }
    else
    {
        all_text.push((18.0, (32, 180), "Everything Already Downloaded!!! \n     Thanks for using this app <3".to_string(), TEXT_COLOR));
    };

    PersistentElements { id: PageId::DownloadingPersistentElements, background_color: Some(Color::RGBA(0, 0, 0, 155)), rects: Some(all_rects), buttons: Some(all_buttons), texts: Some(all_text), images: None }
}

pub fn main_page() -> Page<PageId, ButtonId>
{
    let centered_button_pos = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec![Button { enabled: true, color: BLUE_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y - 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Install, has_transition: None }, Button { enabled: true, color: YELLOW_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Update, has_transition: None }, Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 150, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Unistall, has_transition: None }];

    //===================== texts =========================
    let all_text = vec![(25.0, (all_buttons[0].rect.x + 60, all_buttons[0].rect.y + 15), "Install".to_string(), TEXT_COLOR), (25.0, (all_buttons[1].rect.x + 60, all_buttons[1].rect.y + 15), "Update".to_string(), TEXT_COLOR), (25.0, (all_buttons[2].rect.x + 55, all_buttons[2].rect.y + 15), "Unistall".to_string(), TEXT_COLOR)];

    let centered_image_pos = get_center((100, 100), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), "icons/hoyoumu_icon.bmp".to_string())];

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

    let centered_image_pos = get_center((100, 100), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), HOYOUMU_FILES[9].to_string())];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::AreYouSurePage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}
