use std::{rc::Rc, cell::RefCell};

use super::{tree_node::TreeNode, traversing::{preorder_traversal, level_order}};

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

pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match root {
        Some(n) => {
            let reflected = reflect_node(Some(&n)); 
            Some(n.clone()) == reflected
        },
        None => false,
    }   
}

fn reflect_node(node: Option<&Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(n) => {
            let reflected_left = reflect_node(n.borrow().left.as_ref());
            let reflected_right = reflect_node(n.borrow().right.as_ref());
            Some(Rc::new(RefCell::new(TreeNode{
                val: n.borrow().val,
                left: reflected_right,
                right: reflected_left,
            })))
        },
        None => None,
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
    #[test]
    fn test_symmetrical(){
        let t = Some(Rc::new(RefCell::new(test_tree())));
        assert_eq!(is_symmetric(t), false)
    }
}
