#[derive(Debug, Clone)]
pub enum GrammarItem {
    Product,
    Sum,
    Number(u64),
    Paren,
}

// A node of a parse tree
#[derive(Debug, Clone)]
pub struct ParseNode {
    pub entry: GrammarItem,
    // A tree would only have 2 nodes. However a vec was used to avoid self-referential
    // issues and the use of `Box`. Could have instead have done leftNode and rightNode.
    pub children: Vec<ParseNode>,
}

impl ParseNode {
    // Deviate from blog post by passing in the entry
    pub fn new(entry: GrammarItem) -> ParseNode {
        ParseNode {
            entry,
            children: Vec::new(),
        }
    }
}
