enum Status {
    Init,
    RomajiText,
    OtherText,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub(crate) fn index<'a>(&self, str: &'a str) -> &'a str {
        &str[self.start..self.end]
    }
}

pub fn segment(input_text: &str) -> Vec<Span> {
    let mut segs = Vec::new();
    let mut status = Status::Init;
    let mut last_segment_begin = 0;
    for (pos, byte) in input_text.bytes().enumerate() {
        let is_romaji = byte.is_ascii_lowercase();
        match status {
            Status::Init => {
                if is_romaji {
                    status = Status::RomajiText;
                } else {
                    status = Status::OtherText;
                }
            }
            Status::RomajiText => {
                if !is_romaji {
                    segs.push(Span::new(last_segment_begin, pos));
                    status = Status::OtherText;
                    last_segment_begin = pos;
                }
            }
            Status::OtherText => {
                if is_romaji {
                    segs.push(Span::new(last_segment_begin, pos));
                    status = Status::RomajiText;
                    last_segment_begin = pos;
                }
            }
        }
    }
    let remainder = Span::new(last_segment_begin, input_text.len());
    if remainder.len() != 0 {
        segs.push(remainder);
    }
    segs
}

#[test]
fn test_segment() {
    macro_rules! test_cases {
        ($($src:literal => $($token:literal$(,)?)*;)*) => {
            $(
                let mut spans = segment($src).into_iter();
                dbg!(&spans);
                $(
                    assert_eq!(spans.next().unwrap().index($src), $token);
                )*
            )*
        };
    }
    test_cases! {
        "watashi ha" => "watashi", " ", "ha";
        "watashi  ha" => "watashi", "  ", "ha";
        "hai, sou desu. nani?" => "hai", ", ", "sou", " ", "desu", ". ", "nani", "?";
        "are ha nandesu ka? zenkai boosto da!" => "are", " ", "ha", " ", "nandesu", " ",
        "ka", "? ", "zenkai", " ", "boosto", " ", "da", "!";
    }
}
