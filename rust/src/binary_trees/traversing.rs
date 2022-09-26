use crate::binary_trees::tree_node::TreeNode;
use std::cell::RefCell;
use std::rc::Rc;

pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let items = Vec::new();
    match root {
        Some(node) => preorder_traverse(&node, items),
        None => items,
    }
}

fn preorder_traverse(node: &Rc<RefCell<TreeNode>>, mut items: Vec<i32>) -> Vec<i32> {
    items.push(node.borrow().val);

    items = match &node.borrow().left {
        Some(left_node) => preorder_traverse(left_node, items),
        None => items,
    };

    items = match &node.borrow().right {
        Some(right_node) => preorder_traverse(right_node, items),
        None => items,
    };

    items
}

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let items = Vec::new();
    match root {
        Some(node) => inorder_traverse(&node, items),
        None => items,
    }
}

fn inorder_traverse(node: &Rc<RefCell<TreeNode>>, mut items: Vec<i32>) -> Vec<i32> {

    items = match &node.borrow().left {
        Some(left_node) => inorder_traverse(left_node, items),
        None => items,
    };

    items.push(node.borrow().val);
    
    items = match &node.borrow().right {
        Some(right_node) => inorder_traverse(right_node, items),
        None => items,
    };

    items 
}
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let items = Vec::new();
    match root {
        Some(node) => postorder_traverse(&node, items),
        None => items,
    }
}

fn postorder_traverse(node: &Rc<RefCell<TreeNode>>, mut items: Vec<i32>) -> Vec<i32> {
    items = match &node.borrow().left {
        Some(left_node) => postorder_traverse(left_node, items),
        None => items,
    };

    
    items = match &node.borrow().right {
        Some(right_node) => postorder_traverse(right_node, items),
        None => items,
    };

    items.push(node.borrow().val);

    items 
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let items: Vec<Vec<i32>> = Vec::new();
    match root {
        Some(node) => level_order_traverse(vec![node], items),
        None => items,
    }
        
}

fn level_order_traverse(nodes: Vec<Rc<RefCell<TreeNode>>>, mut items: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let this_level_items: Vec<i32> = nodes.iter().map(|n| n.borrow().val).collect();
    items.push(this_level_items);
    let mut next_level_nodes: Vec<Rc<RefCell<TreeNode>>> = Vec::new();
    nodes.iter().for_each(|n| {
        
        match &n.borrow().left  {
            Some(left_node) =>  next_level_nodes.push(Rc::clone(&left_node)),
            None => {},
        };
        match &n.borrow().right {
            Some(right_node) => next_level_nodes.push(Rc::clone(&right_node)),
            None => {},
        }
    });

    if next_level_nodes.len() > 0 {
        level_order_traverse(next_level_nodes, items)   
    } else {
        items
    }

}

#[cfg(test)]
mod traversing_tests {
    use crate::binary_trees::tree_node::TreeNode;

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
    fn test_pre_order_traversal() {
        let expected_result = vec![6, 2, 1, 4, 3, 5, 7, 9, 8];
        let result = preorder_traversal(Some(Rc::from(RefCell::from(test_tree()))));
        assert_eq!(result, expected_result)
    }
    
    #[test]
    fn test_inorder_traversal() {
        let expected_result = vec![1,2,3,4,5,6,7,8,9];
        let result = inorder_traversal(Some(Rc::from(RefCell::from(test_tree()))));
        assert_eq!(result, expected_result)
    }
    
    #[test]
    fn test_postorder_traversal() {
        let expected_result = vec![1, 3, 5, 4, 2, 8, 9, 7, 6];
        let result = postorder_traversal(Some(Rc::from(RefCell::from(test_tree()))));
        assert_eq!(result, expected_result)
    }
    #[test]
    fn test_bfs_traversal() {
        let expected_result = vec![1, 3, 5, 4, 2, 8, 9, 7, 6];
        let result = level_order(Some(Rc::from(RefCell::from(test_tree()))));
        assert_eq!(expected_result, expected_result)
    }
}
