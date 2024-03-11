enum Status {
    Init,
    RomajiText,
    OtherText,
}

pub fn segment(input_text: &str) -> Vec<&str> {
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
                    segs.push(&input_text[last_segment_begin..pos]);
                    status = Status::OtherText;
                    last_segment_begin = pos;
                }
            }
            Status::OtherText => {
                if is_romaji {
                    segs.push(&input_text[last_segment_begin..pos]);
                    status = Status::RomajiText;
                    last_segment_begin = pos;
                }
            }
        }
    }
    let remainder = &input_text[last_segment_begin..];
    if !remainder.is_empty() {
        segs.push(remainder);
    }
    segs
}

#[test]
fn test_segment() {
    assert_eq!(segment("watashi ha"), vec!["watashi", " ", "ha"]);
    assert_eq!(segment("watashi  ha"), vec!["watashi", "  ", "ha"]);
    assert_eq!(
        segment("hai, sou desu. nani?"),
        vec!["hai", ", ", "sou", " ", "desu", ". ", "nani", "?"]
    );
    assert_eq!(
        segment("are ha nandesu ka? zenkai boosto da!"),
        vec![
            "are", " ", "ha", " ", "nandesu", " ", "ka", "? ", "zenkai", " ", "boosto", " ", "da",
            "!"
        ]
    );
}
