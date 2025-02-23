#[derive(Clone, Copy, Debug)]
pub struct Radical {
    pub chars: &'static [char],
    pub names: &'static [&'static str],
    pub common_names: &'static [&'static str],
}

macro_rules! radicals {
    ($($($ch:literal)+,$($name:literal)+,$($cname:literal)+;)*) => {
        pub const RADICALS: &'static [Radical] = &[
            $(
                Radical {
                    chars: &[$($ch,)+],
                    names: &[$($name,)+],
                    common_names: &[$($cname,)+],
                },
            )*
        ];
    }
}

include!("../radicals.incl");

pub fn by_name(name_frag: &str) -> impl Iterator<Item = Radical> + '_ {
    RADICALS
        .iter()
        .cloned()
        .filter(move |rad| rad.common_names.iter().any(|name| name.contains(name_frag)))
}
