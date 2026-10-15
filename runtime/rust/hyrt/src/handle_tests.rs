use super::*;

fn empty_table() -> Table {
    Table { slots: Vec::new(), free: Vec::new() }
}

#[test]
fn released_handle_cannot_alias_a_reused_slot() {
    let mut old = 0;
    assert_eq!(hy_json_new_array(&mut old), 0);
    assert_eq!(hy_json_release(old), 0);
    let mut live = 0;
    assert_eq!(hy_json_new_array(&mut live), 0);
    // Same slot or a different one: the old identity must never be live again.
    let mut len = 99;
    assert_eq!(hy_json_array_len(old, &mut len), ERR_TABLE);
    assert_eq!(len, 99);
    assert_eq!(hy_json_array_len(live, &mut len), 0);
    assert_eq!(len, 0);
    assert_eq!(hy_json_release(live), 0);
}

#[test]
fn delayed_release_after_move_preserves_replacement() {
    let (mut parent, mut child, mut replacement) = (0, 0, 0);
    assert_eq!(hy_json_new_object(&mut parent), 0);
    assert_eq!(hy_json_new_array(&mut child), 0);
    assert_eq!(hy_json_set(parent, b"child".as_ptr() as u64, 5, child), 0);
    assert_eq!(hy_json_new_array(&mut replacement), 0);
    // AddCleanup receives the original ID by value, including after Set.
    assert_eq!(hy_json_release(child), 0);
    let mut len = 99;
    assert_eq!(hy_json_array_len(replacement, &mut len), 0);
    assert_eq!(len, 0);
    assert_eq!(hy_json_release(parent), 0);
    assert_eq!(hy_json_release(replacement), 0);
}

#[test]
fn live_handles_retain_isolated_values() {
    let mut t = empty_table();
    let first = table_insert(&mut t, Json::from(11));
    let second = table_insert(&mut t, Json::from(22));
    assert_ne!(first, second);
    assert_eq!(table_get(&t, first).unwrap(), &Json::from(11));
    assert_eq!(table_get(&t, second).unwrap(), &Json::from(22));
}
