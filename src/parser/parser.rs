use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::*;

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn cur(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or_else(|| {
            // Return a dummy EOF token if we're past the end
            &self.tokens[self.tokens.len() - 1]
        })
    }

    fn peek(&self) -> TokenKind {
        self.cur().kind.clone()
    }

    fn advance(&mut self) -> Token {
        let t = self.cur().clone();
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        t
    }

    fn expect(&mut self, kind: TokenKind, msg: &str) -> Result<Token, String> {
        if self.peek() == kind {
            Ok(self.advance())
        } else {
            let t = self.cur();
            Err(format!("{} at line {}: expected {}, got {}", msg, t.line, kind, t.kind))
        }
    }

    fn skip_semis(&mut self) {
        while self.peek() == TokenKind::Semi {
            self.advance();
        }
    }

    // program → stmt*
    pub fn parse_program(&mut self) -> Result<Program, String> {
        // Skip opening tag if present
        if self.peek() == TokenKind::OpenTag {
            self.advance();
        }

        let mut stmts = Vec::new();
        while self.peek() != TokenKind::Eof && self.peek() != TokenKind::CloseTag {
            stmts.push(self.parse_stmt()?);
        }
        Ok(Program::new(stmts))
    }

    // stmt → let_stmt | function | if_stmt | for_stmt | while_stmt
    //       | foreach_stmt | return_stmt | echo_stmt | expr_stmt
    //       | block | struct_def | enum_def
    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            TokenKind::Let => self.parse_let(),
            TokenKind::Function => self.parse_function(),
            TokenKind::Extern => {
                self.advance(); // consume 'extern'
                // Expect 'function' keyword next
                if self.peek() != TokenKind::Function {
                    return Err(format!("Expected 'function' after 'extern' at line {}", self.cur().line));
                }
                self.parse_function() // parse_function will detect ';' -> ExternFunction
            }
            TokenKind::If => self.parse_if(),
            TokenKind::For => self.parse_for(),
            TokenKind::While => self.parse_while(),
            TokenKind::Do => self.parse_do_while(),
            TokenKind::Foreach => self.parse_foreach(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Break => self.parse_break_continue(true),
            TokenKind::Continue => self.parse_break_continue(false),
            TokenKind::Throw => self.parse_throw(),
            TokenKind::Try => self.parse_try_catch(),
            TokenKind::Echo => self.parse_echo(),
            TokenKind::Struct => self.parse_struct_def(),
            TokenKind::Enum => self.parse_enum_def(),
            TokenKind::Match => self.parse_match_stmt(),
            TokenKind::LBrace => self.parse_block(),
            _ => self.parse_expr_stmt(),
        }
    }

    // let_stmt → "let" "mut"? "$" IDENT (":" type)? ("=" expr)? ";"
    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.advance(); // let

        let mutable = if self.peek() == TokenKind::Mut {
            self.advance();
            true
        } else {
            false
        };

        let name = match self.peek() {
            TokenKind::Var(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected $variable after 'let' at line {}", self.cur().line)),
        };

        let ty = if self.peek() == TokenKind::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let value = if self.peek() == TokenKind::Eq {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        // Optional semicolon (for both stmt and for-loop init contexts)
        if self.peek() == TokenKind::Semi {
            self.advance();
        }

        Ok(Stmt::Let { name, ty, value, mutable })
    }

    // function → "function" IDENT "(" params? ")" (":" type)? block
    fn parse_function(&mut self) -> Result<Stmt, String> {
        self.advance(); // function

        let name = match self.peek() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected function name at line {}", self.cur().line)),
        };

        self.expect(TokenKind::LParen, "Expected '(' after function name")?;
        let params = self.parse_params()?;
        self.expect(TokenKind::RParen, "Expected ')' after params")?;

        let return_type = if self.peek() == TokenKind::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        // Check for extern function (terminated by ';') vs regular function (has '{' body)
        if self.peek() == TokenKind::Semi {
            self.advance(); // consume ';'
            Ok(Stmt::ExternFunction { name, params, return_type })
        } else {
            let body = self.parse_block()?;
            Ok(Stmt::Function { name, params, return_type, body: Box::new(body) })
        }
    }

    // params → param ("," param)*
    fn parse_params(&mut self) -> Result<Vec<FnParam>, String> {
        let mut params = Vec::new();
        if self.peek() == TokenKind::RParen {
            return Ok(params);
        }
        loop {
            params.push(self.parse_param()?);
            if self.peek() == TokenKind::Comma {
                self.advance();
            } else {
                break;
            }
        }
        Ok(params)
    }

    // param → type? "$" IDENT (":" type)?
    // Supports: "int $a" (PHP style) and "$a: int" (Rust style)
    fn parse_param(&mut self) -> Result<FnParam, String> {
        let by_ref = false;
        let by_mut_ref = false;

        // Consume 'move' keyword if present
        if self.peek() == TokenKind::Move {
            self.advance();
        }

        // PHP style: type before $variable (e.g., int $a)
        let first_ty = if self.peek() != TokenKind::Var(String::new())
            && self.peek() != TokenKind::RParen
            && self.peek() != TokenKind::Comma
        {
            // Try to parse as type; fall back if it fails
            self.parse_type().ok()
        } else {
            None
        };

        let name = match self.peek() {
            TokenKind::Var(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected $variable in parameter at line {}", self.cur().line)),
        };

        // Rust style type annotation: $var: type
        let ty = if self.peek() == TokenKind::Colon {
            self.advance();
            Some(self.parse_type()?)
        } else {
            first_ty
        };

        Ok(FnParam { name, ty, by_ref, by_mut_ref })
    }

    // if_stmt → "if" "(" expr ")" stmt ("else" ("if" "(" expr ")" stmt)* ("else" stmt)?)?
    //         | "elseif" "(" expr ")" stmt  (desugars to else { if ... })
    fn parse_if(&mut self) -> Result<Stmt, String> {
        self.advance(); // if
        let (condition, then_branch) = self.parse_if_body()?;

        let else_branch = self.parse_else_or_elseif()?;

        Ok(Stmt::If { condition, then_branch, else_branch })
    }

    // Parse the body of an if (condition + stmt) without consuming the 'if' keyword
    fn parse_if_body(&mut self) -> Result<(Expr, Box<Stmt>), String> {
        self.expect(TokenKind::LParen, "Expected '(' after 'if'")?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::RParen, "Expected ')' after condition")?;
        let then_branch = Box::new(self.parse_stmt()?);
        Ok((condition, then_branch))
    }

    // Parse optional else / elseif clause
    fn parse_else_or_elseif(&mut self) -> Result<Option<Box<Stmt>>, String> {
        if self.peek() == TokenKind::Else {
            self.advance();
            if self.peek() == TokenKind::If {
                // else if → recursively parse
                Ok(Some(Box::new(self.parse_if()?)))
            } else {
                Ok(Some(Box::new(self.parse_stmt()?)))
            }
        } else if self.peek() == TokenKind::Elseif {
            self.advance(); // consume 'elseif'
            // Desugar: elseif (cond) body → else { if (cond) body (no else) }
            let (condition, then_branch) = self.parse_if_body()?;
            let if_stmt = Stmt::If { condition, then_branch, else_branch: None };
            // The else branch should also check for further elseif/else chaining
            let if_stmt = self.maybe_chain_elseif(if_stmt)?;
            Ok(Some(Box::new(if_stmt)))
        } else {
            Ok(None)
        }
    }

    // Chain elseif/else onto an existing if statement
    fn maybe_chain_elseif(&mut self, if_stmt: Stmt) -> Result<Stmt, String> {
        match self.peek() {
            TokenKind::Else => {
                self.advance();
                let else_branch = if self.peek() == TokenKind::If {
                    Some(Box::new(self.parse_if()?))
                } else {
                    Some(Box::new(self.parse_stmt()?))
                };
                match if_stmt {
                    Stmt::If { condition, then_branch, .. } => {
                        Ok(Stmt::If { condition, then_branch, else_branch })
                    }
                    _ => Ok(if_stmt),
                }
            }
            TokenKind::Elseif => {
                self.advance(); // consume 'elseif'
                let (condition, then_branch) = self.parse_if_body()?;
                let inner_if = Stmt::If { condition, then_branch, else_branch: None };
                let chained = self.maybe_chain_elseif(inner_if)?;
                match if_stmt {
                    Stmt::If { condition, then_branch, .. } => {
                        Ok(Stmt::If { condition, then_branch, else_branch: Some(Box::new(chained)) })
                    }
                    _ => Ok(if_stmt),
                }
            }
            _ => Ok(if_stmt),
        }
    }

    // for_stmt → "for" "(" stmt expr ";" expr? ")" stmt
    fn parse_for(&mut self) -> Result<Stmt, String> {
        self.advance(); // for
        self.expect(TokenKind::LParen, "Expected '(' after 'for'")?;

        let init = Box::new(self.parse_stmt()?);

        let condition = if self.peek() == TokenKind::Semi {
            self.advance();
            None
        } else {
            let cond = Some(self.parse_expr()?);
            self.expect(TokenKind::Semi, "Expected ';' after for condition")?;
            cond
        };

        let update = if self.peek() == TokenKind::RParen {
            None
        } else {
            let upd = Some(self.parse_expr()?);
            self.expect(TokenKind::RParen, "Expected ')' after for update")?;
            upd
        };

        // Handle case where we already consumed RParen above
        if update.is_some() {
            // RParen already consumed by expect
        } else {
            self.expect(TokenKind::RParen, "Expected ')' after for clauses")?;
        }

        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::For { init, condition, update, body })
    }

    // while_stmt → "while" "(" expr ")" stmt
    fn parse_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // while
        self.expect(TokenKind::LParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::RParen, "Expected ')' after condition")?;
        let body = Box::new(self.parse_stmt()?);
        Ok(Stmt::While { condition, body })
    }

    // do_while → "do" stmt "while" "(" expr ")" ";"
    fn parse_do_while(&mut self) -> Result<Stmt, String> {
        self.advance(); // do
        let body = Box::new(self.parse_stmt()?);
        self.expect(TokenKind::While, "Expected 'while' after 'do' body")?;
        self.expect(TokenKind::LParen, "Expected '(' after 'while'")?;
        let condition = self.parse_expr()?;
        self.expect(TokenKind::RParen, "Expected ')' after do-while condition")?;
        self.skip_semis();
        Ok(Stmt::DoWhile { body, condition })
    }

    // foreach → "foreach" "(" expr "as" "$" IDENT ("=>" "$" IDENT)? ")" stmt
    fn parse_foreach(&mut self) -> Result<Stmt, String> {
        self.advance(); // foreach
        self.expect(TokenKind::LParen, "Expected '(' after 'foreach'")?;
        let iterable = self.parse_expr()?;
        self.expect(TokenKind::As, "Expected 'as' in foreach")?;

        let first = match self.peek() {
            TokenKind::Var(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected $variable after 'as' at line {}", self.cur().line)),
        };

        let (key_var, value_var) = if self.peek() == TokenKind::FatArrow {
            self.advance();
            let val = match self.peek() {
                TokenKind::Var(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(format!("Expected $variable after '=>' at line {}", self.cur().line)),
            };
            (Some(first), val)
        } else {
            (None, first)
        };

        self.expect(TokenKind::RParen, "Expected ')' after foreach")?;
        let body = Box::new(self.parse_stmt()?);

        Ok(Stmt::Foreach { iterable, key_var, value_var, body })
    }

    // return → "return" expr? ";"
    fn parse_return(&mut self) -> Result<Stmt, String> {
        self.advance(); // return
        let value = if self.peek() == TokenKind::Semi {
            None
        } else {
            let v = Some(self.parse_expr()?);
            // Optional semicolon
            if self.peek() == TokenKind::Semi {
                self.advance();
            }
            return Ok(Stmt::Return(v));
        };
        self.skip_semis();
        Ok(Stmt::Return(value))
    }

    // break → "break" ";"
    // continue → "continue" ";"
    fn parse_break_continue(&mut self, is_break: bool) -> Result<Stmt, String> {
        self.advance(); // break or continue
        self.skip_semis();
        if is_break {
            Ok(Stmt::Break)
        } else {
            Ok(Stmt::Continue)
        }
    }

    // throw → "throw" expr ";"
    fn parse_throw(&mut self) -> Result<Stmt, String> {
        self.advance(); // throw
        let expr = self.parse_expr()?;
        self.skip_semis();
        Ok(Stmt::Throw(expr))
    }

    // try_catch → "try" block "catch" "(" "$" IDENT ")" block
    fn parse_try_catch(&mut self) -> Result<Stmt, String> {
        self.advance(); // try
        let try_body = self.parse_block()?;

        // Expect 'catch'
        if self.peek() != TokenKind::Catch {
            return Err(format!("Expected 'catch' after try block at line {}", self.cur().line));
        }
        self.advance(); // catch

        // Expect '('
        if self.peek() != TokenKind::LParen {
            return Err(format!("Expected '(' after 'catch' at line {}", self.cur().line));
        }
        self.advance(); // (

        // Expect $variable
        let catch_var = match self.peek() {
            TokenKind::Var(n) => {
                let name = n.clone();
                self.advance();
                name
            }
            _ => return Err(format!("Expected $variable in catch clause at line {}", self.cur().line)),
        };

        // Expect ')'
        if self.peek() != TokenKind::RParen {
            return Err(format!("Expected ')' after catch variable at line {}", self.cur().line));
        }
        self.advance(); // )

        let catch_body = self.parse_block()?;

        Ok(Stmt::TryCatch {
            try_body: Box::new(try_body),
            catch_var,
            catch_body: Box::new(catch_body),
        })
    }

    // echo → "echo" expr ";"
    fn parse_echo(&mut self) -> Result<Stmt, String> {
        self.advance(); // echo
        let expr = self.parse_expr()?;
        self.skip_semis();
        Ok(Stmt::Echo(expr))
    }

    // expr_stmt → expr ";"
    fn parse_expr_stmt(&mut self) -> Result<Stmt, String> {
        let expr = self.parse_expr()?;
        self.skip_semis();
        Ok(Stmt::ExprStmt(expr))
    }

    // block → "{" stmt* "}"
    fn parse_block(&mut self) -> Result<Stmt, String> {
        self.expect(TokenKind::LBrace, "Expected '{'")?;
        let mut stmts = Vec::new();
        while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
            stmts.push(self.parse_stmt()?);
        }
        self.expect(TokenKind::RBrace, "Expected '}'")?;
        Ok(Stmt::Block(stmts))
    }

    // struct_def → "struct" IDENT "{" (IDENT ":" type ","?)* "}"
    fn parse_struct_def(&mut self) -> Result<Stmt, String> {
        self.advance(); // struct
        let name = match self.peek() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected struct name at line {}", self.cur().line)),
        };
        self.expect(TokenKind::LBrace, "Expected '{' in struct")?;
        let mut fields = Vec::new();
        while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
            let field_name = match self.peek() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(format!("Expected field name in struct at line {}", self.cur().line)),
            };
            self.expect(TokenKind::Colon, "Expected ':' after field name")?;
            let ty = self.parse_type()?;
            fields.push((field_name, ty));
            if self.peek() == TokenKind::Comma {
                self.advance();
            }
        }
        self.expect(TokenKind::RBrace, "Expected '}' in struct")?;
        Ok(Stmt::StructDef { name, fields })
    }

    // enum_def → "enum" IDENT "{" (IDENT ("(" type ("," type)* ")")? ","?)* "}"
    fn parse_enum_def(&mut self) -> Result<Stmt, String> {
        self.advance(); // enum
        let name = match self.peek() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(format!("Expected enum name at line {}", self.cur().line)),
        };
        self.expect(TokenKind::LBrace, "Expected '{' in enum")?;
        let mut variants = Vec::new();
        while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
            let variant_name = match self.peek() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(format!("Expected variant name at line {}", self.cur().line)),
            };

            let types = if self.peek() == TokenKind::LParen {
                self.advance();
                let mut types = Vec::new();
                while self.peek() != TokenKind::RParen && self.peek() != TokenKind::Eof {
                    types.push(self.parse_type()?);
                    if self.peek() == TokenKind::Comma {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RParen, "Expected ')' after enum variant")?;
                types
            } else {
                Vec::new()
            };

            variants.push((variant_name, types));
            if self.peek() == TokenKind::Comma {
                self.advance();
            }
        }
        self.expect(TokenKind::RBrace, "Expected '}' in enum")?;
        Ok(Stmt::EnumDef { name, variants })
    }

    // match_stmt → "match" "(" expr ")" "{" match_arm* "}"
    #[allow(dead_code)]
    fn parse_match_expr(&mut self) -> Result<Expr, String> {
        self.advance(); // match
        self.expect(TokenKind::LParen, "Expected '(' after 'match'")?;
        let _expr = self.parse_expr()?;
        self.expect(TokenKind::RParen, "Expected ')' after match expr")?;
        self.expect(TokenKind::LBrace, "Expected '{' in match")?;

        let mut arms = Vec::new();
        while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
            let pattern = self.parse_match_pattern()?;
            self.expect(TokenKind::FatArrow, "Expected '=>' in match arm")?;
            let body = self.parse_expr()?;
            arms.push(MatchArm { pattern, body });
            if self.peek() == TokenKind::Comma {
                self.advance();
            }
        }
        self.expect(TokenKind::RBrace, "Expected '}' after match arms")?;
        // Match expression is wrapped as a Stmt::Match in parse_expr
        // Return a placeholder, will be handled in primary
        // Actually, let's handle this differently - return as a special expr
        // For now, we'll use a dummy and handle in parse_stmt instead
        unreachable!("match should be parsed via parse_match_stmt")
    }

    fn parse_match_stmt(&mut self) -> Result<Stmt, String> {
        self.advance(); // match
        self.expect(TokenKind::LParen, "Expected '(' after 'match'")?;
        let expr = self.parse_expr()?;
        self.expect(TokenKind::RParen, "Expected ')' after match expr")?;
        self.expect(TokenKind::LBrace, "Expected '{' in match")?;

        let mut arms = Vec::new();
        while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
            let pattern = self.parse_match_pattern()?;
            self.expect(TokenKind::FatArrow, "Expected '=>' in match arm")?;
            let body = self.parse_expr()?;
            arms.push(MatchArm { pattern, body });
            if self.peek() == TokenKind::Comma {
                self.advance();
            }
        }
        self.expect(TokenKind::RBrace, "Expected '}' after match arms")?;
        Ok(Stmt::Match { expr, arms })
    }

    fn parse_match_pattern(&mut self) -> Result<MatchPattern, String> {
        // Check for wildcard
        if self.peek() == TokenKind::Ident(String::new()) {
            // Check if it's a single underscore (wildcard)
        }

        // Save position in case we need to backtrack
        let saved_pos = self.pos;

        // Try to parse as a simple primary expression first
        let primary = self.parse_primary().ok();

        match self.peek() {
            TokenKind::DotDot => {
                self.advance();
                let end = self.parse_expr().ok().map(Box::new);
                Ok(MatchPattern::Range { start: primary.map(Box::new), end, inclusive: false })
            }
            TokenKind::DotDotEq => {
                self.advance();
                let end = self.parse_expr().ok().map(Box::new);
                Ok(MatchPattern::Range { start: primary.map(Box::new), end, inclusive: true })
            }
            _ => {
                // It was a regular literal or variable, not a range
                match primary {
                    Some(Expr::Literal(Literal::Int(n))) => Ok(MatchPattern::Literal(Literal::Int(n))),
                    Some(Expr::Literal(Literal::Float(n))) => Ok(MatchPattern::Literal(Literal::Float(n))),
                    Some(Expr::Literal(Literal::String_(s))) => Ok(MatchPattern::Literal(Literal::String_(s))),
                    Some(Expr::Literal(Literal::Bool(b))) => Ok(MatchPattern::Literal(Literal::Bool(b))),
                    Some(Expr::Literal(Literal::Null)) => Ok(MatchPattern::Literal(Literal::Null)),
                    Some(Expr::Variable(name)) => {
                        if name == "_" {
                            Ok(MatchPattern::Wildcard)
                        } else {
                            Ok(MatchPattern::Variable(name))
                        }
                    }
                    _ => {
                        // Not a valid pattern
                        self.pos = saved_pos;
                        Err(format!("Invalid match pattern at line {}", self.cur().line))
                    }
                }
            }
        }
    }

    // Expression parsing by precedence (Pratt-style for expressions)
    //
    // assignment → target ("=" | "+=" | "-=" | "*=" | "/=" | ".=" | "%=") assignment | or_expr
    // or_expr    → and_expr ("||" and_expr)*
    // and_expr   → eq_expr ("&&" eq_expr)*
    // eq_expr    → comp_expr (("==" | "!=") comp_expr)*
    // comp_expr  → concat_expr (("<" | ">" | "<=" | ">=") concat_expr)*
    // concat_expr → add_expr ("." add_expr)*
    // add_expr   → mul_expr (("+" | "-") mul_expr)*
    // mul_expr   → unary_expr (("*" | "/" | "%") unary_expr)*
    // unary_expr → ("!" | "-") unary_expr | call_expr
    // call_expr  → primary ("(" args? ")")? ("[" expr "]")? ("." IDENT)*

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expr, String> {
        let target = self.parse_or()?;

        match self.peek() {
            TokenKind::Eq
            | TokenKind::PlusEq
            | TokenKind::MinusEq
            | TokenKind::StarEq
            | TokenKind::SlashEq
            | TokenKind::DotEq
            | TokenKind::PercentEq => {
                let op = match self.peek() {
                    TokenKind::Eq => None,
                    TokenKind::PlusEq => Some(BinaryOp::Add),
                    TokenKind::MinusEq => Some(BinaryOp::Sub),
                    TokenKind::StarEq => Some(BinaryOp::Mul),
                    TokenKind::SlashEq => Some(BinaryOp::Div),
                    TokenKind::DotEq => Some(BinaryOp::Concat),
                    TokenKind::PercentEq => Some(BinaryOp::Mod),
                    _ => unreachable!(),
                };
                self.advance();
                let value = self.parse_assignment()?;
                Ok(Expr::Assign { target: Box::new(target), op, value: Box::new(value) })
            }
            _ => Ok(target),
        }
    }

    fn parse_or(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and()?;
        while self.peek() == TokenKind::OrOr {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::Or, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_eq()?;
        while self.peek() == TokenKind::AndAnd {
            self.advance();
            let right = self.parse_eq()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::And, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_eq(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_compare()?;
        loop {
            match self.peek() {
                TokenKind::EqEq => {
                    self.advance();
                    let right = self.parse_compare()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Eq, right: Box::new(right) };
                }
                TokenKind::EqEqEq => {
                    self.advance();
                    let right = self.parse_compare()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::StrictEq, right: Box::new(right) };
                }
                TokenKind::Neq => {
                    self.advance();
                    let right = self.parse_compare()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Neq, right: Box::new(right) };
                }
                TokenKind::NeqEq => {
                    self.advance();
                    let right = self.parse_compare()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::StrictNeq, right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_compare(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_concat()?;
        loop {
            match self.peek() {
                TokenKind::Lt => {
                    self.advance();
                    let right = self.parse_concat()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Lt, right: Box::new(right) };
                }
                TokenKind::Gt => {
                    self.advance();
                    let right = self.parse_concat()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Gt, right: Box::new(right) };
                }
                TokenKind::Le => {
                    self.advance();
                    let right = self.parse_concat()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Le, right: Box::new(right) };
                }
                TokenKind::Ge => {
                    self.advance();
                    let right = self.parse_concat()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Ge, right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_concat(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_add()?;
        while self.peek() == TokenKind::Dot {
            self.advance();
            let right = self.parse_add()?;
            left = Expr::Binary { left: Box::new(left), op: BinaryOp::Concat, right: Box::new(right) };
        }
        Ok(left)
    }

    fn parse_add(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_mul()?;
        loop {
            match self.peek() {
                TokenKind::Plus => {
                    self.advance();
                    let right = self.parse_mul()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Add, right: Box::new(right) };
                }
                TokenKind::Minus => {
                    self.advance();
                    let right = self.parse_mul()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Sub, right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_mul(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_unary()?;
        loop {
            match self.peek() {
                TokenKind::Star => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Mul, right: Box::new(right) };
                }
                TokenKind::Slash => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Div, right: Box::new(right) };
                }
                TokenKind::Percent => {
                    self.advance();
                    let right = self.parse_unary()?;
                    left = Expr::Binary { left: Box::new(left), op: BinaryOp::Mod, right: Box::new(right) };
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            TokenKind::Minus => {
                self.advance();
                let right = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Neg, right: Box::new(right) })
            }
            TokenKind::Not => {
                self.advance();
                let right = self.parse_unary()?;
                Ok(Expr::Unary { op: UnaryOp::Not, right: Box::new(right) })
            }
            TokenKind::Inc => {
                self.advance();
                let target = self.parse_unary()?;
                Ok(Expr::IncDec { target: Box::new(target), is_inc: true, is_prefix: true })
            }
            TokenKind::Dec => {
                self.advance();
                let target = self.parse_unary()?;
                Ok(Expr::IncDec { target: Box::new(target), is_inc: false, is_prefix: true })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;

        loop {
            match self.peek() {
                TokenKind::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if self.peek() != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.peek() == TokenKind::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.expect(TokenKind::RParen, "Expected ')' after call args")?;
                    expr = Expr::Call { callee: Box::new(expr), args };
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBracket, "Expected ']' after index")?;
                    expr = Expr::Index { target: Box::new(expr), index: Box::new(index) };
                }
                TokenKind::ThinArrow => {
                    self.advance(); // ->
                    let field = match self.peek() {
                        TokenKind::Ident(n) => {
                            let n = n.clone();
                            self.advance();
                            n
                        }
                        TokenKind::Var(n) => {
                            let n = n.clone();
                            self.advance();
                            n
                        }
                        _ => return Err(format!("Expected field name after '->' at line {}", self.cur().line)),
                    };
                    expr = Expr::FieldAccess { target: Box::new(expr), field };
                }
                TokenKind::Inc => {
                    self.advance();
                    expr = Expr::IncDec { target: Box::new(expr), is_inc: true, is_prefix: false };
                }
                TokenKind::Dec => {
                    self.advance();
                    expr = Expr::IncDec { target: Box::new(expr), is_inc: false, is_prefix: false };
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek() {
            TokenKind::Int(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Int(n)))
            }
            TokenKind::Float(n) => {
                self.advance();
                Ok(Expr::Literal(Literal::Float(n)))
            }
            TokenKind::String_(s) => {
                self.advance();
                Ok(Expr::Literal(Literal::String_(s)))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false)))
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expr::Literal(Literal::Null))
            }
            TokenKind::Var(n) => {
                self.advance();
                Ok(Expr::Variable(n))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen, "Expected ')'")?;
                Ok(expr)
            }
            TokenKind::LBracket => {
                self.advance();
                // Check if it's an empty array or dict
                if self.peek() == TokenKind::RBracket {
                    self.advance();
                    return Ok(Expr::Array(Vec::new()));
                }

                // Check if first element has => (dict) or not (array)
                let first = self.parse_expr()?;
                if self.peek() == TokenKind::FatArrow {
                    // Dict
                    self.advance();
                    let first_val = self.parse_expr()?;
                    let mut pairs = vec![(first, first_val)];
                    while self.peek() == TokenKind::Comma {
                        self.advance();
                        if self.peek() == TokenKind::RBracket {
                            break;
                        }
                        let key = self.parse_expr()?;
                        self.expect(TokenKind::FatArrow, "Expected '=>' in dict literal")?;
                        let val = self.parse_expr()?;
                        pairs.push((key, val));
                    }
                    self.expect(TokenKind::RBracket, "Expected ']'")?;
                    Ok(Expr::Dict(pairs))
                } else {
                    // Array
                    let mut items = vec![first];
                    while self.peek() == TokenKind::Comma {
                        self.advance();
                        if self.peek() == TokenKind::RBracket {
                            break;
                        }
                        items.push(self.parse_expr()?);
                    }
                    self.expect(TokenKind::RBracket, "Expected ']'")?;
                    Ok(Expr::Array(items))
                }
            }
            TokenKind::Fn => {
                self.advance();
                self.expect(TokenKind::LParen, "Expected '(' after 'fn'")?;
                let params = self.parse_params()?;
                self.expect(TokenKind::RParen, "Expected ')' after closure params")?;

                let return_type = if self.peek() == TokenKind::Colon {
                    self.advance();
                    Some(self.parse_type()?)
                } else {
                    None
                };

                // Arrow closure: fn($x) => $x * 2
                if self.peek() == TokenKind::FatArrow {
                    self.advance();
                    let body = self.parse_expr()?;
                    // Wrap expression body in a block with implicit return
                    Ok(Expr::Closure {
                        params,
                        return_type,
                        body: Box::new(Stmt::Block(vec![Stmt::Return(Some(body))])),
                    })
                } else {
                    // Block closure: fn($x) { return $x * 2; }
                    let body = self.parse_block()?;
                    Ok(Expr::Closure { params, return_type, body: Box::new(body) })
                }
            }
            TokenKind::Match => {
                self.advance(); // match
                self.expect(TokenKind::LParen, "Expected '(' after 'match'")?;
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen, "Expected ')' after match expr")?;
                self.expect(TokenKind::LBrace, "Expected '{' in match")?;
                let mut arms = Vec::new();
                while self.peek() != TokenKind::RBrace && self.peek() != TokenKind::Eof {
                    let pattern = self.parse_match_pattern()?;
                    self.expect(TokenKind::FatArrow, "Expected '=>' in match arm")?;
                    let body = self.parse_expr()?;
                    arms.push(MatchArm { pattern, body });
                    if self.peek() == TokenKind::Comma {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RBrace, "Expected '}' after match arms")?;
                return Ok(Expr::MatchExpr { expr: Box::new(expr), arms });
            }
            TokenKind::Ident(n) => {
                let name = n.clone();
                self.advance();
                Ok(Expr::Variable(name))
            }
            _ => Err(format!("Unexpected token at line {}: {}", self.cur().line, self.cur().kind)),
        }
    }

    // type → "int" | "f64" | "float" | "string" | "bool" | "void"
    //       | "[" type "]" | "[" type ":" type "]"
    //       | IDENT ("<" type ("," type)* ">")?
    //       | "Result" "<" type "," type ">"
    //       | "Option" "<" type ">"
    //       | "&" type | "&mut" type
    fn parse_type(&mut self) -> Result<TypeAnnotation, String> {
        match self.peek() {
            TokenKind::Ident(n) => {
                let name = n.clone();
                self.advance();
                match name.as_str() {
                    "int" => Ok(TypeAnnotation::Int),
                    "f64" | "float" => Ok(TypeAnnotation::Float),
                    "string" => Ok(TypeAnnotation::String_),
                    "bool" => Ok(TypeAnnotation::Bool),
                    "void" => Ok(TypeAnnotation::Void),
                    "Result" => {
                        if self.peek() == TokenKind::Lt {
                            self.advance();
                            let ok = self.parse_type()?;
                            self.expect(TokenKind::Comma, "Expected ',' in Result<_, _>")?;
                            let err = self.parse_type()?;
                            self.expect(TokenKind::Gt, "Expected '>' after Result types")?;
                            Ok(TypeAnnotation::Result_(Box::new(ok), Box::new(err)))
                        } else {
                            Ok(TypeAnnotation::Named(name))
                        }
                    }
                    "Option" => {
                        if self.peek() == TokenKind::Lt {
                            self.advance();
                            let inner = self.parse_type()?;
                            self.expect(TokenKind::Gt, "Expected '>' after Option type")?;
                            Ok(TypeAnnotation::Option_(Box::new(inner)))
                        } else {
                            Ok(TypeAnnotation::Named(name))
                        }
                    }
                    _ => {
                        if self.peek() == TokenKind::Lt {
                            self.advance();
                            let mut args = Vec::new();
                            loop {
                                args.push(self.parse_type()?);
                                if self.peek() == TokenKind::Comma {
                                    self.advance();
                                } else {
                                    break;
                                }
                            }
                            self.expect(TokenKind::Gt, "Expected '>' after generic args")?;
                            Ok(TypeAnnotation::Generic(name, args))
                        } else {
                            Ok(TypeAnnotation::Named(name))
                        }
                    }
                }
            }
            TokenKind::LBracket => {
                self.advance();
                let first = self.parse_type()?;
                if self.peek() == TokenKind::Colon {
                    // Dict type
                    self.advance();
                    let second = self.parse_type()?;
                    self.expect(TokenKind::RBracket, "Expected ']'")?;
                    Ok(TypeAnnotation::Dict(Box::new(first), Box::new(second)))
                } else {
                    // Array type
                    self.expect(TokenKind::RBracket, "Expected ']'")?;
                    Ok(TypeAnnotation::Array(Box::new(first)))
                }
            }
            _ => Err(format!("Expected type at line {}", self.cur().line)),
        }
    }

    #[allow(dead_code)]
    fn parse_match_or_stmt(&mut self) -> Result<Stmt, String> {
        self.parse_match_stmt()
    }
}

