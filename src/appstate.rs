use {
    crate::{
        conv::IntpMap,
        kanji::KanjiDb,
        ui::{DictUiState, KanjiUiState},
        WinDims, WIN_DIMS,
    },
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
    pub kanji_ui_state: KanjiUiState,
    pub selected_segment: usize,
    pub kanji_db: KanjiDb,
    /// Used to keep track of whether the text segmentation has changed
    pub last_segs_len: usize,
}

pub enum UiState {
    Input,
    Dict,
    Kanji,
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
            kanji_ui_state: KanjiUiState::default(),
            selected_segment: 0,
            kanji_db: KanjiDb::load(),
            last_segs_len: 0,
        })
    }
}
