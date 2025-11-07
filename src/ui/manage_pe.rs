use rust_page_system::{system::page_system::PageData, AppState};
use crate::{
    actions::buttons_actions::{
        ALL_DOWNLOAD_SUCCEEDED, DOWNLOADING_HOYOPLAY, DOWNLOADING_HOYOPLAY_SETUP, DOWNLOADING_OTHERS,
        DOWNLOADING_PROTON_GE, GITHUB_API_AVAILABLE, HOYOPLAY_DOWNLOAD_SUCCEEDED, LOADING,
        PROTON_DOWNLOAD_SUCCEEDED, PROTON_EXIST, PROTON_LATEST_EXIST, UMU_RUN_EXIST,
    },
    ui::pages::{
        already_installed_pe, download_not_succeed_pe, download_not_succeed_proton_pe, download_succeed,
        downloading_pe, github_api_unavailabe_pe, loading, umu_run_dont_exist, using_local_proton, ButtonId,
        PageId,
    },
};
use std::sync::atomic::Ordering;

pub fn manage_pe(page_data: &mut PageData<PageId, ButtonId>, app_state: &mut AppState<PageId, ButtonId>) {
    // Clear previous persistent elements and enable events
    page_data.forced_persistent_elements = None;
    app_state.all_events_disable = false;

    // Retrieve global state values safely
    let proton_download_state = *PROTON_DOWNLOAD_SUCCEEDED.lock().unwrap();
    if proton_download_state == Some((true, false)) {
        page_data.forced_persistent_elements = Some(vec![download_succeed(true)]);
    }
    if proton_download_state == Some((false, false)) {
        page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe(false)]);
    }
    if proton_download_state == Some((false, true)) {
        page_data.forced_persistent_elements = Some(vec![download_not_succeed_proton_pe(true)]);
    }

    let proton_exist_state = *PROTON_EXIST.lock().unwrap();
    if proton_exist_state == Some(true) {
        page_data.forced_persistent_elements = Some(vec![using_local_proton()]);
    }
    let proton_latest_exist_state = *PROTON_LATEST_EXIST.lock().unwrap();
    if proton_latest_exist_state == Some(true) {
        page_data.forced_persistent_elements = Some(vec![already_installed_pe(true)]);
    }
    let umu_run_exist_state = *UMU_RUN_EXIST.lock().unwrap();
    if umu_run_exist_state == Some(false) {
        page_data.forced_persistent_elements = Some(vec![umu_run_dont_exist()]);
    }

    let all_download_succeeded = *ALL_DOWNLOAD_SUCCEEDED.lock().unwrap();
    if all_download_succeeded == Some(true) {
        page_data.forced_persistent_elements = Some(vec![download_succeed(false)]);
    }
    if all_download_succeeded == Some(false) {
        page_data.forced_persistent_elements = Some(vec![download_not_succeed_pe()]);
    }

    let hoyoplay_download_succeeded = *HOYOPLAY_DOWNLOAD_SUCCEEDED.lock().unwrap();
    if hoyoplay_download_succeeded == Some(false) {
        page_data.forced_persistent_elements = Some(vec![download_not_succeed_pe()]);
    }

    let github_api_available = *GITHUB_API_AVAILABLE.lock().unwrap();
    if github_api_available == Some(false) {
        page_data.forced_persistent_elements = Some(vec![github_api_unavailabe_pe()]);
    }

    // Check downloading/loading flags
    if DOWNLOADING_OTHERS.load(Ordering::SeqCst) {
        page_data.forced_persistent_elements = Some(vec![downloading_pe(false, false, true)]);
        app_state.all_events_disable = true;
    }
    if DOWNLOADING_PROTON_GE.load(Ordering::SeqCst) {
        page_data.forced_persistent_elements = Some(vec![downloading_pe(false, true, false)]);
        app_state.all_events_disable = true;
    }
    if DOWNLOADING_HOYOPLAY.load(Ordering::SeqCst) {
        page_data.forced_persistent_elements = Some(vec![downloading_pe(true, false, false)]);
        app_state.all_events_disable = true;
    }
    if DOWNLOADING_HOYOPLAY_SETUP.load(Ordering::SeqCst) {
        page_data.forced_persistent_elements = Some(vec![downloading_pe(true, false, false)]);
        app_state.all_events_disable = true;
    }

    if LOADING.load(Ordering::SeqCst) {
        page_data.forced_persistent_elements = Some(vec![loading()]);
    }
}
