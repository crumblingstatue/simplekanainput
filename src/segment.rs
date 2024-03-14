#[derive(Clone, Copy, PartialEq, Debug)]
pub enum InputSpan {
    RomajiWord { start: usize, end: usize },
    RomajiPunct { start: usize, end: usize },
    Other { start: usize, end: usize },
}

impl InputSpan {
    pub fn len(self) -> usize {
        match self {
            InputSpan::RomajiWord { start, end }
            | InputSpan::Other { start, end }
            | InputSpan::RomajiPunct { start, end } => end - start,
        }
    }

    /// Private method, shouldn't be used willy-nilly, match on the enum first, then index.
    fn index(self, str: &str) -> &str {
        match self {
            InputSpan::RomajiWord { start, end }
            | InputSpan::Other { start, end }
            | InputSpan::RomajiPunct { start, end } => &str[start..end],
        }
    }

    /// Used to check whether the text cursor is "on" this span
    pub fn contains_cursor(self, cursor: usize) -> bool {
        match self {
            InputSpan::RomajiWord { start, end }
            | InputSpan::Other { start, end }
            | InputSpan::RomajiPunct { start, end } => (start..=end).contains(&cursor),
        }
    }

    pub(crate) fn is_romaji_word(self) -> bool {
        matches!(self, InputSpan::RomajiWord { .. })
    }
}

pub fn segment(input_text: &str) -> Vec<InputSpan> {
    enum Status {
        Init,
        // Romaji word
        RomajiWord,
        // Punctuation, segmented separately from word text
        RomajiPunct,
        OtherText,
        ExplicitOther,
    }

    let mut segs = Vec::new();
    let mut status = Status::Init;
    let mut last_segment_begin = 0;
    for (pos, byte) in input_text.bytes().enumerate() {
        let is_romaji_word = byte.is_ascii_alphabetic() || matches!(byte, b'-');
        let is_romaji_punct = matches!(byte, b'.' | b',' | b'!' | b'?');
        match status {
            Status::Init => {
                if is_romaji_word {
                    status = Status::RomajiWord;
                } else if byte == b'{' {
                    status = Status::ExplicitOther;
                    last_segment_begin = pos + 1;
                } else if is_romaji_punct {
                    status = Status::RomajiPunct;
                } else {
                    status = Status::OtherText;
                }
            }
            Status::RomajiWord => {
                if !is_romaji_word {
                    segs.push(InputSpan::RomajiWord {
                        start: last_segment_begin,
                        end: pos,
                    });
                    if is_romaji_punct {
                        status = Status::RomajiPunct;
                    } else {
                        status = Status::OtherText;
                    }
                    last_segment_begin = pos;
                }
            }
            Status::OtherText => {
                if is_romaji_word {
                    segs.push(InputSpan::Other {
                        start: last_segment_begin,
                        end: pos,
                    });
                    status = Status::RomajiWord;
                    last_segment_begin = pos;
                } else if byte == b'{' {
                    segs.push(InputSpan::Other {
                        start: last_segment_begin,
                        end: pos,
                    });
                    status = Status::ExplicitOther;
                    last_segment_begin = pos + 1;
                }
            }
            Status::ExplicitOther => {
                if byte == b'}' {
                    segs.push(InputSpan::Other {
                        start: last_segment_begin,
                        end: pos,
                    });
                    status = Status::Init;
                    last_segment_begin = pos + 1;
                }
            }
            Status::RomajiPunct => {
                if !is_romaji_punct {
                    segs.push(InputSpan::RomajiPunct {
                        start: last_segment_begin,
                        end: pos,
                    });
                    if is_romaji_word {
                        status = Status::RomajiWord;
                    } else {
                        status = Status::OtherText;
                    }
                    last_segment_begin = pos;
                }
            }
        }
    }
    // Deal with remainder
    let start = last_segment_begin;
    let end = input_text.len();
    let remainder = match status {
        Status::Init => {
            // The most likely (only?) scenario here is that the input text was empty.
            return segs;
        }
        Status::RomajiWord => InputSpan::RomajiWord { start, end },
        Status::RomajiPunct => InputSpan::RomajiPunct { start, end },
        Status::OtherText | Status::ExplicitOther => InputSpan::Other { start, end },
    };
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
        "hai, sou desu. nani?" => "hai", ",", "sou","desu", ".", "nani", "?";
        "are ha nandesu ka? zenkai boosto da!" => "are", "ha", "nandesu", "ka", "?", "zenkai",
            "boosto", "da", "!";
        "supe-su ha sugoi ne" => "supe-su", "ha", "sugoi", "ne";
        "konnichiha {Yes. This is a free space 空.} rafaeru san." => "konnichiha",
            "Yes. This is a free space 空.", "rafaeru", "san";
        "{free space}" => "free space";
    }
}
