use {
    crate::{
        kana::{RomajiKanaTable, HIRAGANA, KATAKANA},
        segment::Segment,
    },
    serde::Deserialize,
    std::collections::HashMap,
};

pub enum Intp {
    AsIs,
    Hiragana,
    Katakana,
    Dictionary { en: jmdict::Entry, kanji_idx: usize },
}

pub type IntpMap = HashMap<usize, Intp>;

#[derive(Debug)]
pub struct Element<'a> {
    pub atom: &'a str,
}

// Based on character count, not byte count (same as egui selection)
#[derive(Debug)]
pub struct CharSpan {
    pub begin: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct DecomposeResult<'a> {
    pub elems: Vec<Element<'a>>,
}

impl<'a> DecomposeResult<'a> {
    pub fn to_kana_string(&self) -> String {
        let mut out = String::new();
        for elem in &self.elems {
            out.push_str(elem.atom);
        }
        out
    }
}

#[derive(Deserialize, Debug)]
pub struct DictEntry {
    pub romaji: String,
    pub kanji: String,
    pub desc: String,
}

/// Max possible length of a romaji kana "atom".
/// "sshi" is an example of a romaji kana atom, with length of 4.
const MAX_ROMAJI_ATOM_LEN: usize = 4;

pub fn decompose<'a>(romaji: &'a str, table: &RomajiKanaTable) -> DecomposeResult<'a> {
    let mut elems = Vec::new();
    let mut skip = 0;
    for (i, _c) in romaji.char_indices() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        let mut found_kana = false;
        for (j, end) in (i..=(i + MAX_ROMAJI_ATOM_LEN).min(romaji.len())).enumerate() {
            let Some(src_atom) = romaji.get(i..end) else {
                continue
            };
            if let Some(kana) = table.lookup(src_atom) {
                elems.push(Element { atom: kana });
                skip = j - 1;
                found_kana = true;
                break;
            }
        }
        if !found_kana {
            let &Some(atom) = &romaji.get(i..i + 1) else {
                continue
            };
            elems.push(Element {
                atom: if atom == "n" {
                    table.lookup("nn").unwrap()
                } else {
                    atom
                },
            });
        }
    }
    DecomposeResult { elems }
}
pub fn to_japanese<'a>(segments: &'a [Segment<'a>], intp: &IntpMap) -> String {
    let mut s = String::new();
    for (i, seg) in segments.iter().enumerate() {
        let intp = intp.get(&i).unwrap_or(&Intp::Hiragana);
        match intp {
            Intp::AsIs => match seg {
                Segment::Simple(text) => s.push_str(text),
                Segment::DictAndExtra { .. } => s.push_str("<non-applicable, sorry>"),
            },
            Intp::Hiragana => {
                let dec = decompose(seg.dict_root(), &HIRAGANA);
                s.push_str(&dec.to_kana_string());
            }
            Intp::Katakana => {
                let dec = decompose(seg.dict_root(), &KATAKANA);
                s.push_str(&dec.to_kana_string());
            }
            Intp::Dictionary { en, kanji_idx } => {
                let kanji_str = en.kanji_elements().nth(*kanji_idx).unwrap().text;
                match seg {
                    Segment::Simple(_) => {
                        s.push_str(kanji_str);
                    }
                    Segment::DictAndExtra {
                        dict: _,
                        extra,
                        cutoff,
                    } => {
                        let mut kan_owned = kanji_str.to_owned();
                        for _ in 0..*cutoff {
                            kan_owned.pop();
                        }
                        s.push_str(&kan_owned);
                        s.push_str(&decompose(extra, &HIRAGANA).to_kana_string());
                    }
                }
            }
        }
    }
    s
}
