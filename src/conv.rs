use {serde::Deserialize, std::collections::HashMap};

pub enum Intp {
    AsIs,
    Hiragana,
    Katakana,
    String(String),
}

pub type IntpMap = HashMap<usize, Intp>;

struct RomajiKanaPair {
    romaji: &'static str,
    kana: &'static str,
}

pub struct RomajiKanaTable {
    table: &'static [RomajiKanaPair],
}

impl RomajiKanaTable {
    fn lookup(&self, romaji: &str) -> Option<&'static str> {
        self.table
            .iter()
            .find_map(|pair| (pair.romaji == romaji).then_some(pair.kana))
    }
}

macro_rules! kana_table {
    ($id:ident $($romaji:literal $kana:literal)+) => {
        pub static $id: RomajiKanaTable = RomajiKanaTable { table: &[
                $(
                RomajiKanaPair {
                        romaji: $romaji,
                        kana: $kana
                },
                )+
            ]
        };
    };
}

kana_table! {
    HIRAGANA
    "a" "あ"
    "_a" "ぁ"
    "i" "い"
    "u" "う"
    "e" "え"
    "o" "お"
    "ka" "か"
    "ki" "き"
    "kyu" "きゅ"
    "kyo" "きょ"
    "kya" "きゃ"
    "ku" "く"
    "ke" "け"
    "kke" "っけ"
    "kka" "っか"
    "kku" "っく"
    "kki" "っき"
    "ko" "こ"
    "kko" "っこ"
    "ga" "が"
    "gi" "ぎ"
    "gya" "ぎゃ"
    "gyo" "ぎょ"
    "gu" "ぐ"
    "ge" "げ"
    "go" "ご"
    "ra" "ら"
    "ri" "り"
    "ru" "る"
    "re" "れ"
    "ro" "ろ"
    "sa" "さ"
    "za" "ざ"
    "sshi" "っし"
    "shi" "し"
    "sha" "しゃ"
    "sho" "しょ"
    "shu" "しゅ"
    "ji" "じ"
    "ju" "じゅ"
    "jo" "じょ"
    "ja" "じゃ"
    "su" "す"
    "zu" "ず"
    "se" "せ"
    "ze" "ぜ"
    "so" "そ"
    "zo" "ぞ"
    "ta" "た"
    "tta" "った"
    "da" "だ"
    "cha" "ちゃ"
    "chi" "ち"
    "cho" "ちょ"
    "di" "ぢ"
    "tsu" "つ"
    "du" "づ"
    "te" "て"
    "tte" "って"
    "de" "で"
    "to" "と"
    "tto" "っと"
    "do" "ど"
    "na" "な"
    "ni" "に"
    "nu" "ぬ"
    "ne" "ね"
    "no" "の"
    "ha" "は"
    "ba" "ば"
    "pa" "ぱ"
    "hi" "ひ"
    "hya" "ひゃ"
    "bi" "び"
    "pi" "ぴ"
    "fu" "ふ"
    "bu" "ぶ"
    "pu" "ぷ"
    "he" "へ"
    "be" "べ"
    "pe" "ぺ"
    "ho" "ほ"
    "bo" "ぼ"
    "po" "ぽ"
    "ma" "ま"
    "mi" "み"
    "mu" "む"
    "me" "め"
    "mo" "も"
    "ya" "や"
    "yu" "ゆ"
    "yo" "よ"
    "wa" "わ"
    "wo" "を"
    "nn" "ん"
    "-"  "ー"
    "."  "。"
    ","  "、"
}

kana_table! {
    KATAKANA
    "a" "ア"
    "i" "イ"
    "u" "ウ"
    "e" "エ"
    "o" "オ"
    "ka" "カ"
    "ga" "ガ"
    "ki" "キ"
    "kyu" "キュ"
    "kyo" "キョ"
    "kya" "キャ"
    "gi" "ギ"
    "gya" "ギャ"
    "ku" "ク"
    "kku" "ック"
    "kki" "ッキ"
    "gu" "グ"
    "ke" "ケ"
    "ge" "ゲ"
    "ko" "コ"
    "go" "ゴ"
    "sa" "サ"
    "za" "ザ"
    "shi" "シ"
    "sshi" "ッシ"
    "sha" "シャ"
    "sho" "ショ"
    "shu" "シュ"
    "ji" "ジ"
    "ju" "ジュ"
    "ja" "ジャ"
    "jo" "ジョ"
    "su" "ス"
    "zu" "ズ"
    "se" "セ"
    "ze" "ゼ"
    "so" "ソ"
    "zo" "ゾ"
    "ta" "タ"
    "da" "ダ"
    "chi" "チ"
    "cha" "チャ"
    "di" "ヂ"
    "tsu" "ツ"
    "du" "ヅ"
    "te" "テ"
    "de" "デ"
    "to" "ト"
    "do" "ド"
    "na" "ナ"
    "ni" "ニ"
    "nu" "ヌ"
    "ne" "ネ"
    "no" "ノ"
    "ha" "ハ"
    "ba" "バ"
    "pa" "パ"
    "hi" "ヒ"
    "hya" "ヒャ"
    "bi" "ビ"
    "pi" "ピ"
    "fu" "フ"
    "bu" "ブ"
    "pu" "プ"
    "he" "ヘ"
    "be" "ベ"
    "pe" "ペ"
    "ho" "ホ"
    "bo" "ボ"
    "po" "ポ"
    "ma" "マ"
    "mi" "ミ"
    "mu" "ム"
    "me" "メ"
    "mo" "モ"
    "ya" "ヤ"
    "yu" "ユ"
    "yo" "ヨ"
    "ra" "ラ"
    "ri" "リ"
    "ru" "ル"
    "re" "レ"
    "ro" "ロ"
    "wa" "ワ"
    "wo" "ヲ"
    "nn" "ン"
    "-"  "ー"
    "."  "。"
    ","  "、"
}

// ぃぅぇぉゃゅゎゕゖ゛゜ゝゞゟ ゔ
// ゐ = wi (obsolete kana)
// ゑ = we (obsolete kana)

// ッィゥェォャュョヰヱヴヵヶヷヸヹヺ・ーヽヾヮ

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
                        segs.push(Segment::Simple(&romaji[begin..cursor]));
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor],
                            cutoff: colons,
                        })
                    }
                    begin = cursor;
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
    assert_eq!(
        segment("[chiisai:nakute]a"),
        vec![
            Segment::DictAndExtra {
                dict: "chiisai",
                extra: "nakute",
                cutoff: 1
            },
            Segment::Simple("a")
        ]
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
            let src_atom = &romaji[i..end];
            if let Some(kana) = table.lookup(src_atom) {
                elems.push(Element { atom: kana });
                skip = j - 1;
                found_kana = true;
                break;
            }
        }
        if !found_kana {
            let atom = &romaji[i..i + 1];
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
