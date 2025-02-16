#[derive(Clone, Copy, Debug)]
pub struct RadicalPair {
    pub name: &'static str,
    pub ch: char,
}

macro_rules! radicals {
    ($($ch:literal $name:literal)*) => {
        pub const PAIRS: &[RadicalPair] = &[$(RadicalPair {
            name: $name,
            ch: $ch,
        }),*];
    };
}

radicals! {
    '⺅' "にんべん"
    '𠆢' "ひとやね"
    '⼉' "ひとあし"
    '⺡' "さんずい"
    '⺨' "けものへん"
    '⼻' "ぎょうにんべん"
    '⺾' "くさかんむり"
    '隹' "ふるとり"
}

pub fn by_name(name_frag: &str) -> impl Iterator<Item = RadicalPair> + '_ {
    PAIRS
        .iter()
        .cloned()
        .filter(move |pair| pair.name.contains(name_frag))
}
