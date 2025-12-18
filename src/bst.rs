use crate::arena::{Arena, Handle};
use crate::node::Node;

struct gddset<T: Ord> {
    root: Option<Handle>,
    storage: Arena<Node<T>>,
}

impl<T: Ord + Clone> gddset<T> {
    pub(crate) fn debug_traverse(&self, pos_handle: Handle, out: &mut Vec<T>) {
        // yet allow empty tree, handle no Option (for the future)
        let node = self.storage.get(pos_handle).unwrap();

        // in-order
        if let Some(lc) = node.left {
            self.debug_traverse(lc, out);
        }
        
        out.push(node.key.clone());
        if let Some(rc) = node.right {
            self.debug_traverse(rc, out);
        }
    }
}

impl<T: Ord> gddset<T> {
    pub(crate) fn new() -> Self {
        gddset {
            root: None,
            storage: Arena::new(),
        }
    }
    // when false ?
    // - key already exist
    pub(crate) fn insert(&mut self, insert_value: T) -> bool {
        if self.root.is_none() {
            let handle = self.storage.alloc(Node::new(insert_value, None));
            self.root = Some(handle);

            return true;
        }

        // Handle class is not moving but cloning
        // pos traverses and stop at the target's parent
        let mut pos_handle = self.root.clone().unwrap();
        let mut go_left = false;

        'traverse: loop {
            let pos_node = self.storage.get(pos_handle).unwrap();

            if insert_value == pos_node.key {
                return false;
            } else if insert_value < pos_node.key {
                if let Some(tar_handle) = pos_node.left {
                    pos_handle = tar_handle;
                } else {
                    go_left = true;
                    break 'traverse;
                }
            } else {
                if let Some(tar_node) = pos_node.right {
                    pos_handle = tar_node;
                } else {
                    go_left = false;
                    break 'traverse;
                }
            }
        }

        // connect par(pos) and child(tar)
        let tar_handle = self
            .storage
            .alloc(Node::new(insert_value, Some(pos_handle)));
        let pos_node = self.storage.get_mut(pos_handle).unwrap();
        if go_left {
            pos_node.left = Some(tar_handle);
        } else {
            pos_node.right = Some(tar_handle);
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 引入 gddset
    use std::fmt::Debug;

    // 一個測試輔助函式（只存在於測試）
    fn collect_in_order<T: Ord + Debug>(set: &gddset<T>) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = Vec::new();
        set.debug_traverse(set.root.unwrap(), &mut out);
        out
    }

    #[test]
    fn insert_random_order_should_be_sorted() {
        let mut set = gddset::new();

        let input = vec![5, 3, 7, 2, 4, 6, 8];
        for v in &input {
            set.insert(*v);
        }

        let result = collect_in_order(&set);

        // 1. 數量正確
        assert_eq!(result.len(), input.len());

        // 2. 中序結果必須是非遞減
        assert!(
            result.windows(2).all(|w| w[0] <= w[1]),
            "not sorted: {:?}",
            result
        );
    }

    #[test]
    fn insert_increasing_order_should_still_work() {
        let mut set = gddset::new();

        for v in 1..=5 {
            set.insert(v);
        }

        let result = collect_in_order(&set);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn insert_decreasing_order_should_still_work() {
        let mut set = gddset::new();

        for v in (1..=5).rev() {
            set.insert(v);
        }

        let result = collect_in_order(&set);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }
}
