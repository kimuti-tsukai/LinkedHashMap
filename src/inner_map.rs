use std::{
    collections::HashMap,
    hash::{BuildHasher, Hash, RandomState},
    rc::Rc,
};

use crate::LinkedValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct InnerLinkedHashMap<K, V, S = RandomState> {
    table: HashMap<Rc<K>, LinkedValue<K, V>, S>,
    now_key: Rc<K>,
    first_key: Rc<K>,
    end_key: Rc<K>,
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> InnerLinkedHashMap<K, V, S> {
    pub(crate) fn with_first_value(k: K, v: V) -> Self {
        let k = Rc::new(k);

        let mut inner_table = HashMap::default();
        inner_table.insert(
            Rc::clone(&k),
            LinkedValue {
                value: v,
                key: Rc::clone(&k),
                prev: None,
                next: None,
            },
        );

        InnerLinkedHashMap {
            table: inner_table,
            now_key: Rc::clone(&k),
            first_key: Rc::clone(&k),
            end_key: Rc::clone(&k),
        }
    }
}

impl<K, V, S> InnerLinkedHashMap<K, V, S> {
    pub(crate) fn len(&self) -> usize {
        self.table.len()
    }

    pub(crate) fn first_key(&self) -> &K {
        &self.first_key
    }

    pub(crate) fn first_key_rc(&self) -> &Rc<K> {
        &self.first_key
    }
}

impl<K, V, S> InnerLinkedHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub(crate) fn reverse(&mut self) {
        for v in self.table.values_mut() {
            std::mem::swap(&mut v.prev, &mut v.next);
        }
    }

    pub(crate) fn get(&self, k: &K) -> Option<&V> {
        Some(&self.table.get(k)?.value)
    }

    pub(crate) fn get_raw(&self, k: &K) -> Option<&LinkedValue<K, V>> {
        self.table.get(k)
    }

    pub(crate) fn get_mut(&mut self, k: &K) -> Option<&mut V> {
        Some(&mut self.table.get_mut(k)?.value)
    }

    pub(crate) fn get_raw_mut(&mut self, k: &K) -> Option<&mut LinkedValue<K, V>> {
        self.table.get_mut(k)
    }

    pub(crate) fn get_key_value(&self, k: &K) -> Option<(&K, &V)> {
        let (k, v) = self.table.get_key_value(k)?;

        Some((k.as_ref(), &v.value))
    }

    pub(crate) fn contains_key(&self, k: &K) -> bool {
        self.table.contains_key(k)
    }

    pub(crate) fn insert(&mut self, k: K, v: V) -> Option<V> {
        let k = Rc::new(k);

        if let Some(before_value) = self.get_mut(&k) {
            Some(std::mem::replace(before_value, v))
        } else {
            self.table.insert(
                Rc::clone(&k),
                LinkedValue {
                    value: v,
                    key: Rc::clone(&k),
                    prev: Some(Rc::clone(&self.now_key)),
                    next: None
                }
            );

            let r = self.table.get_mut(&self.now_key).unwrap();
            r.next = Some(Rc::clone(&k));
            self.end_key = Rc::clone(&k);
            self.now_key = Rc::clone(&k);
            None
        }
    }

    pub(crate) fn remove(&mut self, k: &K) -> Result<Option<V>, ()> {
        let Some(LinkedValue { value, key: _, prev, next }) = self.table.remove(k) else {
            return Ok(None);
        };

        match (prev, next) {
            (None, None) => return Err(()),
            (None, Some(next)) => {
                let next_value = self.table.get_mut(&next).unwrap();

                next_value.prev = None;

                self.first_key = next;
            }
            (Some(prev), None) => {
                let prev_value = self.table.get_mut(&prev).unwrap();

                prev_value.next = None;

                self.end_key = Rc::clone(&prev);
                self.now_key = prev;
            }
            (Some(prev), Some(next)) => {
                let prev_value = self.table.get_mut(&prev).unwrap();

                prev_value.next = Some(Rc::clone(&next));

                let next_value = self.table.get_mut(&next).unwrap();

                next_value.prev = Some(prev);
            }
        }

        Ok(Some(value))
    }

    pub(crate) fn remove_raw(&mut self, k: &K) -> Option<LinkedValue<K, V>> {
        self.table.remove(k)
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn insert() {
        let mut v: InnerLinkedHashMap<i32, &str, RandomState> = InnerLinkedHashMap::with_first_value(10, "Hello");

        dbg!(&v);

        v.insert(20, "Hello Hello");

        dbg!(&v);

        v.insert(30, "Hello Hello Hello");
    }
}
