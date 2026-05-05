use crate::typeck::Ty;

/// Virtual register index
pub type Reg = usize;

/// Basic block index
pub type BlockId = usize;

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<(String, Ty)>,
    pub ret_ty: Ty,
    pub blocks: Vec<BasicBlock>,
    pub next_reg: Reg,
    pub next_block: BlockId,
}

#[derive(Debug, Clone)]
pub struct BasicBlock {
    pub id: BlockId,
    pub stmts: Vec<MirStmt>,
    pub term: Terminator,
}

#[derive(Debug, Clone)]
pub enum MirStmt {
    /// reg = alloca type
    Alloca { dst: Reg, ty: Ty },
    /// store value into pointer
    Store { ptr: Reg, value: Operand },
    /// reg = load ptr
    Load { dst: Reg, ptr: Reg },
    /// reg = binary op left, right
    Binary { dst: Reg, op: MirBinOp, left: Operand, right: Operand },
    /// reg = unary op operand
    Unary { dst: Reg, op: MirUnOp, operand: Operand },
    /// call func(args) -> reg
    Call { dst: Option<Reg>, func: String, args: Vec<Operand> },
    /// reg = const literal
    Const { dst: Reg, value: MirConst },
    /// print operand
    Print { value: Operand },
    /// reg = phi [ (val0, block0), (val1, block1), ... ]
    Phi { dst: Reg, incoming: Vec<(Operand, BlockId)> },
}

#[derive(Debug, Clone)]
pub enum Terminator {
    Return(Option<Operand>),
    Jump(BlockId),
    Branch { cond: Operand, then_block: BlockId, else_block: BlockId },
    Switch { value: Operand, default: BlockId, cases: Vec<(MirConst, BlockId)> },
    Throw(Operand),
    Unreachable,
}

#[derive(Debug, Clone)]
pub enum Operand {
    Reg(Reg),
    Const(MirConst),
}

#[derive(Debug, Clone)]
pub enum MirConst {
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum MirBinOp {
    Add, Sub, Mul, Div, Mod,
    Eq, Neq, StrictEq, StrictNeq, Lt, Gt, Le, Ge,
    And, Or,
    Concat,
}

#[derive(Debug, Clone)]
pub enum MirUnOp {
    Neg, Not,
}

#[derive(Debug, Clone)]
pub struct MirProgram {
    pub functions: Vec<MirFunction>,
    pub strings: Vec<String>, // String constants pool
}

impl MirFunction {
    pub fn new(name: String, params: Vec<(String, Ty)>, ret_ty: Ty) -> Self {
        Self {
            name,
            params,
            ret_ty,
            blocks: Vec::new(),
            next_reg: 0,
            next_block: 0,
        }
    }

    pub fn new_reg(&mut self) -> Reg {
        let r = self.next_reg;
        self.next_reg += 1;
        r
    }

    pub fn new_block(&mut self) -> BlockId {
        let id = self.next_block;
        self.next_block += 1;
        self.blocks.push(BasicBlock { id, stmts: Vec::new(), term: Terminator::Unreachable });
        id
    }

    pub fn block_mut(&mut self, id: BlockId) -> &mut BasicBlock {
        &mut self.blocks[id]
    }

    pub fn block(&self, id: BlockId) -> &BasicBlock {
        &self.blocks[id]
    }

    pub fn push_stmt(&mut self, block: BlockId, stmt: MirStmt) {
        self.blocks[block].stmts.push(stmt);
    }

    pub fn set_term(&mut self, block: BlockId, term: Terminator) {
        self.blocks[block].term = term;
    }
}

impl MirProgram {
    pub fn new() -> Self {
        Self { functions: Vec::new(), strings: Vec::new() }
    }

    pub fn add_string(&mut self, s: &str) -> usize {
        if let Some(pos) = self.strings.iter().position(|x| x == s) {
            pos
        } else {
            self.strings.push(s.to_string());
            self.strings.len() - 1
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Reg(r) => write!(f, "%{}", r),
            Operand::Const(c) => write!(f, "{}", c),
        }
    }
}

impl std::fmt::Display for MirConst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MirConst::Int(n) => write!(f, "{}", n),
            MirConst::Float(n) => write!(f, "{}", n),
            MirConst::String(s) => write!(f, "\"{}\"", s),
            MirConst::Bool(b) => write!(f, "{}", b),
            MirConst::Null => write!(f, "null"),
        }
    }
}
