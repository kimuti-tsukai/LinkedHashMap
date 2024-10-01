use std::rc::Rc;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub struct LinkedValue<K, V> {
    pub value: V,
    pub(crate) key: Rc<K>,
    pub(crate) prev: Option<Rc<K>>,
    pub(crate) next: Option<Rc<K>>,
}
