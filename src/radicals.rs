#[derive(Clone, Copy)]
pub struct RadicalPair {
    pub name: &'static str,
    pub ch: char,
}

const PAIRS: &[RadicalPair] = &[RadicalPair {
    name: "にんべん",
    ch: '⺅',
}];

pub fn by_name(name_frag: &str) -> impl Iterator<Item = RadicalPair> + '_ {
    PAIRS
        .iter()
        .cloned()
        .filter(move |pair| pair.name.contains(name_frag))
}
