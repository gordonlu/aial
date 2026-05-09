// lexer.rs - AAL 语言的词法分析器

use crate::token::{lookup_keyword, Span, Token, TokenKind};

/// 词法分析器结构体
pub struct Lexer<'a> {
    source: &'a str,               // 源代码
    chars: Vec<char>,              // 字符数组（便于索引）
    pos: usize,                    // 当前字符位置（字节偏移）
    start: usize,                  // 当前词法起始位置
    line: usize,                   // 当前行号
    col: usize,                    // 当前列号
    tokens: Vec<Token>,            // 生成的 Token 列表
    errors: Vec<String>,           // 错误信息
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source,
            chars: source.chars().collect(),
            pos: 0,
            start: 0,
            line: 1,
            col: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// 执行词法分析，返回 Token 列表和错误列表
    pub fn tokenize(mut self) -> (Vec<Token>, Vec<String>) {
        while !self.is_at_end() {
            self.start = self.pos;
            self.scan_token();
        }
        self.tokens.push(Token::new(TokenKind::Eof, self.current_span()));
        (self.tokens, self.errors)
    }

    // === 辅助方法 ===

    fn is_at_end(&self) -> bool {
        self.pos >= self.chars.len()
    }

    fn current_char(&self) -> char {
        if self.is_at_end() { '\0' } else { self.chars[self.pos] }
    }

    fn peek_next(&self) -> char {
        if self.pos + 1 >= self.chars.len() { '\0' } else { self.chars[self.pos + 1] }
    }

    fn advance(&mut self) -> char {
        let c = self.current_char();
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        c
    }

    fn current_span(&self) -> Span {
        Span {
            start: self.start,
            end: self.pos,
            line: self.line,
            col: self.col,
        }
    }

    fn add_token(&mut self, kind: TokenKind) {
        self.tokens.push(Token::new(kind, self.current_span()));
    }

    fn add_error(&mut self, msg: String) {
        self.errors.push(format!("[{}:{}] {}", self.line, self.col, msg));
    }

    // === 主扫描函数 ===
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // 空白
            ' ' | '\t' | '\r' => { /* 忽略空白 */ }
            '\n' => { /* 换行已在 advance 中处理 */ }

            // 注释
            '/' if self.current_char() == '/' => {
                // 行注释：跳过直到行尾
                while self.current_char() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            '/' if self.current_char() == '*' => {
                // 块注释：跳过直到 */
                self.advance(); // 吞 '*'
                let mut closed = false;
                while !self.is_at_end() {
                    if self.current_char() == '*' && self.peek_next() == '/' {
                        self.advance(); // 吞 '*'
                        self.advance(); // 吞 '/'
                        closed = true;
                        break;
                    }
                    self.advance();
                }
                if !closed {
                    self.add_error("unclosed block comment".to_string());
                }
            }

            // 字符串
            '"' => self.scan_string(),

            // 数字（整数或浮点）
            '0'..='9' => self.scan_number(),

            // 标识符或关键字
            'a'..='z' | 'A'..='Z' | '_' => self.scan_identifier_or_keyword(),

            // 操作符与符号
            '=' => {
                if self.current_char() == '=' {
                    self.advance();
                    self.add_token(TokenKind::EqEq);
                } else if self.current_char() == '>' {
                    self.advance();
                    self.add_token(TokenKind::FatArrow);
                } else {
                    self.add_token(TokenKind::Assign);
                }
            }
            '!' => {
                if self.current_char() == '=' {
                    self.advance();
                    self.add_token(TokenKind::NotEq);
                } else {
                    self.add_token(TokenKind::Not);
                }
            }
            '<' => {
                if self.current_char() == '=' {
                    self.advance();
                    self.add_token(TokenKind::LtEq);
                } else {
                    self.add_token(TokenKind::Lt);
                }
            }
            '>' => {
                if self.current_char() == '=' {
                    self.advance();
                    self.add_token(TokenKind::GtEq);
                } else {
                    self.add_token(TokenKind::Gt);
                }
            }
            '|' => {
                if self.current_char() == '>' {
                    self.advance();
                    self.add_token(TokenKind::PipeGt);
                } else if self.current_char() == '|' {
                    self.advance();
                    self.add_token(TokenKind::OrOr);
                } else {
                    self.add_token(TokenKind::Pipe);
                }
            }
            '&' => {
                if self.current_char() == '&' {
                    self.advance();
                    self.add_token(TokenKind::AndAnd);
                } else {
                    self.add_error("单独使用 '&' 非法".to_string());
                }
            }
            '.' => {
                if self.current_char() == '.' {
                    self.advance();
                    self.add_token(TokenKind::DotDot);
                } else {
                    self.add_token(TokenKind::Dot);
                }
            }
            ':' => {
                if self.current_char() == ':' {
                    self.advance();
                    self.add_token(TokenKind::ColonColon);
                } else {
                    self.add_token(TokenKind::Colon);
                }
            }
            '-' => {
                if self.current_char() == '>' {
                    self.advance();
                    self.add_token(TokenKind::Arrow);
                } else {
                    self.add_token(TokenKind::Minus);
                }
            }
            '+' => self.add_token(TokenKind::Plus),
            '*' => self.add_token(TokenKind::Star),
            '%' => self.add_token(TokenKind::Percent),
            ';' => self.add_token(TokenKind::Semicolon),
            ',' => self.add_token(TokenKind::Comma),
            '(' => self.add_token(TokenKind::Lparen),
            ')' => self.add_token(TokenKind::Rparen),
            '{' => self.add_token(TokenKind::Lbrace),
            '}' => self.add_token(TokenKind::Rbrace),
            '[' => {
                // 检查 #[ 注解开始
                if self.current_char() == '#' {
                    // 已经读入了 '['，现在检查下一个 '#'
                    // 实际上，调用栈是：先遇到 '#' 还是 '['?
                    // 我们应处理 '#' 触发注解。在 match c 中，c 是当前字符。
                    // 如果 c 是 '['，我们无法知道前面是否有 '#'，所以需要将注解开始的检测放在 '#' 分支里。
                    // 调整：在 '#' 匹配中，如果下一个字符是 '['，则产生 AttrStart，并吞掉 '['
                    // 因此在 '[' 分支中无需特殊处理。
                    self.add_token(TokenKind::Lbracket);
                } else {
                    self.add_token(TokenKind::Lbracket);
                }
            }
            ']' => self.add_token(TokenKind::Rbracket),
            '#' => {
                if self.current_char() == '[' {
                    self.advance(); // 吞 '['
                    self.add_token(TokenKind::AttrStart);
                } else {
                    self.add_error("单独使用 '#' 非法".to_string());
                }
            }
            '@' => self.add_token(TokenKind::AtSign),
            '/' => {
                // 已在注释处理中消耗，不会到这里，但保留
                self.add_token(TokenKind::Slash);
            }
            _ => {
                let msg = format!("illegal character: `{}`", c);
                self.add_error(msg);
                // 不停止，继续
            }
        }
    }

    // === 子扫描函数 ===

    fn scan_string(&mut self) {
        let _start_span = self.current_span();
        let mut result = String::new();
        while self.current_char() != '"' && !self.is_at_end() {
            if self.current_char() == '\\' {
                self.advance(); // 吞反斜杠
                match self.current_char() {
                    'n' => { result.push('\n'); self.advance(); }
                    't' => { result.push('\t'); self.advance(); }
                    'r' => { result.push('\r'); self.advance(); }
                    '0' => { result.push('\0'); self.advance(); }
                    '"' => { result.push('"'); self.advance(); }
                    '\\' => { result.push('\\'); self.advance(); }
                    'x' => {
                        // \xNN
                        self.advance();
                        let hex = self.read_hex_digits(2);
                        if let Some(hex_str) = hex {
                            let code = u8::from_str_radix(&hex_str, 16).unwrap_or(0);
                            result.push(code as char);
                        } else {
                            self.add_error("incomplete hex escape sequence".to_string());
                        }
                    }
                    'u' => {
                        // \u{NNNN}
                        self.advance(); // 吞 'u'
                        if self.current_char() == '{' {
                            self.advance(); // 吞 '{'
                            let mut hex_str = String::new();
                            while self.current_char().is_ascii_hexdigit() {
                                hex_str.push(self.advance());
                            }
                            if self.current_char() == '}' {
                                self.advance(); // 吞 '}'
                                if hex_str.is_empty() {
                                    self.add_error("empty unicode escape sequence".to_string());
                                } else {
                                    let code = u32::from_str_radix(&hex_str, 16).unwrap_or(0);
                                    if let Some(ch) = char::from_u32(code) {
                                        result.push(ch);
                                    } else {
                                        self.add_error(format!("invalid Unicode codepoint: {}", code));
                                    }
                                }
                            } else {
                                self.add_error("unicode escape sequence missing `}`".to_string());
                            }
                        } else {
                            self.add_error("unicode escape must have \\u{...} format".to_string());
                        }
                    }
                    '\n' => {
                        // 跨行字符串不允许
                        self.add_error("unescaped newline not allowed in string literal".to_string());
                    }
                    _ => {
                        result.push(self.current_char());
                    }
                }
            } else {
                result.push(self.advance());
            }
        }
        if self.is_at_end() {
            self.add_error("unclosed string literal".to_string());
            // 补偿一个结束位置
        } else {
            self.advance(); // 吞闭合引号
        }
        self.add_token(TokenKind::String(result));
    }

    fn scan_number(&mut self) {
        let mut has_dot = false;
        let mut has_exp = false;
        let mut num_str = String::new();
        // 我们已经消耗了第一个数字字符
        num_str.push(self.chars[self.pos - 1]);
        while self.current_char().is_ascii_digit() || self.current_char() == '_' {
            if self.current_char() != '_' {
                num_str.push(self.current_char());
            }
            self.advance();
        }
        // 检查浮点
        if self.current_char() == '.' && self.peek_next().is_ascii_digit() {
            has_dot = true;
            num_str.push(self.advance()); // 小数点
            while self.current_char().is_ascii_digit() || self.current_char() == '_' {
                if self.current_char() != '_' {
                    num_str.push(self.current_char());
                }
                self.advance();
            }
        }
        // 指数部分
        if self.current_char() == 'e' || self.current_char() == 'E' {
            let next = self.peek_next();
            if next.is_ascii_digit() || next == '+' || next == '-' {
                has_exp = true;
                num_str.push(self.advance()); // 'e' 或 'E'
                if self.current_char() == '+' || self.current_char() == '-' {
                    num_str.push(self.advance());
                }
                while self.current_char().is_ascii_digit() || self.current_char() == '_' {
                    if self.current_char() != '_' {
                        num_str.push(self.current_char());
                    }
                    self.advance();
                }
            }
        }
        // 解析数值并生成 Token
        if has_dot || has_exp {
            match num_str.parse::<f64>() {
                Ok(val) => self.add_token(TokenKind::Float(val)),
                Err(_) => self.add_error(format!("invalid float literal: {}", num_str)),
            }
        } else {
            match num_str.parse::<u64>() {
                Ok(val) => self.add_token(TokenKind::Int(val)),
                Err(_) => self.add_error(format!("invalid integer literal: {}", num_str)),
            }
        }
    }

    fn scan_identifier_or_keyword(&mut self) {
        while self.current_char().is_alphanumeric() || self.current_char() == '_' {
            self.advance();
        }
        let word: String = self.chars[self.start..self.pos].iter().collect();
        if let Some(keyword) = lookup_keyword(&word) {
            self.add_token(keyword);
        } else {
            self.add_token(TokenKind::Ident(word));
        }
    }

    /// 读取指定数量的十六进制数字，返回字符串
    fn read_hex_digits(&mut self, count: usize) -> Option<String> {
        let mut hex = String::new();
        for _ in 0..count {
            if self.current_char().is_ascii_hexdigit() {
                hex.push(self.advance());
            } else {
                return None;
            }
        }
        Some(hex)
    }
}

