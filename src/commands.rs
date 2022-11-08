use modular_bitfield::{bitfield, BitfieldSpecifier};

pub struct Node<'a> {
    flags: NodeFlags,
    children: Vec<Node<'a>>,
    redirect_node: Option<&'a Node<'a>>,
    name: Option<String>,
    parser_id: Option<i32>,
    //Properties
    suggestions_type: Option<i32>,
}

#[bitfield(bits = 5)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct NodeFlags {
    #[bits = 2]
    pub node_type: NodeType,
    pub is_executable: bool,
    pub has_redirect: bool,
    pub has_suggestions_type: bool,
}

#[derive(BitfieldSpecifier, Debug, PartialEq)]
#[bits = 2]
pub enum NodeType {
    Root,
    Literal,
    Argument,
}
