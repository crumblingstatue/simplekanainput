enum Status {
    Init,
    RomajiText,
    OtherText,
    ExplicitOther,
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

    /// Used to check whether the text cursor is "on" this span
    pub fn contains_cursor(&self, cursor: usize) -> bool {
        (self.start..=self.end).contains(&cursor)
    }
}

pub fn segment(input_text: &str) -> Vec<Span> {
    let mut segs = Vec::new();
    let mut status = Status::Init;
    let mut last_segment_begin = 0;
    for (pos, byte) in input_text.bytes().enumerate() {
        let is_romaji = byte.is_ascii_alphabetic() || byte == b'-';
        match status {
            Status::Init => {
                if is_romaji {
                    status = Status::RomajiText;
                } else if byte == b'{' {
                    status = Status::ExplicitOther;
                    last_segment_begin = pos + 1;
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
                } else if byte == b'{' {
                    segs.push(Span::new(last_segment_begin, pos, SegmentKind::Other));
                    status = Status::ExplicitOther;
                    last_segment_begin = pos + 1;
                }
            }
            Status::ExplicitOther => {
                if byte == b'}' {
                    segs.push(Span::new(last_segment_begin, pos, SegmentKind::Other));
                    status = Status::Init;
                    last_segment_begin = pos + 1;
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
        Status::OtherText | Status::ExplicitOther => SegmentKind::Other,
    };
    let remainder = Span::new(last_segment_begin, input_text.len(), remainder_kind);
    if remainder.len() != 0 {
        segs.push(remainder);
    }
    // Special behavior: Get rid of single space segments. This allows
    // nice continuous output, which Japanese readers usually expect.
    // The user can still insert two spaces if they want to insert a space.
    segs.retain(|seg| seg.index(input_text) != " ");
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
        "watashi ha" => "watashi", "ha";
        "watashi  ha" => "watashi", "  ", "ha";
        "hai, sou desu. nani?" => "hai", ", ", "sou","desu", ". ", "nani", "?";
        "are ha nandesu ka? zenkai boosto da!" => "are", "ha", "nandesu", "ka", "? ", "zenkai",
            "boosto", "da", "!";
        "supe-su ha sugoi ne" => "supe-su", "ha", "sugoi", "ne";
        "konnichiha {Yes. This is a free space 空.} rafaeru san." => "konnichiha",
            "Yes. This is a free space 空.", "rafaeru", "san";
        "{free space}" => "free space";
    }
}
