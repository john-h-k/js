use crate::{
    ast::{Ast, Block, CallNode, ExpressionNode, Function, LiteralNode, Statement},
    interpreter::{ArgDef, FuncDef, Opcode, StringKey},
};

pub struct Lower {
    opcodes: Vec<Opcode>,
}

impl Lower {
    pub fn new() -> Self {
        Self {
            opcodes: Vec::new(),
        }
    }

    pub fn lower(mut self, name_key: StringKey, func: Function) -> FuncDef {
        self.lower_block(func.body);

        FuncDef {
            name: name_key,
            args: func.parameters.iter().map(|_| ()).collect(),
            body: self.opcodes,
        }
    }

    pub fn lower_block(&mut self, body: Block) {
        for stmt in body.stmts {
            self.lower_stmt(stmt);
        }

        if let Some(expr) = body.expr {
            self.lower_expr(*expr);
        }
    }

    pub fn lower_stmt(&mut self, stmt: Statement) {
        self.lower_expr(stmt.expr);
    }

    pub fn lower_expr(&mut self, expr: ExpressionNode) {
        match expr {
            ExpressionNode::Literal(literal) => self.lower_literal(literal),
            ExpressionNode::Block(block) => self.lower_block(block),
            ExpressionNode::Call(call) => self.lower_call(call),
        }
    }

    pub fn lower_literal(&mut self, literal: LiteralNode) {
        match literal {
            LiteralNode::String(string) => self.opcodes.push(Opcode::PushStr(string)),
        }
    }

    pub fn lower_call(&mut self, call: CallNode) {
        let arg_count = call.arguments.len();
        for arg in call.arguments {
            self.lower_expr(arg);
        }

        self.opcodes.push(Opcode::Call {
            target: call.target,
            num_args: arg_count,
        })
    }
}
