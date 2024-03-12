enum Status {
    Init,
    RomajiText,
    OtherText,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub kind: SegmentKind,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SegmentKind {
    Romaji,
    Other,
}

impl Span {
    pub fn new(start: usize, end: usize, kind: SegmentKind) -> Self {
        Self { start, end, kind }
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
                    segs.push(Span::new(last_segment_begin, pos, SegmentKind::Romaji));
                    status = Status::OtherText;
                    last_segment_begin = pos;
                }
            }
            Status::OtherText => {
                if is_romaji {
                    segs.push(Span::new(last_segment_begin, pos, SegmentKind::Other));
                    status = Status::RomajiText;
                    last_segment_begin = pos;
                }
            }
        }
    }
    let remainder_kind = match status {
        Status::Init => {
            // The most likely (only?) scenario here is that the input text was empty.
            return segs;
        }
        Status::RomajiText => SegmentKind::Romaji,
        Status::OtherText => SegmentKind::Other,
    };
    let remainder = Span::new(last_segment_begin, input_text.len(), remainder_kind);
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
