use crate::ui::pages::{are_you_sure_page, main_page};
use rust_page_system::system::page_system::PageData;
use std::rc::Rc;



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
    RetryProton,
    ContinueProtonDownload,
    CancelProtonDownload
}



pub fn populate_page_data(page_data: &mut PageData<PageId, ButtonId>)
{
    page_data.populate_rps_data(Some(vec![Rc::new(main_page), Rc::new(are_you_sure_page)]), None);
}
