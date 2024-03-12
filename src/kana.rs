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
    "-"  "ー" "ー"
    ","  "、" "、"
    "!"  "！" "！"
    "..." "…" "…"
    "."  "。" "。"
    "a" "あ" "ア"
    "ba" "ば" "バ"
    "be" "べ" "ベ"
    "bi" "び" "ビ"
    "bo" "ぼ" "ボ"
    "bu" "ぶ" "ブ"
    "bya" "びゃ" "ビャ"
    "byo" "びょ" "ビョ"
    "byu" "びゅ" "ビュ"
    "ccha" "っちゃ" "ッチャ"
    "cchi" "っち" "ッチ"
    "cha" "ちゃ" "チャ"
    "che" "ちぇ" "チェ"
    "chi" "ち" "チ"
    "cho" "ちょ" "チョ"
    "chu" "ちゅ" "チュ"
    "da" "だ" "ダ"
    "ddo" "っど" "ッド"
    "de" "で" "デ"
    "di" "ぢ" "ヂ"
    "do" "ど" "ド"
    "du" "づ" "ヅ"
    "dzu" "づ" "ヅ"
    "e" "え" "エ"
    "fi" "ふぃ" "フィ"
    "fu" "ふ" "フ"
    "ga" "が" "ガ"
    "ge" "げ" "ゲ"
    "gi" "ぎ" "ギ"
    "go" "ご" "ゴ"
    "gu" "ぐ" "グ"
    "gya" "ぎゃ" "ギャ"
    "gyo" "ぎょ" "ギョ"
    "gyu" "ぎゅ" "ギュ"
    "ha" "は" "ハ"
    "he" "へ" "ヘ"
    "hi" "ひ" "ヒ"
    "ho" "ほ" "ホ"
    "hya" "ひゃ" "ヒャ"
    "hyo" "ひょ" "ヒョ"
    "hyu" "ひゅ" "ヒュ"
    "i" "い" "イ"
    "ja" "じゃ" "ジャ"
    "ji" "じ" "ジ"
    "je" "じぇ" "ジェ"
    "jo" "じょ" "ジョ"
    "ju" "じゅ" "ジュ"
    "ka" "か" "カ"
    "ke" "け" "ケ"
    "ki" "き" "キ"
    "kka" "っか" "ッカ"
    "kke" "っけ" "ッケ"
    "kki" "っき" "ッキ"
    "kko" "っこ" "ッコ"
    "kku" "っく" "ック"
    "kkya" "っきゃ" "ッキャ"
    "kkyo" "っきょ" "ッキョ"
    "kkyu" "っきゅ" "ッキュ"
    "ko" "こ" "コ"
    "ku" "く" "ク"
    "kya" "きゃ" "キャ"
    "kyo" "きょ" "キョ"
    "kyu" "きゅ" "キュ"
    "ma" "ま" "マ"
    "me" "め" "メ"
    "mi" "み" "ミ"
    "mma" "っま" "ッマ"
    "mme" "っめ" "ッメ"
    "mmi" "っみ" "ッミ"
    "mmo" "っも" "ッモ"
    "mmu" "っむ" "ッム"
    "mo" "も" "モ"
    "mu" "む" "ム"
    "mya" "みゃ" "ミャ"
    "myo" "みょ" "ミョ"
    "myu" "みゅ" "ミュ"
    "na" "な" "ナ"
    "ne" "ね" "ネ"
    "ni" "に" "ニ"
    "n" "ん" "ン"
    "no" "の" "ノ"
    "nu" "ぬ" "ヌ"
    "nya" "にゃ" "ニャ"
    "nyo" "にょ" "ニョ"
    "nyu" "にゅ" "ニュ"
    "o" "お" "オ"
    "pa" "ぱ" "パ"
    "pe" "ぺ" "ペ"
    "pi" "ぴ" "ピ"
    "po" "ぽ" "ポ"
    "ppa" "っぱ" "ッパ"
    "ppi" "っぴ" "ッピ"
    "ppo" "っぽ" "ッポ"
    "ppu" "っぷ" "ップ"
    "pu" "ぷ" "プ"
    "pyo" "ぴょ" "ピョ"
    "pyu" "ぴゅ" "ピュ"
    "ra" "ら" "ラ"
    "re" "れ" "レ"
    "ri" "り" "リ"
    "ro" "ろ" "ロ"
    "ru" "る" "ル"
    "rya" "りゃ" "リャ"
    "ryo" "りょ" "リョ"
    "ryu" "りゅ" "リュ"
    "sa" "さ" "サ"
    "se" "せ" "セ"
    "sha" "しゃ" "シャ"
    "shi" "し" "シ"
    "sho" "しょ" "ショ"
    "shu" "しゅ" "シュ"
    "so" "そ" "ソ"
    "ssa" "っさ" "ッサ"
    "sse" "っせ" "ッセ"
    "ssha" "っしゃ" "ッシャ"
    "sshi" "っし" "ッシ"
    "ssho" "っしょ" "ッショ"
    "sshu" "っしゅ" "ッシュ"
    "sso" "っそ" "ッソ"
    "ssu" "っす" "ッス"
    "su" "す" "ス"
    "ta" "た" "タ"
    "te" "て" "テ"
    "ti" "てぃ" "ティ"
    "to" "と" "ト"
    "tsu" "つ" "ツ"
    "tta" "った" "ッタ"
    "tte" "って" "ッテ"
    "tto" "っと" "ット"
    "ttsu" "っつ" "ッツ"
    "tu" "つ" "ツ"
    "u" "う" "ウ"
    "wa" "わ" "ワ"
    "wo" "を" "ヲ"
    "ya" "や" "ヤ"
    "yo" "よ" "ヨ"
    "yu" "ゆ" "ユ"
    "za" "ざ" "ザ"
    "ze" "ぜ" "ゼ"
    "zo" "ぞ" "ゾ"
    "zu" "ず" "ズ"
}

// ぃぅぇぉゃゅゎゕゖ゛゜ゝゞゟ ゔ
// ゐ = wi (obsolete kana)
// ゑ = we (obsolete kana)

// ッィゥェォャュョヰヱヴヵヶヷヸヹヺ・ーヽヾヮ
