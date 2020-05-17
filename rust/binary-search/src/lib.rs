use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mid = array.len() / 2;
    let value = array[mid];

    match value.cmp(&key) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => find(array.split_at(mid).0, key),
        Ordering::Less => find(array.split_at(mid + 1).1, key).map(|index| mid + 1 + index),
    }
}
