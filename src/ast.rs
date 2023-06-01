pub struct Ast {
    nodes:          Vec<Option<Node>>,
    freed_indexes:  Vec<usize>,
}

impl Ast {
    /*
        NOTE: ROOT NODE IS THE FIRST NODE (nodes[0])
    */


    pub fn new(root_node: Node) -> Self {
        Self {
            nodes:          vec![Some(root_node)],
            freed_indexes:  vec![],
        }
    }

    // Add a node & return the index
    pub fn add_node(&mut self, node: Node) -> usize {
        if let Some(index) = self.freed_indexes.pop() {
            // If there are any freed indexes, recycle!
            self.nodes[index] = Some(node)
        } else {
            // If there are no freed indexes, just add the node
            self.nodes.push(Some(node))
        }
        self.nodes.len() - 1
    }

    // Remove a node & return an error if a node with the provided index does not exist
    pub fn remove_node(&mut self, index: usize) -> Result<(), ()> {
        if index >= self.nodes.len() || self.nodes[index].is_none() {
            return Err(())
        }

        self.nodes[index] = None;

        Ok(())
    }

    // Retrieve a node from the tree
    pub fn get_node(&self, index: usize) -> Result<Node, ()> {
        if index >= self.nodes.len() || self.nodes[index].is_none() {
            return Err(())
        }

        return Ok(self.nodes[index].clone().unwrap())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    n_type:     NodeType,
    children:   Option<Vec<usize>>,
}

impl Node {
    pub fn new(n_type: NodeType) -> Self {
        Self {
            n_type,
            children: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum NodeType {
    Module
}