use {
    crate::{
        kana::{RomajiKanaTable, HIRAGANA, KATAKANA},
        kanji::KanjiDb,
        radicals::RadicalPair,
        segment::Span,
    },
    mugo::RootKind,
    serde::Deserialize,
    std::collections::HashMap,
};

pub enum Intp {
    AsIs,
    Hiragana,
    Katakana,
    Dictionary {
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
    let mut skip = 0;
    for i in 0..romaji.len() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let mut found_kana = false;
        for (j, end) in (i..=(i + MAX_ROMAJI_ATOM_LEN).min(romaji.len())).enumerate() {
            let Some(src_atom) = romaji.get(i..end) else {
                continue;
            };
            if let Some(kana) = table.lookup(src_atom) {
                out.push_str(kana);
                skip = j - 1;
                found_kana = true;
                break;
            }
        }
        if !found_kana {
            let &Some(atom) = &romaji.get(i..i + 1) else {
                continue;
            };
            out.push_str(if atom == "n" {
                table.lookup("nn").unwrap()
            } else {
                atom
            });
        }
    }
    out
}
pub fn to_japanese(text: &str, segments: &[Span], intp: &IntpMap, kanji_db: &KanjiDb) -> String {
    let mut s = String::new();
    for (i, span) in segments.iter().enumerate() {
        let seg = span.index(text);
        let intp = intp.get(&i).unwrap_or(&Intp::Hiragana);
        match intp {
            Intp::AsIs => s.push_str(seg),
            Intp::Hiragana => {
                s.push_str(&romaji_to_kana(seg, &HIRAGANA));
            }
            Intp::Katakana => {
                s.push_str(&romaji_to_kana(seg, &KATAKANA));
            }
            Intp::Dictionary {
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
