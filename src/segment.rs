pub fn segment(romaji: &str) -> Vec<&str> {
    let mut begin = 0;
    let mut cursor = 0;
    let bytes = romaji.as_bytes();
    let mut segs = Vec::new();
    while cursor < bytes.len() {
        match bytes[cursor] {
            b' ' | b'\n' => {
                let txt = &romaji[begin..cursor];
                if !txt.is_empty() {
                    segs.push(txt);
                    begin = cursor + 1;
                }
            }
            _ if cursor == bytes.len() - 1 => {
                segs.push(&romaji[begin..cursor + 1]);
            }
            _ => {}
        }
        cursor += 1;
    }
    segs
}

#[test]
fn test_segment() {
    assert_eq!(segment("watashi ha"), vec!["watashi", "ha"]);
    assert_eq!(segment("watashi  ha"), vec!["watashi", " ha"]);
}
