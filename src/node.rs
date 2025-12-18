use crate::arena::Handle;

pub(crate) struct Node<T> {
    pub(crate) key: T,
    pub(crate) sz: usize, // subtree size
    pub(crate) parent: Option<Handle>,
    pub(crate) left: Option<Handle>,  // left child
    pub(crate) right: Option<Handle>, // right child
}

impl<T> Node<T> {
    pub(crate) fn new(value: T, parent: Option<Handle>) -> Self {
        return Self {
            key: value,
            sz: 1, // include itself
            parent: parent,
            left: None,
            right: None,
        };
    }
}
