use crate::{
    conv::{romaji_to_kana, IntpMap},
    kana::{HIRAGANA, KATAKANA},
    kanji::KanjiDb,
    segment::InputSpan,
    ui::{input::InputUiAction, DictUiState, KanjiUiState},
};
#[cfg(feature = "backend-sfml")]
use arboard::Clipboard;
#[cfg(feature = "ipc")]
use existing_instance::Listener;

pub struct AppState {
    pub intp: IntpMap,
    pub romaji_buf: String,
    #[cfg(feature = "backend-sfml")]
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
    /// Keeps track whether selected segment changed
    pub last_selected_segment: usize,
    pub cached_suggestions: CachedSuggestions,
    /// Selected dictionary suggestion (index into cache)
    pub selected_suggestion: Option<usize>,
    pub segments: Vec<InputSpan>,
    pub input_ui_action: Option<InputUiAction>,
    /// For some reason the egui memory fails me in getting the scroll offset, so we store it here
    /// Used for synchronizing output scroll and input (romaji) scroll
    pub out_scroll_last_offset: f32,
    #[cfg(feature = "ipc")]
    pub ipc_listener: Listener,
}

#[derive(Default)]
pub struct CachedSuggestions {
    pub jmdict: Vec<CachedJmdictSuggestion>,
}

pub struct CachedJmdictSuggestion {
    pub entry: jmdict::Entry,
    pub mugo_root: Option<mugo::Root>,
}

impl CachedSuggestions {
    fn clear(&mut self) {
        self.jmdict.clear();
    }
}

pub enum UiState {
    Input,
    Dict,
    Kanji,
    Help,
    About,
}

impl AppState {
    pub fn new(#[cfg(feature = "ipc")] ipc_listener: Listener) -> anyhow::Result<Self> {
        Ok(Self {
            intp: IntpMap::default(),
            romaji_buf: String::new(),
            #[cfg(feature = "backend-sfml")]
            clipboard: Clipboard::new()?,
            hide_requested: false,
            quit_requested: false,
            ui_state: UiState::Input,
            dict_ui_state: DictUiState::default(),
            kanji_ui_state: KanjiUiState::default(),
            selected_segment: 0,
            kanji_db: KanjiDb::load(),
            last_segs_len: 0,
            last_selected_segment: 0,
            cached_suggestions: CachedSuggestions::default(),
            selected_suggestion: None,
            segments: Vec::new(),
            input_ui_action: None,
            out_scroll_last_offset: 0.0,
            #[cfg(feature = "ipc")]
            ipc_listener,
        })
    }
    /// Populate the suggestion cache with entries for the selected segment
    pub(crate) fn repopulate_suggestion_cache(&mut self) {
        self.cached_suggestions.clear();
        let i = self.selected_segment;
        let Some(&InputSpan::RomajiWord { start, end }) = self.segments.get(i) else {
            return;
        };
        let hiragana = romaji_to_kana(&self.romaji_buf[start..end], &HIRAGANA);
        let hiragana = hiragana.trim();
        let katakana = romaji_to_kana(&self.romaji_buf[start..end], &KATAKANA);
        let katakana = katakana.trim();
        let root = mugo_jmdict::Root::Bare(hiragana);
        let mugo_roots = mugo::deconjugate(hiragana);
        self.cached_suggestions.jmdict = jmdict::entries()
            .filter_map(|en| {
                if root.matches(&en) {
                    return Some(CachedJmdictSuggestion {
                        entry: en,
                        mugo_root: None,
                    });
                } else {
                    for mugo_root in &mugo_roots {
                        if mugo_jmdict::Root::Conj(mugo_root).matches(&en) {
                            return Some(CachedJmdictSuggestion {
                                entry: en,
                                mugo_root: Some(mugo_root.clone()),
                            });
                        }
                    }
                }
                if mugo_jmdict::Root::Bare(katakana).reading_matches(&en) {
                    return Some(CachedJmdictSuggestion {
                        entry: en,
                        mugo_root: None,
                    });
                }
                None
            })
            .collect();
    }

    /// The SFML backend uses a more robust clipboard mechanism than what SFML offers (arboard),
    /// but it doesn't support wasm32, so we need diverging behavior here
    #[allow(unused_variables)]
    pub(crate) fn set_clipboard_text(&mut self, ctx: &crate::egui::Context, text: &str) {
        #[cfg(feature = "backend-sfml")]
        self.clipboard.set_text(text).unwrap();
        #[cfg(feature = "backend-eframe")]
        ctx.output_mut(|out| out.copied_text = text.to_owned());
    }
}
