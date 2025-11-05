use crate::{
    system::file_and_dirs::HOYOUMU_FILES,
    ui::style::{BACKGROUND_COLOR, BLUE_COLOR, GREEN_COLOR, RED_COLOR, TEXT_COLOR, YELLOW_COLOR}
};
use rust_page_system::{
    get_center,
    system::page_system::{Button, Page}
};
use sdl3::rect::Rect;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageId
{
    MainPage,
    AreYouSurePage
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
#[repr(usize)]
pub enum ButtonId
{
    Install,
    Update,
    Unistall,
    ConfirmUninstall,
    NoConfirmUninstall
}

pub fn main_page() -> Page<PageId, ButtonId>
{
    let centered_button_pos = get_center((200, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: BLUE_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y - 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Install, has_transition: None }, 
        Button { enabled: true, color: YELLOW_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Update, has_transition: None }, 
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 150, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::Unistall, has_transition: None }];

    //===================== texts =========================
    let all_text = vec!
    [
        (25.0, (all_buttons[0].rect.x + 60, all_buttons[0].rect.y + 15), "Install".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[1].rect.x + 60, all_buttons[1].rect.y + 15), "Update".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[2].rect.x + 55, all_buttons[2].rect.y + 15), "Unistall".to_string(), TEXT_COLOR)];

    let centered_image_pos = get_center((100, 100), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), HOYOUMU_FILES[7].to_string())];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::MainPage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}

pub fn are_you_sure_page() -> Page<PageId, ButtonId>
{
    let centered_button_pos = get_center((220, 60), (350, 450));
    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: RED_COLOR,   rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 50, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::ConfirmUninstall, has_transition: None },
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button_pos.pos_x, centered_button_pos.pos_y + 150, centered_button_pos.w, centered_button_pos.h), radius: 10, id: ButtonId::NoConfirmUninstall, has_transition: None }
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (13.0, (65, 120), "This Action Will Removed Everything \nRelated To HoyoPlay And This Project \nLike: (Games, LauncherData, LoginData, \nWinePrefix, Desktop Files, Icons, Etc...)\n \nAre You Sure You Want To Continue?".to_string(), TEXT_COLOR),
        (25.0, (all_buttons[0].rect.x + 33, all_buttons[0].rect.y + 15), "Yes, I'm Sure!".to_string(), TEXT_COLOR),
        (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 15), "No, Please Don't!".to_string(), TEXT_COLOR)
    ];

    let centered_image_pos = get_center((100, 100), (350, 450));
    let all_images = vec![((centered_image_pos.pos_x, 25), (centered_image_pos.w, centered_image_pos.h), HOYOUMU_FILES[9].to_string())];

    //===================== page creation =========================
    Page { has_userinput: None, has_persistent_elements: None, id: PageId::AreYouSurePage, background_color: Some(BACKGROUND_COLOR), rects: None, buttons: Some(all_buttons), texts: Some(all_text), images: Some(all_images) }
}
