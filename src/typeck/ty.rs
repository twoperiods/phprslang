use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
    Int,
    Float,
    String,
    Bool,
    Void,
    Null,
    Array(Box<Ty>),
    Dict(Box<Ty>, Box<Ty>),
    Function(Vec<Ty>, Box<Ty>),
    Unknown, // For type inference before resolution
    Any,     // Dynamic type (e.g., return of json_decode) — checked at runtime
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ty::Int => write!(f, "int"),
            Ty::Float => write!(f, "float"),
            Ty::String => write!(f, "string"),
            Ty::Bool => write!(f, "bool"),
            Ty::Void => write!(f, "void"),
            Ty::Null => write!(f, "null"),
            Ty::Array(t) => write!(f, "{}[]", t),
            Ty::Dict(k, v) => write!(f, "dict<{}, {}>", k, v),
            Ty::Function(params, ret) => {
                write!(f, "fn(")?;
                for (i, p) in params.iter().enumerate() {
                    if i > 0 { write!(f, ", ")?; }
                    write!(f, "{}", p)?;
                }
                write!(f, ") -> {}", ret)
            }
            Ty::Unknown => write!(f, "?"),
            Ty::Any => write!(f, "any"),
        }
    }
}

impl Ty {
    pub fn from_ast_type(ast_ty: &crate::parser::TypeAnnotation) -> Self {
        use crate::parser::TypeAnnotation as AT;
        match ast_ty {
            AT::Int => Ty::Int,
            AT::Float => Ty::Float,
            AT::String_ => Ty::String,
            AT::Bool => Ty::Bool,
            AT::Void => Ty::Void,
            AT::Array(t) => Ty::Array(Box::new(Ty::from_ast_type(t))),
            AT::Dict(k, v) => Ty::Dict(Box::new(Ty::from_ast_type(k)), Box::new(Ty::from_ast_type(v))),
            AT::Named(s) => match s.as_str() {
                "int" => Ty::Int,
                "float" | "f64" => Ty::Float,
                "string" => Ty::String,
                "bool" => Ty::Bool,
                "void" => Ty::Void,
                "any" => Ty::Any,
                _ => Ty::Unknown,
            },
            _ => Ty::Unknown,
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(self, Ty::Int | Ty::Float | Ty::Any)
    }

    pub fn can_concat(&self) -> bool {
        true // All types can be stringified
    }
}
