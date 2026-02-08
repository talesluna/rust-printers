pub struct OptionsCollection<T, G> {
    _raw: Vec<(T, T)>,
    items: Vec<G>,
    pub size: usize,
}

impl<T, G> OptionsCollection<T, G> {
    pub fn new<F>(entries: &[(&str, String)], iterator: F) -> Self
    where
        F: Fn(&(&str, String)) -> ((T, T), G),
    {
        let capacity = entries.len();
        let mut _raw = Vec::with_capacity(capacity);
        let mut items = Vec::with_capacity(capacity);
        for entry in entries {
            let (entry, item) = iterator(entry);
            items.push(item);
            _raw.push(entry);
        }
        OptionsCollection {
            _raw,
            items,
            size: entries.len(),
        }
    }

    pub fn as_ptr(&self) -> *const G {
        self.items.as_ptr()
    }

    pub fn to_vec(&self) -> &Vec<G> {
        &self.items
    }
}