impl Parser {
    /// Parse match as an expression (for use in expression positions)
    #[allow(dead_code)]
    pub fn parse_match_expr_public(&mut self) -> Result<Expr, String> {
        // Not directly supported yet - match is a statement for MVP
        Err("match expression not yet supported in expression position".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    fn parse(src: &str) -> Result<Program, String> {
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize()?;
        let mut parser = Parser::new(tokens);
        parser.parse_program()
    }

    #[test]
    fn test_echo_string() {
        let prog = parse("<?phprs echo \"Hello\"; ?>").unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::Echo(expr) => match expr {
                Expr::Literal(Literal::String_(s)) => assert_eq!(s, "Hello"),
                _ => panic!("Expected string literal"),
            },
            _ => panic!("Expected echo stmt"),
        }
    }

    #[test]
    fn test_let_declaration() {
        let prog = parse("<?phprs let $x = 42; ?>").unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::Let { name, value, .. } => {
                assert_eq!(name, "x");
                assert!(value.is_some());
            }
            _ => panic!("Expected let stmt"),
        }
    }

    #[test]
    fn test_function() {
        let src = r#"<?phprs
            function add(int $a, int $b): int {
                return $a + $b;
            }
        ?>"#;
        let prog = parse(src).unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::Function { name, params, .. } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
            }
            _ => panic!("Expected function"),
        }
    }

    #[test]
    fn test_if_else() {
        let src = r#"<?phprs
            if ($x > 0) {
                echo "positive";
            } else {
                echo "not positive";
            }
        ?>"#;
        let prog = parse(src).unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::If { else_branch, .. } => {
                assert!(else_branch.is_some());
            }
            _ => panic!("Expected if stmt"),
        }
    }

    #[test]
    fn test_foreach() {
        let src = r#"<?phprs
            foreach ($items as $item) {
                echo $item;
            }
        ?>"#;
        let prog = parse(src).unwrap();
        assert_eq!(prog.stmts.len(), 1);
        match &prog.stmts[0] {
            Stmt::Foreach { value_var, .. } => {
                assert_eq!(value_var, "item");
            }
            _ => panic!("Expected foreach"),
        }
    }
}
