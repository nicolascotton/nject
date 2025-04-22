use std::collections::HashMap;

/// Groups the items by a key.
pub fn group_by<T, K, F>(iter: impl Iterator<Item = T>, key: F) -> HashMap<K, Vec<T>>
where
    F: Fn(&T) -> K,
    K: std::hash::Hash + Eq,
{
    let mut map = HashMap::new();
    for item in iter {
        let k = key(&item);
        map.entry(k).or_insert_with(Vec::new).push(item);
    }
    map
}
