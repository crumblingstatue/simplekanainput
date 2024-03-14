use std::collections::HashMap;

/// Detects if `new` inserted or deleted items compared to `old`.
/// Assumes there is only point of edit. (one cursor)
///
/// Returns (position, extent) where extent is:
/// - zero if there are no changes
/// - negative if there were removed items
/// - positive if there were inserted items
fn detect_edit<T: PartialEq>(old: &[T], new: &[T]) -> (usize, isize) {
    let pos = old
        .iter()
        .zip(new.iter())
        .take_while(|(old, new)| old == new)
        .count();
    (pos, new.len() as isize - old.len() as isize)
}

#[test]
fn test_no_change() {
    let slice1 = &[0, 1, 2];
    let slice2 = &[0, 1, 2];
    assert_eq!(detect_edit(slice1, slice2).1, 0);
}

#[test]
fn test_various() {
    let slice1 = &["watashi", "no", "yume"];
    // Insertion of one item at index 2
    let slice2 = &["watashi", "no", "daijina", "yume"];
    assert_eq!(detect_edit(slice1, slice2), (2, 1));
    //  Deletion of two items at index 1
    let slice3 = &["watashi", "yume"];
    assert_eq!(detect_edit(slice2, slice3), (1, -2));
    // Add two items at index 0
    let slice4 = &["kore", "ha", "watashi", "yume"];
    assert_eq!(detect_edit(slice3, slice4), (0, 2));
}

fn update_index_map<V>(map: &mut HashMap<usize, V>, pos: usize, extent: isize) {
    let mut kvpairs: Vec<_> = map.drain().collect();
    kvpairs.retain_mut(|(k, _)| {
        let mut retain = true;
        if *k >= pos {
            match k.checked_add_signed(extent) {
                Some(val) => *k = val,
                None => retain = false,
            }
        }
        retain
    });
    *map = HashMap::from_iter(kvpairs);
}

pub fn detect_edit_update_index_map<T: PartialEq, V>(
    map: &mut HashMap<usize, V>,
    old: &[T],
    new: &[T],
) {
    let (pos, extent) = detect_edit(old, new);
    if extent != 0 {
        update_index_map(map, pos, extent);
    }
}

#[test]
fn index_tracking_proof_of_concept() {
    let tok = |s: &'static str| s.split(' ').collect::<Vec<_>>();
    let tokens1 = tok("watashi ha ningen desu.");
    let mut attr_map = HashMap::from([(0usize, "私"), (2, "人間")]);
    // Insert 3 tokens at 0
    let tokens2 = tok("hai, sou desu. watashi ha ningen desu.");
    detect_edit_update_index_map(&mut attr_map, &tokens1, &tokens2);
    assert_eq!(attr_map[&3], "私");
    assert_eq!(attr_map[&5], "人間");
    let tokens3 = tok("ningen desu.");
    // Remove 5 tokens at 0
    detect_edit_update_index_map(&mut attr_map, &tokens2, &tokens3);
    assert_eq!(attr_map[&0], "人間");
}
