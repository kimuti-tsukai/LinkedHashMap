use std::hash::{BuildHasher, Hash, RandomState};

mod inner_map;
use inner_map::InnerLinkedHashMap;

mod linked_value;
use linked_value::LinkedValue;

mod iters;

#[derive(Debug, Clone, Default)]
pub struct LinkedHashMap<K, V, S = RandomState> {
    inner: Option<InnerLinkedHashMap<K, V, S>>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> LinkedHashMap<K, V, S> {
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        if let Some(table) = &mut self.inner {
            table.insert(k, v)
        } else {
            self.inner = Some(InnerLinkedHashMap::with_first_value(k, v));

            None
        }
    }
}

impl<K, V, S> LinkedHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn reverse(&mut self) {
        if let Some(table) = &mut self.inner {
            table.reverse();
        }
    }

    pub fn get(&self, k: &K) -> Option<&V> {
        self.inner.as_ref()?.get(k)
    }

    pub fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        self.inner.as_mut()?.get_mut(k)
    }

    pub fn contains_key(&self, k: &K) -> bool {
        self.inner
            .as_ref()
            .is_some_and(|table| table.contains_key(k))
    }

    pub fn remove(&mut self, k: &K) -> Option<V> {
        self.inner.as_mut()?.remove(k).unwrap_or_else(|_| {
            self.inner = None;
            None
        })
    }
}

impl<K, V, S> LinkedHashMap<K, V, S> {
    pub fn new() -> Self {
        LinkedHashMap { inner: None }
    }

    pub fn len(&self) -> usize {
        match &self.inner {
            None => 0,
            Some(table) => table.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_none()
    }

    pub fn iter(&self) -> iters::Iter<'_, K, V, S> {
        iters::Iter::from_linked_hash_map(self)
    }

    pub(crate) fn inner(&self) -> Option<&InnerLinkedHashMap<K, V, S>> {
        self.inner.as_ref()
    }

    pub(crate) fn inner_owned(self) -> Option<InnerLinkedHashMap<K, V, S>> {
        self.inner
    }
}

impl<K: Eq + Hash + Clone, V, S: BuildHasher> IntoIterator for LinkedHashMap<K, V, S> {
    type IntoIter = iters::IntoIter<K, V, S>;
    type Item = (K, V);

    fn into_iter(self) -> Self::IntoIter {
        iters::IntoIter::from_linked_hash_map(self)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn into_iter() {
        let mut map: LinkedHashMap<i32, &str, RandomState> = LinkedHashMap::new();

        map.insert(10, "Hello");

        map.insert(20, "Hello Hello");

        map.insert(30, "Hello Hello Hello");

        map.insert(40, "Hello Hello Hello Hello");
    }
}
