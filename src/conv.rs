use {
    crate::{
        kana::{RomajiKanaTable, HIRAGANA, KATAKANA},
        kanji::KanjiDb,
        radicals::RadicalPair,
        segment::InputSpan,
    },
    mugo::RootKind,
    serde::Deserialize,
    std::collections::HashMap,
};

#[derive(Debug)]
pub enum Intp {
    AsIs,
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

#[derive(Deserialize, Debug)]
pub struct DictEntry {
    pub romaji: String,
    pub kanji: String,
    pub desc: String,
}

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
    /// At string end, returns None
    fn next_largest_match(&mut self, table: &RomajiKanaTable) -> Option<&'a str> {
        // The maximum end the match can reach (so it doesn't go out of bounds)
        let max_match_end = std::cmp::min(self.cursor + MAX_ROMAJI_ATOM_LEN, self.src.len());
        let mut match_len = std::cmp::min(MAX_ROMAJI_ATOM_LEN, max_match_end - self.cursor);
        while match_len > 0 {
            let end = self.cursor + match_len;
            if end > max_match_end {
                break;
            }
            match table.lookup(&self.src[self.cursor..end]) {
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
    assert_eq!(parser.next_largest_match(&HIRAGANA), Some("?"));
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

pub fn to_japanese(
    text: &str,
    segments: &[InputSpan],
    intp: &IntpMap,
    kanji_db: &KanjiDb,
) -> String {
    let mut s = String::new();
    for (i, span) in segments.iter().enumerate() {
        let romaji = match *span {
            InputSpan::Romaji { start, end } => &text[start..end],
            InputSpan::Other { start, end } => {
                // We don't want to touch non-romaji segments at all
                s.push_str(&text[start..end]);
                continue;
            }
        };
        let intp = intp.get(&i).unwrap_or(&Intp::Hiragana);
        match intp {
            Intp::AsIs => s.push_str(romaji),
            Intp::Hiragana => {
                s.push_str(&romaji_to_kana(romaji, &HIRAGANA));
            }
            Intp::Katakana => {
                s.push_str(&romaji_to_kana(romaji, &KATAKANA));
            }
            Intp::Dictionary {
                cached_sug_idx: _,
                en,
                kanji_idx,
                root,
            } => {
                let kanji_str = en.kanji_elements().nth(*kanji_idx).unwrap().text;
                s.push_str(kanji_str);
                if let Some(root) = root {
                    // We want to pop the dictionary root for verbs/i adjectives
                    // but not for na adjectives (and maybe more?)
                    if !matches!(root.kind, RootKind::NaAdjective) {
                        s.pop();
                    }
                    // Need to pop an extra character for suru verbs
                    if matches!(root.kind, RootKind::Suru | RootKind::SpecialSuru) {
                        s.pop();
                    }
                    s.push_str(&root.conjugation_suffix());
                }
            }
            Intp::Radical(pair) => {
                s.push(pair.ch);
            }
            Intp::Kanji { db_idx } => s.push_str(kanji_db.kanji[*db_idx].chars[0]),
        }
    }
    s
}

#[test]
fn test_decompose() {
    assert_eq!(romaji_to_kana("sugoi", &HIRAGANA), "すごい");
}
