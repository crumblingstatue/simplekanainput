use {
    crate::kana::{RomajiKanaTable, HIRAGANA, KATAKANA},
    serde::Deserialize,
    std::collections::HashMap,
};

pub enum Intp {
    AsIs,
    Hiragana,
    Katakana,
    String(String),
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

enum SegStatus {
    Init,
    InBracketed,
}

#[derive(PartialEq, Debug)]
pub enum Segment<'s> {
    Simple(&'s str),
    /// A dictionary form text and extra (conjugation) text.
    ///
    /// ```plaintext
    /// [deau:tte]
    ///   ^    ^
    ///  dict  extra
    /// ```
    ///
    /// Used for conjugated words like adjectives/verbs.
    DictAndExtra {
        dict: &'s str,
        extra: &'s str,
        /// How many characters to cut off from dict lookup result (usually 1).
        ///
        /// Determined by number of `:` separators.
        cutoff: usize,
    },
}
impl<'a> Segment<'a> {
    pub(crate) fn label_string(&self) -> String {
        match self {
            Segment::Simple(s) => s.to_string(),
            Segment::DictAndExtra { dict, extra, .. } => {
                format!("{dict}[{extra}]")
            }
        }
    }
    /// Used for dictionary lookups
    pub(crate) fn dict_root(&self) -> &str {
        match self {
            Segment::Simple(s) => s,
            Segment::DictAndExtra { dict, .. } => dict,
        }
    }
}

pub fn segment(romaji: &str) -> Vec<Segment> {
    let mut begin = 0;
    let mut cursor = 0;
    let bytes = romaji.as_bytes();
    let mut status = SegStatus::Init;
    let mut segs = Vec::new();
    let mut colons = 0;
    let mut extra_begin = 0;
    while cursor < bytes.len() {
        match status {
            SegStatus::Init => match bytes[cursor] {
                b'[' => {
                    let s = &romaji[begin..cursor];
                    if !s.is_empty() {
                        if colons == 0 {
                            segs.push(Segment::Simple(s));
                        } else {
                            segs.push(Segment::DictAndExtra {
                                dict: &romaji[begin..extra_begin],
                                extra: &romaji[extra_begin + colons..cursor],
                                cutoff: colons,
                            })
                        }
                        colons = 0;
                    }
                    status = SegStatus::InBracketed;
                    begin = cursor + 1;
                }
                b' ' | b'\n' => {
                    if colons == 0 {
                        let txt = &romaji[begin..cursor];
                        if !txt.is_empty() {
                            segs.push(Segment::Simple(txt));
                            begin = cursor + 1;
                        }
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor],
                            cutoff: colons,
                        });
                        begin = cursor + 1;
                    }
                    colons = 0;
                }
                b':' => {
                    extra_begin = cursor - colons;
                    colons += 1;
                }
                _ if cursor == bytes.len() - 1 => {
                    if colons == 0 {
                        segs.push(Segment::Simple(&romaji[begin..cursor + 1]));
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor + 1],
                            cutoff: colons,
                        })
                    }
                    colons = 0;
                }
                _ => {}
            },
            SegStatus::InBracketed => match bytes[cursor] {
                b']' => {
                    if colons == 0 {
                        segs.push(Segment::Simple(&romaji[begin..cursor]));
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor],
                            cutoff: colons,
                        })
                    }
                    colons = 0;
                    begin = cursor + 1;
                    status = SegStatus::Init;
                }
                b':' => {
                    extra_begin = cursor - colons;
                    colons += 1;
                }
                _ => {}
            },
        }
        cursor += 1;
    }
    segs
}

#[test]
fn test_segment() {
    use Segment::*;
    assert_eq!(
        segment("[chiisai:nakute]a"),
        vec![
            DictAndExtra {
                dict: "chiisai",
                extra: "nakute",
                cutoff: 1
            },
            Simple("a")
        ]
    );
    assert_eq!(
        segment("watashi[ha]chiisai:kute[shizuka]janaide[omoshiroi]machi[ni]sumu:ndeimasu[.]"),
        vec![
            Simple("watashi"),
            Simple("ha"),
            DictAndExtra {
                dict: "chiisai",
                extra: "kute",
                cutoff: 1
            },
            Simple("shizuka"),
            Simple("janaide"),
            Simple("omoshiroi"),
            Simple("machi"),
            Simple("ni"),
            DictAndExtra {
                dict: "sumu",
                extra: "ndeimasu",
                cutoff: 1
            },
            Simple(".")
        ]
    );
    assert_eq!(segment("watashi ha"), vec![Simple("watashi"), Simple("ha")]);
    assert_eq!(
        segment("watashi  ha"),
        vec![Simple("watashi"), Simple(" ha")]
    );
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
            Intp::String(str) => {
                s.push_str(str);
            }
        }
    }
    s
}
