/// Sourced from `joyo_data.txt` from https://github.com/wareya/ids_rust
const JOYO_DATA: &str = include_str!("../joyokanji.bin");

#[derive(Debug, Clone)]
pub struct Kanji {
    pub chars: [&'static str; 3],
    pub meaning: &'static str,
    pub readings: Vec<&'static str>,
}

pub struct KanjiDb {
    pub kanji: Vec<Kanji>,
}

impl KanjiDb {
    pub fn load() -> Self {
        let mut iter = JOYO_DATA.split('\0');
        let mut kanji = Vec::new();
        loop {
            let Some([k1, k2, k3, meaning, readings]) = std::array::try_from_fn(|_| iter.next())
            else {
                break Self { kanji };
            };
            let readings: Vec<_> = readings.split('、').collect();
            kanji.push(Kanji {
                chars: [k1, k2, k3],
                meaning,
                readings,
            });
        }
    }
}
