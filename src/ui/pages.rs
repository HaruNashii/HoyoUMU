use crate::{system::setup_rps::{PageId, ButtonId}, ui::style::{BACKGROUND_COLOR, BLUE_COLOR, GREEN_COLOR, RED_COLOR, TEXT_COLOR, YELLOW_COLOR}};
use rust_page_system::{get_center, system::page_system::{Button, Page}};
use sdl3::rect::Rect;



pub fn main_page() -> Page<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_button = get_center((200, 60), (350, 450));
    let centered_image = get_center((100, 150), (350, 450));

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: BLUE_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y - 50, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Install, has_transition: None }, 
        Button { enabled: true, color: YELLOW_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 50, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Update, has_transition: None }, 
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 150, centered_button.w, centered_button.h), radius: 10, id: ButtonId::Unistall, has_transition: None }
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (25.0, (all_buttons[0].rect.x + 60, all_buttons[0].rect.y + 15), "Install".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[1].rect.x + 60, all_buttons[1].rect.y + 15), "Update".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[2].rect.x + 50, all_buttons[2].rect.y + 15), "Uninstall".to_string(), TEXT_COLOR)
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, -5), (centered_image.w, centered_image.h), "icons/hutao.bmp".to_string())
    ];

    //===================== page creation =========================
    Page 
    { 
        has_userinput: None,
        has_persistent_elements: None,
        id: PageId::MainPage,
        background_color: Some(BACKGROUND_COLOR), 
        rects: None,
        buttons: Some(all_buttons),
        texts: Some(all_text),
        images: Some(all_images) 
    }
}

pub fn are_you_sure_page() -> Page<PageId, ButtonId>
{
    //===================== variables =========================
    let centered_button = get_center((220, 60), (350, 450));
    let centered_image = get_center((90, 140), (350, 450));

    //===================== buttons =========================
    let all_buttons = vec!
    [
        Button { enabled: true, color: RED_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 50, centered_button.w, centered_button.h), radius: 10, id: ButtonId::ConfirmUninstall, has_transition: None },
        Button { enabled: true, color: GREEN_COLOR, rect: Rect::new(centered_button.pos_x, centered_button.pos_y + 150, centered_button.w, centered_button.h), radius: 10, id: ButtonId::NoConfirmUninstall, has_transition: None }
    ];

    //===================== texts =========================
    let all_text = vec!
    [
        (13.0, (65, 120), "This Action Will Remove Everything \nRelated To HoyoPlay And This Project \nLike: (Games, LauncherData, LoginData, \nWinePrefix, Desktop Files, Icons, Etc...)\n \nAre You Sure You Want To Continue?".to_string(), TEXT_COLOR), 
        (25.0, (all_buttons[0].rect.x + 33, all_buttons[0].rect.y + 15), "Yes, I'm Sure!".to_string(), TEXT_COLOR), (25.0, (all_buttons[1].rect.x + 10, all_buttons[1].rect.y + 15), "No, Please Don't!".to_string(), TEXT_COLOR)
    ];

    //===================== images =========================
    let all_images = vec!
    [
        ((centered_image.pos_x, -10), (centered_image.w, centered_image.h), "icons/hutao_colapse.bmp".to_string())
    ];

    //===================== page creation =========================
    Page 
    { 
        has_userinput: None, 
        has_persistent_elements: None, 
        id: PageId::AreYouSurePage, 
        background_color: Some(BACKGROUND_COLOR), 
        rects: None, 
        buttons: Some(all_buttons), 
        texts: Some(all_text), 
        images: Some(all_images) 
    }
}
