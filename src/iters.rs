use std::{
    hash::{BuildHasher, Hash},
    rc::Rc,
};

use crate::{inner_map::InnerLinkedHashMap, linked_value::LinkedValue, LinkedHashMap};

pub struct Iter<'a, K, V, S> {
    inner: Option<InnerIter<'a, K, V, S>>,
}

impl<'a, K, V, S> Iter<'a, K, V, S> {
    pub(crate) fn from_linked_hash_map(table: &'a LinkedHashMap<K, V, S>) -> Self {
        let Some(table) = table.inner() else {
            return Iter { inner: None };
        };

        Iter {
            inner: Some(InnerIter {
                table,
                next_key: Some(table.first_key()),
            }),
        }
    }
}

impl<'a, K, V, S> Iterator for Iter<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut()?.next()
    }
}

struct InnerIter<'a, K, V, S> {
    table: &'a InnerLinkedHashMap<K, V, S>,
    next_key: Option<&'a K>,
}

impl<'a, K, V, S> Iterator for InnerIter<'a, K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.next_key?;
        let LinkedValue {
            value: v,
            prev: _,
            next,
        } = self.table.get_raw(k).unwrap();

        if let Some(next_key) = next.as_ref() {
            self.next_key = Some(next_key.as_ref())
        }

        Some((k, v))
    }
}

pub struct IntoIter<K, V, S> {
    inner: Option<InnerIntoIter<K, V, S>>,
}

impl<K, V, S> IntoIter<K, V, S> {
    pub(crate) fn from_linked_hash_map(table: LinkedHashMap<K, V, S>) -> Self {
        let Some(table) = table.inner_owned() else {
            return IntoIter { inner: None };
        };

        let next_key = Rc::clone(table.first_key_rc());

        IntoIter {
            inner: Some(InnerIntoIter {
                next_key: Some(next_key),
                table,
            }),
        }
    }
}

impl<K, V, S> Iterator for IntoIter<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.as_mut()?.next()
    }
}

struct InnerIntoIter<K, V, S> {
    table: InnerLinkedHashMap<K, V, S>,
    next_key: Option<Rc<K>>,
}

impl<K, V, S> Iterator for InnerIntoIter<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.next_key.as_ref()?;
        let LinkedValue {
            value: v,
            prev: _,
            next,
        } = self.table.remove_raw(k).unwrap();

        // std::mem::drop(prev);

        let Some(next_key) = next else {
            unreachable!();
        };

        let k =
            std::mem::replace(&mut self.next_key, Some(next_key)).unwrap();

        Some((Rc::into_inner(k).unwrap(), v))
    }
}
