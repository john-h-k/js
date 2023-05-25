#[derive(Debug)]
pub struct Ast {
    pub root: Module,
}

#[derive(Debug)]
pub struct Module {
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub body: Block,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: String,
    pub r#type: String,
}

#[derive(Debug)]
pub struct Block {
    pub stmts: Vec<Statement>,
    pub expr: Option<Box<ExpressionNode>>,
}

#[derive(Debug)]
pub struct Statement {
    pub expr: ExpressionNode,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Block(Block),
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
