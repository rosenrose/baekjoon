#[derive(Debug, Clone)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new(val: i32) -> TreeNode {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn infix_traversal(root: &Option<Box<TreeNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut stack = vec![];
    let mut node = root;

    while !stack.is_empty() || node.is_some() {
        while let Some(n) = node {
            stack.push(n);
            node = &n.left;
        }

        if let Some(n) = stack.pop() {
            result.push(n.val);
            node = &n.right;
        }
    }

    result
}

fn main() {
    let root = Some(Box::new(TreeNode {
        val: 1,
        left: Some(Box::new(TreeNode::new(2))),
        right: Some(Box::new(TreeNode::new(3))),
    }));

    let result = infix_traversal(&root);
    println!("{:?}", result);
}
