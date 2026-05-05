use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Let,
    Mut,
    Fn,
    Function,
    Extern,
    If,
    Else,
    For,
    While,
    Do,
    Foreach,
    Return,
    Echo,
    Struct,
    Enum,
    Match,
    Use,
    Mod,
    Pub,
    Const,
    Async,
    Await,
    Break,
    Continue,
    Try,
    Catch,
    Throw,
    Elseif,
    As,
    In,
    Move,
    True,
    False,
    Null,

    // Identifiers
    Ident(String),
    Var(String), // $variable

    // Literals
    Int(i64),
    Float(f64),
    String_(String),

    // Operators
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    Percent,   // %
    Dot,       // .
    DotEq,     // .=
    Eq,        // =
    PlusEq,    // +=
    MinusEq,   // -=
    StarEq,    // *=
    SlashEq,   // /=
    PercentEq, // %=
    EqEq,      // ==
    EqEqEq,    // ===
    Neq,       // !=
    NeqEq,     // !==
    Lt,        // <
    Gt,        // >
    Le,        // <=
    Ge,        // >=
    AndAnd,    // &&
    OrOr,      // ||
    Not,       // !
    FatArrow,  // =>
    ThinArrow, // ->
    Colon,     // :
    ColonColon,// ::
    Semi,      // ;
    Comma,     // ,
    DotDot,    // ..
    DotDotEq,  // ..=
    Question,  // ?
    Inc,       // ++
    Dec,       // --

    // Delimiters
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,

    // Special
    OpenTag,  // <?phprs
    CloseTag, // ?>
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub col: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, col: usize) -> Self {
        Self { kind, line, col }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Let => write!(f, "let"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::Fn => write!(f, "fn"),
            TokenKind::Function => write!(f, "function"),
            TokenKind::Extern => write!(f, "extern"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::For => write!(f, "for"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Do => write!(f, "do"),
            TokenKind::Foreach => write!(f, "foreach"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Echo => write!(f, "echo"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Enum => write!(f, "enum"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::Use => write!(f, "use"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Pub => write!(f, "pub"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Async => write!(f, "async"),
            TokenKind::Await => write!(f, "await"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Try => write!(f, "try"),
            TokenKind::Catch => write!(f, "catch"),
            TokenKind::Throw => write!(f, "throw"),
            TokenKind::Elseif => write!(f, "elseif"),
            TokenKind::As => write!(f, "as"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Move => write!(f, "move"),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::Var(s) => write!(f, "${}", s),
            TokenKind::Int(n) => write!(f, "{}", n),
            TokenKind::Float(n) => write!(f, "{}", n),
            TokenKind::String_(s) => write!(f, "\"{}\"", s),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::Dot => write!(f, "."),
            TokenKind::DotEq => write!(f, ".="),
            TokenKind::Eq => write!(f, "="),
            TokenKind::PlusEq => write!(f, "+="),
            TokenKind::MinusEq => write!(f, "-="),
            TokenKind::StarEq => write!(f, "*="),
            TokenKind::SlashEq => write!(f, "/="),
            TokenKind::PercentEq => write!(f, "%="),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::EqEqEq => write!(f, "==="),
            TokenKind::Neq => write!(f, "!="),
            TokenKind::NeqEq => write!(f, "!=="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Le => write!(f, "<="),
            TokenKind::Ge => write!(f, ">="),
            TokenKind::AndAnd => write!(f, "&&"),
            TokenKind::OrOr => write!(f, "||"),
            TokenKind::Not => write!(f, "!"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::ThinArrow => write!(f, "->"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::ColonColon => write!(f, "::"),
            TokenKind::Semi => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::DotDot => write!(f, ".."),
            TokenKind::DotDotEq => write!(f, "..="),
            TokenKind::Question => write!(f, "?"),
            TokenKind::Inc => write!(f, "++"),
            TokenKind::Dec => write!(f, "--"),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::OpenTag => write!(f, "<?phprs"),
            TokenKind::CloseTag => write!(f, "?>"),
            TokenKind::Eof => write!(f, "EOF"),
        }
    }
}
