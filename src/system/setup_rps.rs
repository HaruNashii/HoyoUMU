use crate::ui::pages::{ButtonId, PageId, are_you_sure_page, main_page};
use rust_page_system::system::page_system::PageData;
use std::rc::Rc;

pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.push_page_link(Some(vec![(PageId::MainPage, Rc::new(main_page)), (PageId::AreYouSurePage, Rc::new(are_you_sure_page))]), None);
}
