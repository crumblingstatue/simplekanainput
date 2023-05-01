use {
    crate::{conv::IntpMap, WinDims, WIN_DIMS},
    arboard::Clipboard,
};

pub struct AppState {
    pub intp: IntpMap,
    pub half_dims: WinDims,
    pub romaji_buf: String,
    pub clipboard: Clipboard,
    pub quit_requested: bool,
}

impl AppState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            intp: IntpMap::default(),
            half_dims: WIN_DIMS.half(),
            romaji_buf: String::new(),
            clipboard: Clipboard::new()?,
            quit_requested: false,
        })
    }
}
