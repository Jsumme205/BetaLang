use crate::common::{syntax_tree::{BinOp, UniOp}, yarn::Yarn, Constants};



pub(super) enum Value {
    Predefined(Constants),
    ProgramDefined(usize),
    Undefined
}

pub(super) enum OpNode {
    BinaryOp {
        lhs: Box<OpNode>,
        rhs: Box<OpNode>,
        op: BinOp
    },
    UnaryOp {
        lhs: Box<OpNode>,
        op: UniOp
    },
    Value {
        inner: Value
    }
}

pub(super) enum Type {

}

pub(super) struct Token<'a> {
    ty: Type,
    chunk: &'a Chunk<'a>,
}

impl OpNode {
    pub(super) fn from_yarn<'a>(string: &Yarn<'a>) -> Self {
        todo!()
    }
}

