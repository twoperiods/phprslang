use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeAnnotation {
    Int,
    Float,
    String_,
    Bool,
    Void,
    Array(Box<TypeAnnotation>),
    Dict(Box<TypeAnnotation>, Box<TypeAnnotation>),
    Named(String),
    Generic(String, Vec<TypeAnnotation>),
    Result_(Box<TypeAnnotation>, Box<TypeAnnotation>),
    Option_(Box<TypeAnnotation>),
    Ref(Box<TypeAnnotation>),
    MutRef(Box<TypeAnnotation>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnParam {
    pub name: String,
    pub ty: Option<TypeAnnotation>,
    pub by_ref: bool,
    pub by_mut_ref: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Float(f64),
    String_(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Concat,
    Eq,
    Neq,
    StrictEq,
    StrictNeq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        right: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
    },
    FieldAccess {
        target: Box<Expr>,
        field: String,
    },
    Array(Vec<Expr>),
    Dict(Vec<(Expr, Expr)>), // key => value pairs
    Assign {
        target: Box<Expr>,
        op: Option<BinaryOp>, // None = plain =, Some = compound (+=, .=, etc.)
        value: Box<Expr>,
    },
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        inclusive: bool,
    },
    Closure {
        params: Vec<FnParam>,
        return_type: Option<TypeAnnotation>,
        body: Box<Stmt>,
    },
    MatchExpr {
        expr: Box<Expr>,
        arms: Vec<MatchArm>,
    },
    IncDec {
        target: Box<Expr>,
        is_inc: bool,    // true = ++, false = --
        is_prefix: bool, // true = ++$x, false = $x++
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    Literal(Literal),
    Variable(String),
    Range { start: Option<Box<Expr>>, end: Option<Box<Expr>>, inclusive: bool },
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Let {
        name: String,
        ty: Option<TypeAnnotation>,
        value: Option<Expr>,
        mutable: bool,
    },
    Function {
        name: String,
        params: Vec<FnParam>,
        return_type: Option<TypeAnnotation>,
        body: Box<Stmt>,
    },
    ExternFunction {
        name: String,
        params: Vec<FnParam>,
        return_type: Option<TypeAnnotation>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    For {
        init: Box<Stmt>,
        condition: Option<Expr>,
        update: Option<Expr>,
        body: Box<Stmt>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    DoWhile {
        body: Box<Stmt>,
        condition: Expr,
    },
    Foreach {
        iterable: Expr,
        key_var: Option<String>,
        value_var: String,
        body: Box<Stmt>,
    },
    Return(Option<Expr>),
    Break,
    Continue,
    Throw(Expr),
    TryCatch {
        try_body: Box<Stmt>,
        catch_var: String,
        catch_body: Box<Stmt>,
    },
    Echo(Expr),
    ExprStmt(Expr),
    Block(Vec<Stmt>),
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
    StructDef {
        name: String,
        fields: Vec<(String, TypeAnnotation)>,
    },
    EnumDef {
        name: String,
        variants: Vec<(String, Vec<TypeAnnotation>)>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }
}

impl fmt::Display for TypeAnnotation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeAnnotation::Int => write!(f, "int"),
            TypeAnnotation::Float => write!(f, "f64"),
            TypeAnnotation::String_ => write!(f, "string"),
            TypeAnnotation::Bool => write!(f, "bool"),
            TypeAnnotation::Void => write!(f, "void"),
            TypeAnnotation::Array(t) => write!(f, "[{}]", t),
            TypeAnnotation::Dict(k, v) => write!(f, "[{}: {}]", k, v),
            TypeAnnotation::Named(n) => write!(f, "{}", n),
            TypeAnnotation::Generic(n, args) => {
                write!(f, "{}<", n)?;
                for (i, a) in args.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", a)?;
                }
                write!(f, ">")
            }
            TypeAnnotation::Result_(ok, err) => write!(f, "Result<{}, {}>", ok, err),
            TypeAnnotation::Option_(t) => write!(f, "Option<{}>", t),
            TypeAnnotation::Ref(t) => write!(f, "&{}", t),
            TypeAnnotation::MutRef(t) => write!(f, "&mut {}", t),
        }
    }
}
