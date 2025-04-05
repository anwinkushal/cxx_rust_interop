use rstest::rstest;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
type BinaryTree = Option<Rc<RefCell<TreeNode>>>;
struct Solution;

impl Solution {
    fn preorder_traversal_iter(root: BinaryTree) -> Vec<i32> {
        let mut result = vec![];
        let mut stack  = vec![root];
        while let Some(node_opt) = stack.pop() {
            match node_opt {
                None => continue,
                Some(pnode) => {
                    let node = pnode.borrow();
                    result.push(node.val);
                    stack.push(node.right.clone());
                    stack.push(node.left.clone());
                }
            }
        }
        result
    }
}

#[rstest]
#[case(vec![Some(1), None, Some(2), Some(3)], vec![1, 2, 3])] // Right child only
#[case(vec![Some(1), Some(2), Some(3), Some(4), Some(5), None, Some(8), None, None, Some(6), Some(7), Some(9)], vec![1, 2, 4, 5, 6, 7, 3, 8, 9])] // Complex tree
#[case(vec![], vec![])] // Empty tree
#[case(vec![Some(1)], vec![1])] // Single node
#[case(vec![Some(1), Some(2), None, Some(3)], vec![1, 2, 3])] // Left child only
#[case(vec![Some(1), None, Some(2), None, Some(3)], vec![1, 2, 3])] // Right skewed tree
#[case(vec![Some(1), Some(2), Some(3), None, Some(4), Some(5), None], vec![1, 2, 4, 3, 5])] // Mixed children
fn test_preorder_traversal(#[case] tree: Vec<Option<i32>>, #[case] expected: Vec<i32>) {
    // println!("Running test case: {:?}", tree);
    let root = build_tree(&tree);
    // print_tree(&root.clone());
    assert_eq!(Solution::preorder_traversal_iter(root), expected);
}
pub fn print_tree(root: &Option<Rc<RefCell<TreeNode>>>) {
    fn print_tree_helper(node: &Option<Rc<RefCell<TreeNode>>>, indent: &str, is_left: bool) {
        // If node exists
        if let Some(node_ref) = node {
            let node = node_ref.borrow(); // Get the TreeNode inside
            
            // Print current node with appropriate prefix
            let connector = if is_left { "├── " } else { "└── " };
            println!("{}{}{}", indent, connector, node.val);
            
            // Prepare indentation for children
            let child_indent = if is_left { 
                format!("{}│   ", indent)  // Keep vertical line for left children
            } else { 
                format!("{}    ", indent) // Just spaces for right children
            };
            
            // Print left and right children
            print_tree_helper(&node.left, &child_indent, true);
            print_tree_helper(&node.right, &child_indent, false);
        }
    }
    
    println!("Tree:");
    print_tree_helper(root, "", false);
}

// Helper function to build tree from level-order traversal
fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    if values.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0]?)));
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < values.len() {
        let node = queue.pop_front()?;
        let mut node = node.borrow_mut();

        if i < values.len() {
            if let Some(val) = values[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                queue.push_back(left);
            }
            i += 1;
        }

        if i < values.len() {
            if let Some(val) = values[i] {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                queue.push_back(right);
            }
            i += 1;
        }
    }

    Some(root)
}