use std::{rc::Rc, cell::RefCell};

use super::tree_node::TreeNode;

pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(n) => get_depth(&n, 1),
        None => 0,
    }
}

fn get_depth(node: &Rc<RefCell<TreeNode>>, current_depth: i32) -> i32 {
    let left_depth = match &node.borrow().left {
        Some(l) => get_depth(l, current_depth + 1),
        None => current_depth,
    };
    let right_depth = match &node.borrow().right {
        Some(r) => get_depth(r, current_depth + 1),
        None => current_depth,
    };

    if left_depth > right_depth {
        left_depth
    } else {
        right_depth
    }
}

#[cfg(test)]
mod binary_tree_recursion_test {
    use super::*;

    fn test_tree() -> TreeNode {
        let a = TreeNode {
            val: 1,
            left: None,
            right: None,
        };
        let c = TreeNode {
            val: 3,
            left: None,
            right: None,
        };
        let e = TreeNode {
            val: 5,
            left: None,
            right: None,
        };
        let h = TreeNode {
            val: 8,
            left: None,
            right: None,
        };

        let d = TreeNode {
            val: 4,
            left: Some(Rc::from(RefCell::from(c))),
            right: Some(Rc::from(RefCell::from(e))),
        };
        let i = TreeNode {
            val: 9,
            left: Some(Rc::from(RefCell::from(h))),
            right: None,
        };
        let b = TreeNode {
            val: 2,
            left: Some(Rc::from(RefCell::from(a))),
            right: Some(Rc::from(RefCell::from(d))),
        };
        let g = TreeNode {
            val: 7,
            left: None,
            right: Some(Rc::from(RefCell::from(i))),
        };

        TreeNode {
            val: 6,
            left: Some(Rc::from(RefCell::from(b))),
            right: Some(Rc::from(RefCell::from(g))),
        }
    }

    #[test]
    fn test_max_depth(){
        let t = Some(Rc::new(RefCell::new(test_tree())));
        assert_eq!(max_depth(t), 4)
    }
}
