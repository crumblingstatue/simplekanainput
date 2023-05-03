enum SegStatus {
    Init,
    InBracketed,
}

#[derive(PartialEq, Debug)]
pub enum Segment<'s> {
    Simple(&'s str),
    /// A dictionary form text and extra (conjugation) text.
    ///
    /// ```plaintext
    /// [deau:tte]
    ///   ^    ^
    ///  dict  extra
    /// ```
    ///
    /// Used for conjugated words like adjectives/verbs.
    DictAndExtra {
        dict: &'s str,
        extra: &'s str,
        /// How many characters to cut off from dict lookup result (usually 1).
        ///
        /// Determined by number of `:` separators.
        cutoff: usize,
    },
}
impl<'a> Segment<'a> {
    pub(crate) fn label_string(&self) -> String {
        match self {
            Segment::Simple(s) => s.to_string(),
            Segment::DictAndExtra { dict, extra, .. } => {
                format!("{dict}[{extra}]")
            }
        }
    }
    /// Used for dictionary lookups
    pub(crate) fn dict_root(&self) -> &str {
        match self {
            Segment::Simple(s) => s,
            Segment::DictAndExtra { dict, .. } => dict,
        }
    }
}

pub fn segment(romaji: &str) -> Vec<Segment> {
    let mut begin = 0;
    let mut cursor = 0;
    let bytes = romaji.as_bytes();
    let mut status = SegStatus::Init;
    let mut segs = Vec::new();
    let mut colons = 0;
    let mut extra_begin = 0;
    while cursor < bytes.len() {
        match status {
            SegStatus::Init => match bytes[cursor] {
                b'[' => {
                    let s = &romaji[begin..cursor];
                    if !s.is_empty() {
                        if colons == 0 {
                            segs.push(Segment::Simple(s));
                        } else {
                            segs.push(Segment::DictAndExtra {
                                dict: &romaji[begin..extra_begin],
                                extra: &romaji[extra_begin + colons..cursor],
                                cutoff: colons,
                            })
                        }
                        colons = 0;
                    }
                    status = SegStatus::InBracketed;
                    begin = cursor + 1;
                }
                b' ' | b'\n' => {
                    if colons == 0 {
                        let txt = &romaji[begin..cursor];
                        if !txt.is_empty() {
                            segs.push(Segment::Simple(txt));
                            begin = cursor + 1;
                        }
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor],
                            cutoff: colons,
                        });
                        begin = cursor + 1;
                    }
                    colons = 0;
                }
                b':' => {
                    extra_begin = cursor - colons;
                    colons += 1;
                }
                _ if cursor == bytes.len() - 1 => {
                    if colons == 0 {
                        segs.push(Segment::Simple(&romaji[begin..cursor + 1]));
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor + 1],
                            cutoff: colons,
                        })
                    }
                    colons = 0;
                }
                _ => {}
            },
            SegStatus::InBracketed => match bytes[cursor] {
                b']' => {
                    if colons == 0 {
                        segs.push(Segment::Simple(&romaji[begin..cursor]));
                    } else {
                        segs.push(Segment::DictAndExtra {
                            dict: &romaji[begin..extra_begin],
                            extra: &romaji[extra_begin + colons..cursor],
                            cutoff: colons,
                        })
                    }
                    colons = 0;
                    begin = cursor + 1;
                    status = SegStatus::Init;
                }
                b':' => {
                    extra_begin = cursor - colons;
                    colons += 1;
                }
                _ => {}
            },
        }
        cursor += 1;
    }
    segs
}

#[test]
fn test_segment() {
    use Segment::*;
    assert_eq!(
        segment("[chiisai:nakute]a"),
        vec![
            DictAndExtra {
                dict: "chiisai",
                extra: "nakute",
                cutoff: 1
            },
            Simple("a")
        ]
    );
    assert_eq!(
        segment("watashi[ha]chiisai:kute[shizuka]janaide[omoshiroi]machi[ni]sumu:ndeimasu[.]"),
        vec![
            Simple("watashi"),
            Simple("ha"),
            DictAndExtra {
                dict: "chiisai",
                extra: "kute",
                cutoff: 1
            },
            Simple("shizuka"),
            Simple("janaide"),
            Simple("omoshiroi"),
            Simple("machi"),
            Simple("ni"),
            DictAndExtra {
                dict: "sumu",
                extra: "ndeimasu",
                cutoff: 1
            },
            Simple(".")
        ]
    );
    assert_eq!(segment("watashi ha"), vec![Simple("watashi"), Simple("ha")]);
    assert_eq!(
        segment("watashi  ha"),
        vec![Simple("watashi"), Simple(" ha")]
    );
}
