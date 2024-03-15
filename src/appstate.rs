use {
    crate::{
        conv::{romaji_to_kana, IntpMap},
        kana::HIRAGANA,
        kanji::KanjiDb,
        segment::InputSpan,
        ui::{input::InputUiAction, DictUiState, KanjiUiState},
        WinDims, WIN_DIMS,
    },
    arboard::Clipboard,
    mugo::RootKind,
    std::borrow::Cow,
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
    /// Keeps track whether selected segment changed
    pub last_selected_segment: usize,
    pub cached_suggestions: CachedSuggestions,
    /// Selected dictionary suggestion (index into cache)
    pub selected_suggestion: Option<usize>,
    pub segments: Vec<InputSpan>,
    pub input_ui_action: Option<InputUiAction>,
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
            last_selected_segment: 0,
            cached_suggestions: CachedSuggestions::default(),
            selected_suggestion: None,
            segments: Vec::new(),
            input_ui_action: None,
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
        let root = Root::Bare(hiragana);
        let mugo_roots: Vec<mugo::Root> = mugo::deconjugate(hiragana).into_iter().collect();
        self.cached_suggestions.jmdict = jmdict::entries()
            .filter_map(|en| {
                // Filter out entries with no kanji elements
                if en.kanji_elements().len() == 0 {
                    return None;
                }
                if root.matches(&en) {
                    return Some(CachedJmdictSuggestion {
                        entry: en,
                        mugo_root: None,
                    });
                } else {
                    for mugo_root in &mugo_roots {
                        if Root::Conj(mugo_root.clone()).matches(&en) {
                            return Some(CachedJmdictSuggestion {
                                entry: en,
                                mugo_root: Some(mugo_root.clone()),
                            });
                        }
                    }
                }
                None
            })
            .collect();
    }
}

pub enum Root<'a> {
    Bare(&'a str),
    Conj(mugo::Root),
}

impl<'a> Root<'a> {
    fn dict_text(&self) -> Cow<str> {
        match self {
            Root::Bare(s) => Cow::Borrowed(s),
            Root::Conj(root) => Cow::Owned(root.dict()),
        }
    }

    pub fn matches(&self, e: &jmdict::Entry) -> bool {
        match self {
            Root::Bare(_) => self.reading_matches(e),
            Root::Conj(root) => {
                root_kind_matches(&root.kind, e.senses()) && self.reading_matches(e)
            }
        }
    }

    fn reading_matches(&self, e: &jmdict::Entry) -> bool {
        e.reading_elements().any(|e| e.text == self.dict_text())
    }
}

fn root_kind_matches(kind: &mugo::RootKind, mut senses: jmdict::Senses) -> bool {
    senses.any(|sense| {
        sense
            .parts_of_speech()
            .any(|part| part == kind.to_jmdict_part_of_speech())
    })
}

pub trait RootKindExt {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech;
}

impl RootKindExt for RootKind {
    fn to_jmdict_part_of_speech(&self) -> jmdict::PartOfSpeech {
        use jmdict::PartOfSpeech as Part;
        match self {
            RootKind::Ichidan => Part::IchidanVerb,
            RootKind::GodanBu => Part::GodanBuVerb,
            RootKind::GodanMu => Part::GodanMuVerb,
            RootKind::GodanNu => Part::GodanNuVerb,
            RootKind::GodanRu => Part::GodanRuVerb,
            RootKind::GodanSu => Part::GodanSuVerb,
            RootKind::GodanTsu => Part::GodanTsuVerb,
            RootKind::GodanU => Part::GodanUVerb,
            RootKind::GodanGu => Part::GodanGuVerb,
            RootKind::GodanKu => Part::GodanKuVerb,
            RootKind::IAdjective => Part::Adjective,
            RootKind::Iku => Part::GodanIkuVerb,
            RootKind::Kuru => Part::KuruVerb,
            RootKind::NaAdjective => Part::AdjectivalNoun,
            RootKind::Suru => Part::SuruVerb,
            RootKind::SpecialSuru => Part::SpecialSuruVerb,
        }
    }
}
