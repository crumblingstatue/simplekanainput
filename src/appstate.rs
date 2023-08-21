use {
    crate::{conv::IntpMap, ui::DictUiState, WinDims, WIN_DIMS},
    arboard::Clipboard,
};

pub struct AppState {
    pub intp: IntpMap,
    pub half_dims: WinDims,
    pub romaji_buf: String,
    pub clipboard: Clipboard,
    pub hide_requested: bool,
    pub quit_requested: bool,
    pub ui_state: UiState,
    pub dict_ui_state: DictUiState,
}

pub enum UiState {
    Input,
    Dict,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            intp: IntpMap::default(),
            half_dims: WIN_DIMS.half(),
            romaji_buf: String::new(),
            clipboard: Clipboard::new()?,
            hide_requested: false,
            quit_requested: false,
            ui_state: UiState::Input,
            dict_ui_state: DictUiState::default(),
        })
    }
}
