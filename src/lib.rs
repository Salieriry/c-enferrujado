pub mod lexer;
pub mod parser;
pub mod token;

// re-exporta para facilitar o uso no Tauri
pub use lexer::Lexer;
pub use parser::{Parser, Stmt};
pub use token::Token;
