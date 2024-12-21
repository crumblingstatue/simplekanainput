use {
    crate::{
        kana::{HIRAGANA, KATAKANA, RomajiKanaTable},
        kanji::KanjiDb,
        radicals::RadicalPair,
        segment::InputSpan,
    },
    std::collections::HashMap,
};

#[derive(Debug)]
pub enum Intp {
    Hiragana,
    Katakana,
    Dictionary {
        /// Index into cached suggestions.
        ///
        /// Not ideal that the entry and this index can potentially get desynced,
        /// but jmdict::Entry doesn't really provide a way to identify itself uniquely.
        cached_sug_idx: usize,
        en: jmdict::Entry,
        kanji_idx: usize,
        root: Option<mugo::Root>,
    },
    Kanji {
        db_idx: usize,
    },
    Radical(RadicalPair),
}

pub type IntpMap = HashMap<usize, Intp>;

/// Max possible length of a romaji kana "atom".
/// "sshi" is an example of a romaji kana atom, with length of 4.
const MAX_ROMAJI_ATOM_LEN: usize = 4;

pub fn romaji_to_kana(romaji: &str, table: &RomajiKanaTable) -> String {
    let mut out = String::new();
    let mut parser = RomajiParser::new(romaji);
    while let Some(str) = parser.next_largest_match(table) {
        out.push_str(str)
    }
    out
}

struct RomajiParser<'a> {
    cursor: usize,
    src: &'a str,
}

impl<'a> RomajiParser<'a> {
    fn new(src: &'a str) -> Self {
        Self { cursor: 0, src }
    }
    /// Attempts to find the largest romaji atom that matches.
    ///
    /// On match, returns a match from the kana table.
    /// If there are no matches, returns a single character from the source string.
    ///
    /// At string end, or non-unicode-boundary index, it returns None
    fn next_largest_match(&mut self, table: &RomajiKanaTable) -> Option<&'a str> {
        // The maximum end the match can reach (so it doesn't go out of bounds)
        let max_match_end = std::cmp::min(self.cursor + MAX_ROMAJI_ATOM_LEN, self.src.len());
        let mut match_len = std::cmp::min(MAX_ROMAJI_ATOM_LEN, max_match_end - self.cursor);
        while match_len > 0 {
            let end = self.cursor + match_len;
            if end > max_match_end {
                break;
            }
            let atom = self.src.get(self.cursor..end)?;
            match table.lookup(atom) {
                Some(kana) => {
                    self.cursor += match_len;
                    return Some(kana);
                }
                None => {
                    match_len -= 1;
                }
            }
        }
        let ret = self.src.get(self.cursor..self.cursor + 1);
        self.cursor += 1;
        ret
    }
}

#[test]
fn test_find_largest_match() {
    let mut parser = RomajiParser::new("...nani?");
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("…"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("な"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("に"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("？"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), None);
    parser = RomajiParser::new("sonna...");
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("そ"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("ん"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("な"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("…"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), None);
    parser = RomajiParser::new("konnichiha...");
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("こ"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("ん"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("に"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("ち"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("は"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("…"));
    assert_eq!(parser.next_largest_match(&HIRAGANA), None);
}

/// Precondition: If a text atom has an intp, it must have a kanji reading
pub fn with_input_span_converted_form(
    span: &InputSpan,
    i: usize,
    text: &str,
    intp: &IntpMap,
    kanji_db: &KanjiDb,
    mut f: impl FnMut(&str),
) {
    let romaji = match *span {
        InputSpan::RomajiWord { start, end } | InputSpan::RomajiPunct { start, end } => {
            &text[start..end]
        }
        InputSpan::Other { start, end } => {
            // We don't want to touch non-romaji segments at all
            f(&text[start..end]);
            return;
        }
    };
    let intp = intp.get(&i).unwrap_or(&Intp::Hiragana);
    match intp {
        Intp::Hiragana => {
            f(&romaji_to_kana(romaji, &HIRAGANA));
        }
        Intp::Katakana => {
            f(&romaji_to_kana(romaji, &KATAKANA));
        }
        Intp::Dictionary {
            cached_sug_idx: _,
            en,
            kanji_idx,
            root,
        } => {
            let mut kanji_string = en
                .kanji_elements()
                .nth(*kanji_idx)
                .unwrap()
                .text
                .to_string();
            if let Some(root) = root {
                if let Some(pos) = kanji_string.rfind(root.dict_suffix()) {
                    kanji_string.truncate(pos);
                }
                kanji_string.push_str(&root.conjugation_suffix());
            }
            f(&kanji_string);
        }
        Intp::Radical(pair) => {
            f(&pair.ch.to_string());
        }
        Intp::Kanji { db_idx } => f(kanji_db.kanji[*db_idx].chars[0]),
    }
}

pub fn to_japanese(
    text: &str,
    segments: &[InputSpan],
    intp: &IntpMap,
    kanji_db: &KanjiDb,
) -> String {
    let mut s = String::new();
    for (i, span) in segments.iter().enumerate() {
        with_input_span_converted_form(span, i, text, intp, kanji_db, |conv| {
            s.push_str(conv);
        })
    }
    s
}

#[test]
fn test_decompose() {
    assert_eq!(romaji_to_kana("sugoi", &HIRAGANA), "すごい");
}