// ========== 测试 ==========

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize(source: &str) -> Vec<TokenKind> {
        let lexer = Lexer::new(source);
        let (tokens, _errors) = lexer.tokenize();
        tokens.into_iter().map(|t| t.kind).collect()
    }

    #[test]
    fn test_empty() {
        let tokens = tokenize("");
        assert_eq!(tokens, vec![TokenKind::Eof]);
    }

    #[test]
    fn test_keywords() {
        let source = "fn let mut if else match as";
        let tokens = tokenize(source);
        assert_eq!(
            tokens,
            vec![
                TokenKind::Fn,
                TokenKind::Let,
                TokenKind::Mut,
                TokenKind::If,
                TokenKind::Else,
                TokenKind::Match,
                TokenKind::As,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let source = "42 3.14 2.7e-5 100_000";
        let tokens = tokenize(source);
        match &tokens[..] {
            [TokenKind::Int(42), TokenKind::Float(3.14), TokenKind::Float(2.7e-5), TokenKind::Int(100_000), TokenKind::Eof] => (),
            _ => panic!("Unexpected tokens: {:?}", tokens),
        }
    }

    #[test]
    fn test_string() {
        let source = r#""hello\nworld" "#;
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 2); // string + eof
        if let TokenKind::String(s) = &tokens[0] {
            assert_eq!(s, "hello\nworld");
        } else {
            panic!("Not a string token");
        }
    }

    #[test]
    fn test_operators() {
        let source = "+ - * / % == != < > <= >= && || ! |> . :: -> => #[ ] ;";
        let tokens = tokenize(source);
        let expected = vec![
            TokenKind::Plus,
            TokenKind::Minus,
            TokenKind::Star,
            TokenKind::Slash,
            TokenKind::Percent,
            TokenKind::EqEq,
            TokenKind::NotEq,
            TokenKind::Lt,
            TokenKind::Gt,
            TokenKind::LtEq,
            TokenKind::GtEq,
            TokenKind::AndAnd,
            TokenKind::OrOr,
            TokenKind::Not,
            TokenKind::PipeGt,
            TokenKind::Dot,
            TokenKind::ColonColon,
            TokenKind::Arrow,
            TokenKind::FatArrow,
            TokenKind::AttrStart,
            TokenKind::Rbracket,
            TokenKind::Semicolon,
            TokenKind::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_attribute() {
        let source = r#"#[tool(name = "x")]"#;
        let tokens = tokenize(source);
        let expected = vec![
            TokenKind::AttrStart,
            TokenKind::Ident("tool".into()),
            TokenKind::Lparen,
            TokenKind::Ident("name".into()),
            TokenKind::Assign,
            TokenKind::String("x".into()),
            TokenKind::Rparen,
            TokenKind::Rbracket,
            TokenKind::Eof,
        ];
        assert_eq!(tokens, expected);
    }

    #[test]
    fn test_comments() {
        let source = "// This is a comment\nlet x = 1; /* block */";
        let tokens = tokenize(source);
        assert_eq!(
            tokens,
            vec![
                TokenKind::Let,
                TokenKind::Ident("x".into()),
                TokenKind::Assign,
                TokenKind::Int(1),
                TokenKind::Semicolon,
                TokenKind::Eof,
            ]
        );
    }

    #[test]
    fn test_error_recovery() {
        let source = "let @ x";
        let tokens = tokenize(source);
        assert_eq!(tokens.len(), 4);
        assert!(matches!(tokens[0], TokenKind::Let));
        assert!(matches!(tokens[1], TokenKind::AtSign));
        assert!(matches!(tokens[2], TokenKind::Ident(_)));
        assert!(matches!(tokens[3], TokenKind::Eof));
    }
}
