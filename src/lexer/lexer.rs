use crate::lexer::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
    in_phprs: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
            in_phprs: false,
        }
    }

    fn cur(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek(&self, offset: usize) -> Option<char> {
        self.source.get(self.pos + offset).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.get(self.pos).copied();
        if let Some(ch) = c {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 1;
            } else {
                self.col += 1;
            }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.cur() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(c) = self.cur() {
            if c == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        while let Some(c) = self.cur() {
            self.advance();
            if c == '*' && self.cur() == Some('/') {
                self.advance();
                return;
            }
        }
    }

    fn read_number(&mut self, first: char) -> TokenKind {
        let mut num = String::from(first);
        let mut is_float = false;

        // Hex prefix
        if first == '0' && self.cur() == Some('x') {
            num.push(self.advance().unwrap());
            while let Some(c) = self.cur() {
                if c.is_ascii_hexdigit() || c == '_' {
                    num.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            let s: String = num.chars().filter(|c| *c != '_').collect();
            return TokenKind::Int(i64::from_str_radix(&s[2..], 16).unwrap_or(0));
        }

        while let Some(c) = self.cur() {
            if c.is_ascii_digit() || c == '_' {
                num.push(self.advance().unwrap());
            } else if c == '.' && self.peek(1).map_or(false, |c2| c2.is_ascii_digit()) {
                is_float = true;
                num.push(self.advance().unwrap()); // .
            } else {
                break;
            }
        }

        let s: String = num.chars().filter(|c| *c != '_').collect();
        if is_float {
            TokenKind::Float(s.parse().unwrap_or(0.0))
        } else {
            TokenKind::Int(s.parse().unwrap_or(0))
        }
    }

    fn read_string(&mut self) -> Result<TokenKind, String> {
        let mut s = String::new();
        self.advance(); // skip opening "
        while let Some(c) = self.cur() {
            self.advance();
            if c == '"' {
                return Ok(TokenKind::String_(s));
            }
            if c == '\\' {
                match self.cur() {
                    Some('n') => {
                        s.push('\n');
                        self.advance();
                    }
                    Some('t') => {
                        s.push('\t');
                        self.advance();
                    }
                    Some('r') => {
                        s.push('\r');
                        self.advance();
                    }
                    Some('\\') => {
                        s.push('\\');
                        self.advance();
                    }
                    Some('"') => {
                        s.push('"');
                        self.advance();
                    }
                    Some('$') => {
                        s.push('$');
                        self.advance();
                    }
                    Some(c) => {
                        s.push('\\');
                        s.push(c);
                        self.advance();
                    }
                    None => return Err("Unterminated string".to_string()),
                }
            } else {
                s.push(c);
            }
        }
        Err("Unterminated string".to_string())
    }

    fn read_ident(&mut self, first: char) -> TokenKind {
        let mut ident = String::from(first);
        while let Some(c) = self.cur() {
            if c.is_alphanumeric() || c == '_' {
                ident.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        match ident.as_str() {
            "let" => TokenKind::Let,
            "mut" => TokenKind::Mut,
            "fn" => TokenKind::Fn,
            "function" => TokenKind::Function,
            "extern" => TokenKind::Extern,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "foreach" => TokenKind::Foreach,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "echo" => TokenKind::Echo,
            "struct" => TokenKind::Struct,
            "enum" => TokenKind::Enum,
            "match" => TokenKind::Match,
            "use" => TokenKind::Use,
            "mod" => TokenKind::Mod,
            "pub" => TokenKind::Pub,
            "const" => TokenKind::Const,
            "async" => TokenKind::Async,
            "await" => TokenKind::Await,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "throw" => TokenKind::Throw,
            "elseif" => TokenKind::Elseif,
            "as" => TokenKind::As,
            "in" => TokenKind::In,
            "move" => TokenKind::Move,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            _ => TokenKind::Ident(ident),
        }
    }

    fn try_scan_tag(&mut self) -> Option<TokenKind> {
        if self.pos + 8 <= self.source.len() {
            let slice: String = self.source[self.pos..self.pos + 8].iter().collect();
            if slice == "<?phprs " || slice == "<?phprs\n" || slice == "<?phprs\t" || slice == "<?phprs=" {
                for _ in 0..7 {
                    self.advance();
                }
                self.in_phprs = true;
                return Some(TokenKind::OpenTag);
            }
            if slice.starts_with("<?phprs") && self.pos + 7 <= self.source.len() {
                let s7: String = self.source[self.pos..self.pos + 7].iter().collect();
                if s7 == "<?phprs" {
                    for _ in 0..7 {
                        self.advance();
                    }
                    self.in_phprs = true;
                    return Some(TokenKind::OpenTag);
                }
            }
        }
        None
    }

    fn try_scan_close_tag(&mut self) -> Option<TokenKind> {
        if self.cur() == Some('?') && self.peek(1) == Some('>') {
            self.advance();
            self.advance();
            self.in_phprs = false;
            return Some(TokenKind::CloseTag);
        }
        None
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        // If not in PHPRS mode, scan for opening tag
        if !self.in_phprs {
            while let Some(c) = self.cur() {
                if c == '<' {
                    if let Some(tag) = self.try_scan_tag() {
                        return Ok(Token::new(tag, self.line, self.col));
                    }
                }
                self.advance();
            }
            return Ok(Token::new(TokenKind::Eof, self.line, self.col));
        }

        self.skip_whitespace();

        // Check for close tag
        if let Some(tag) = self.try_scan_close_tag() {
            return Ok(Token::new(tag, self.line, self.col));
        }

        let c = match self.cur() {
            Some(c) => c,
            None => return Ok(Token::new(TokenKind::Eof, self.line, self.col)),
        };

        let line = self.line;
        let col = self.col;

        // String literals
        if c == '"' {
            let kind = self.read_string()?;
            return Ok(Token::new(kind, line, col));
        }

        // Variable identifier
        if c == '$' {
            self.advance();
            let first = match self.cur() {
                Some(ch) if ch.is_alphabetic() || ch == '_' => ch,
                Some(ch) => {
                    return Err(format!("Unexpected character after $: '{}' at line {}", ch, line));
                }
                None => return Err(format!("Unexpected end of input after $ at line {}", line)),
            };
            self.advance();
            let mut name = String::from(first);
            while let Some(ch) = self.cur() {
                if ch.is_alphanumeric() || ch == '_' {
                    name.push(self.advance().unwrap());
                } else {
                    break;
                }
            }
            return Ok(Token::new(TokenKind::Var(name), line, col));
        }

        // Numbers
        if c.is_ascii_digit() {
            self.advance(); // consume first char
            let kind = self.read_number(c);
            return Ok(Token::new(kind, line, col));
        }

        // Identifiers and keywords
        if c.is_alphabetic() || c == '_' {
            self.advance(); // consume first char
            let kind = self.read_ident(c);
            return Ok(Token::new(kind, line, col));
        }

        // Operators and delimiters
        self.advance();
        let kind = match c {
            '+' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::PlusEq }
                Some('+') => { self.advance(); TokenKind::Inc }
                _ => TokenKind::Plus,
            },
            '-' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::MinusEq }
                Some('>') => { self.advance(); TokenKind::ThinArrow }
                Some('-') => { self.advance(); TokenKind::Dec }
                _ => TokenKind::Minus,
            },
            '*' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::StarEq }
                _ => TokenKind::Star,
            },
            '/' => match self.cur() {
                Some('/') => {
                    self.skip_line_comment();
                    return self.next_token();
                }
                Some('*') => {
                    self.skip_block_comment();
                    return self.next_token();
                }
                Some('=') => { self.advance(); TokenKind::SlashEq }
                _ => TokenKind::Slash,
            },
            '%' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::PercentEq }
                _ => TokenKind::Percent,
            },
            '.' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::DotEq }
                Some('.') => {
                    self.advance();
                    match self.cur() {
                        Some('=') => { self.advance(); TokenKind::DotDotEq }
                        _ => TokenKind::DotDot,
                    }
                }
                _ => TokenKind::Dot,
            },
            '=' => match self.cur() {
                Some('=') => {
                    self.advance();
                    match self.cur() {
                        Some('=') => { self.advance(); TokenKind::EqEqEq }
                        _ => TokenKind::EqEq,
                    }
                }
                Some('>') => { self.advance(); TokenKind::FatArrow }
                _ => TokenKind::Eq,
            },
            '!' => match self.cur() {
                Some('=') => {
                    self.advance();
                    match self.cur() {
                        Some('=') => { self.advance(); TokenKind::NeqEq }
                        _ => TokenKind::Neq,
                    }
                }
                _ => TokenKind::Not,
            },
            '<' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::Le }
                _ => TokenKind::Lt,
            },
            '>' => match self.cur() {
                Some('=') => { self.advance(); TokenKind::Ge }
                _ => TokenKind::Gt,
            },
            '&' => match self.cur() {
                Some('&') => { self.advance(); TokenKind::AndAnd }
                _ => return Err(format!("Unexpected '&' at line {}", line)),
            },
            '|' => match self.cur() {
                Some('|') => { self.advance(); TokenKind::OrOr }
                _ => return Err(format!("Unexpected '|' at line {}", line)),
            },
            ':' => match self.cur() {
                Some(':') => { self.advance(); TokenKind::ColonColon }
                _ => TokenKind::Colon,
            },
            ';' => TokenKind::Semi,
            ',' => TokenKind::Comma,
            '?' => TokenKind::Question,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            _ => return Err(format!("Unexpected character: '{}' at line {}", c, line)),
        };

        Ok(Token::new(kind, line, col))
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            if token.kind == TokenKind::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token_kinds(tokens: &[Token]) -> Vec<TokenKind> {
        tokens.iter().map(|t| t.kind.clone()).collect()
    }

    #[test]
    fn test_hello_world() {
        let src = "<?phprs echo \"Hello World\"; ?>";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                TokenKind::OpenTag,
                TokenKind::Echo,
                TokenKind::String_("Hello World".into()),
                TokenKind::Semi,
                TokenKind::CloseTag,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_let_statement() {
        let src = "<?phprs let $x: int = 42; ?>";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                TokenKind::OpenTag,
                TokenKind::Let,
                TokenKind::Var("x".into()),
                TokenKind::Colon,
                TokenKind::Ident("int".into()),
                TokenKind::Eq,
                TokenKind::Int(42),
                TokenKind::Semi,
                TokenKind::CloseTag,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_operators() {
        let src = "<?phprs $a += 1; $b .= \"x\"; $c == $d; $e != $f; ?>";
        let mut lexer = Lexer::new(src);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            token_kinds(&tokens),
            vec![
                TokenKind::OpenTag,
                TokenKind::Var("a".into()), TokenKind::PlusEq, TokenKind::Int(1), TokenKind::Semi,
                TokenKind::Var("b".into()), TokenKind::DotEq, TokenKind::String_("x".into()), TokenKind::Semi,
                TokenKind::Var("c".into()), TokenKind::EqEq, TokenKind::Var("d".into()), TokenKind::Semi,
                TokenKind::Var("e".into()), TokenKind::Neq, TokenKind::Var("f".into()), TokenKind::Semi,
                TokenKind::CloseTag,
                TokenKind::Eof,
            ]
        );
    }
}
