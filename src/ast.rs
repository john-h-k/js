#[derive(Debug)]
pub struct Statement {
    pub expr: ExpressionNode,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Literal(LiteralNode),
    Call(CallNode),
}

#[derive(Debug)]
pub enum LiteralNode {
    String(String),
}

#[derive(Debug)]
pub struct CallNode {
    pub target: String,
    pub arguments: Vec<ExpressionNode>,
}
