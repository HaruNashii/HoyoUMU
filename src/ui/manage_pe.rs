use rust_page_system::{system::page_system::PageData, AppState};
use crate::{actions::buttons_actions::{ALL_DOWNLOAD_SUCCEEDED, DOWNLOADING_HOYOPLAY, DOWNLOADING_HOYOPLAY_SETUP, DOWNLOADING_OTHERS, DOWNLOADING_PROTON_GE, GITHUB_API_AVAILABLE, HOYOPLAY_DOWNLOAD_SUCCEEDED, LOADING, PROTON_DOWNLOAD_SUCCEEDED, PROTON_LATEST_EXIST}, ui::pages::{already_installed_pe, download_not_succeed_pe, download_not_succeed_proton_pe, download_succeed, downloading_pe, github_api_unavailabe_pe, loading, ButtonId, PageId}};

pub fn manage_pe(page_data: &mut PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>)
{
        unsafe 
        {
            page_data.forced_persistent_elements = None;
            app_state.all_events_disable = false;

            if PROTON_DOWNLOAD_SUCCEEDED == Some(true) { page_data.forced_persistent_elements = Some(vec![download_succeed(true)]); };
            if PROTON_DOWNLOAD_SUCCEEDED == Some(false) { page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe()]); };


            if PROTON_LATEST_EXIST == Some(true) { page_data.forced_persistent_elements = Some(vec![already_installed_pe(true)]) };


            if ALL_DOWNLOAD_SUCCEEDED == Some(true) { page_data.forced_persistent_elements = Some(vec![download_succeed(false)]); };
            if ALL_DOWNLOAD_SUCCEEDED == Some(false) { page_data.forced_persistent_elements = Some(vec![download_not_succeed_pe()]); };


            if HOYOPLAY_DOWNLOAD_SUCCEEDED == Some(false) { page_data.forced_persistent_elements = Some(vec![download_not_succeed_pe()]); };


            if GITHUB_API_AVAILABLE == Some(false) { page_data.forced_persistent_elements = Some(vec![github_api_unavailabe_pe()]); };


            if DOWNLOADING_OTHERS { page_data.forced_persistent_elements = Some(vec![downloading_pe(false)]);  app_state.all_events_disable = true; };
            if DOWNLOADING_PROTON_GE { page_data.forced_persistent_elements = Some(vec![downloading_pe(true)]); app_state.all_events_disable = true; };
            if DOWNLOADING_HOYOPLAY { page_data.forced_persistent_elements = Some(vec![downloading_pe(false)]); app_state.all_events_disable = true; };
            if DOWNLOADING_HOYOPLAY_SETUP { page_data.forced_persistent_elements = Some(vec![downloading_pe(false)]); app_state.all_events_disable = true;};


            if LOADING { page_data.forced_persistent_elements = Some(vec![loading()]); };
        };
}
