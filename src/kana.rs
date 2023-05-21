struct RomajiKanaPair {
    romaji: &'static str,
    kana: &'static str,
}

pub struct RomajiKanaTable {
    table: &'static [RomajiKanaPair],
}

impl RomajiKanaTable {
    pub fn lookup(&self, romaji: &str) -> Option<&'static str> {
        self.table
            .iter()
            .find_map(|pair| (pair.romaji == romaji).then_some(pair.kana))
    }
}

macro_rules! kana_table {
    ($($romaji:literal $hiragana:literal $katakana:literal)+) => {
        pub static HIRAGANA: RomajiKanaTable = RomajiKanaTable { table: &[
                $(
                RomajiKanaPair {
                        romaji: $romaji,
                        kana: $hiragana
                },
                )+
            ]
        };
        pub static KATAKANA: RomajiKanaTable = RomajiKanaTable { table: &[
            $(
            RomajiKanaPair {
                    romaji: $romaji,
                    kana: $katakana
            },
            )+
        ]
    };
    };
}

kana_table! {
    "a" "あ" "ア"
    "i" "い" "イ"
    "u" "う" "ウ"
    "e" "え" "エ"
    "o" "お" "オ"
    "ka" "か" "カ"
    "ki" "き" "キ"
    "kyu" "きゅ" "キュ"
    "kyo" "きょ" "キョ"
    "kya" "きゃ" "キャ"
    "ku" "く" "ク"
    "ke" "け" "ケ"
    "kke" "っけ" "ッケ"
    "kka" "っか" "ッカ"
    "kku" "っく" "ック"
    "kki" "っき" "ッキ"
    "ko" "こ" "コ"
    "kko" "っこ" "ッコ"
    "ga" "が" "ガ"
    "gi" "ぎ" "ギ"
    "gya" "ぎゃ" "ギャ"
    "gyo" "ぎょ" "ギョ"
    "gyu" "ぎゅ" "ギュ"
    "gu" "ぐ" "グ"
    "ge" "げ" "ゲ"
    "go" "ご" "ゴ"
    "ra" "ら" "ラ"
    "ri" "り" "リ"
    "rya" "りゃ" "リャ"
    "ryo" "りょ" "リョ"
    "ryu" "りゅ" "リュ"
    "ru" "る" "ル"
    "re" "れ" "レ"
    "ro" "ろ" "ロ"
    "sa" "さ" "サ"
    "za" "ざ" "ザ"
    "sshi" "っし" "ッシ"
    "shi" "し" "シ"
    "sha" "しゃ" "シャ"
    "sho" "しょ" "ショ"
    "shu" "しゅ" "シュ"
    "ji" "じ" "ジ"
    "ju" "じゅ" "ジュ"
    "jo" "じょ" "ジョ"
    "ja" "じゃ" "ジャ"
    "su" "す" "ス"
    "zu" "ず" "ズ"
    "se" "せ" "セ"
    "ze" "ぜ" "ゼ"
    "so" "そ" "ソ"
    "sso" "っそ" "ッソ"
    "zo" "ぞ" "ゾ"
    "ta" "た" "タ"
    "tta" "った" "ッタ"
    "da" "だ" "ダ"
    "chi" "ち" "チ"
    "cchi" "っち" "ッチ"
    "cha" "ちゃ" "チャ"
    "ccha" "っちゃ" "ッチャ"
    "cho" "ちょ" "チョ"
    "chu" "ちゅ" "チュ"
    "di" "ぢ" "ヂ"
    "tsu" "つ" "ツ"
    "du" "づ" "ヅ"
    "te" "て" "テ"
    "tte" "って" "テッ"
    "de" "で" "デ"
    "to" "と" "ト"
    "tto" "っと" "ット"
    "do" "ど" "ド"
    "na" "な" "ナ"
    "ni" "に" "ニ"
    "nya" "にゃ" "ニャ"
    "nyo" "にょ" "ニョ"
    "nyu" "にょ" "ニュ"
    "nu" "ぬ" "ヌ"
    "ne" "ね" "ネ"
    "no" "の" "ノ"
    "ha" "は" "ハ"
    "ba" "ば" "バ"
    "pa" "ぱ" "パ"
    "ppa" "っぱ" "ッパ"
    "hi" "ひ" "ヒ"
    "hya" "ひゃ" "ヒャ"
    "bi" "び" "ビ"
    "pi" "ぴ" "ピ"
    "ppi" "っぴ" "ッピ"
    "fu" "ふ" "フ"
    "bu" "ぶ" "ブ"
    "pu" "ぷ" "プ"
    "ppu" "っぷ" "ップ"
    "he" "へ" "ヘ"
    "be" "べ" "ベ"
    "pe" "ぺ" "ペ"
    "ho" "ほ" "ホ"
    "bo" "ぼ" "ボ"
    "po" "ぽ" "ポ"
    "ppo" "っぽ" "ッポ"
    "ma" "ま" "マ"
    "mi" "み" "ミ"
    "mu" "む" "ム"
    "me" "め" "メ"
    "mo" "も" "モ"
    "ya" "や" "ヤ"
    "yu" "ゆ" "ユ"
    "yo" "よ" "ヨ"
    "wa" "わ" "ワ"
    "wo" "を" "ヲ"
    "nn" "ん" "ン"
    "-"  "ー" "ー"
    "."  "。" "。"
    ","  "、" "、"
    "!"  "！" "！"
}

// ぃぅぇぉゃゅゎゕゖ゛゜ゝゞゟ ゔ
// ゐ = wi (obsolete kana)
// ゑ = we (obsolete kana)

// ッィゥェォャュョヰヱヴヵヶヷヸヹヺ・ーヽヾヮ
