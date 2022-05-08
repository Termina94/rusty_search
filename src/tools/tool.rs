use std::collections::HashMap;

pub fn assert_args<T>(keys: T, args: Vec<String>) -> HashMap<String, String>
where
    T: IntoIterator<Item = String>,
{
    let mut map: HashMap<String, String> = HashMap::default();

    for (i, key) in keys.into_iter().enumerate() {
        map.insert(key.clone(), args.get(i + 2).unwrap().clone());
    }

    map
}
