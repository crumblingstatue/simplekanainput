use std::collections::HashMap;

use serde::Deserialize;

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

pub fn segment(romaji: &str) -> Vec<&str> {
    let mut begin = 0;
    let mut cursor = 0;
    let bytes = romaji.as_bytes();
    let mut state = SegStatus::Init;
    let mut segs = Vec::new();
    while cursor < bytes.len() {
        match state {
            SegStatus::Init => match bytes[cursor] {
                b'[' => {
                    let s = &romaji[begin..cursor];
                    if !s.is_empty() {
                        segs.push(s);
                    }
                    state = SegStatus::InBracketed;
                    begin = cursor + 1;
                }
                b' ' | b'\n' => {
                    segs.push(&romaji[begin..cursor]);
                    begin = cursor;
                }
                _ if cursor == bytes.len() - 1 => {
                    segs.push(&romaji[begin..cursor + 1]);
                }
                _ => {}
            },
            SegStatus::InBracketed => {
                if let b']' = bytes[cursor] {
                    segs.push(&romaji[begin..cursor]);
                    begin = cursor + 1;
                    state = SegStatus::Init;
                }
            }
        }
        cursor += 1;
    }
    segs
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
pub fn to_japanese<'a>(segments: &'a [&'a str], intp: &IntpMap) -> String {
    let mut s = String::new();
    for (i, &seg) in segments.iter().enumerate() {
        let mut table = &HIRAGANA;
        if let Some(intp) = intp.get(&i) {
            match intp {
                Intp::AsIs => {
                    s.push_str(seg);
                    continue;
                }
                Intp::Hiragana => table = &HIRAGANA,
                Intp::Katakana => table = &KATAKANA,
                Intp::String(str) => {
                    s.push_str(str);
                    continue;
                }
            }
        }
        let dec = decompose(seg, table);
        s.push_str(&dec.to_kana_string());
    }
    s
}
