pub mod c_backend;
pub mod ast_to_c;

pub use c_backend::{compile_to_c, compile_to_binary};
pub use ast_to_c::{transpile_program, CTranspiler};
