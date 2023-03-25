use std::collections::HashMap;
use std::fs;
#[allow(unused_imports)]
use std::time::{Instant, Duration};

#[derive(PartialEq, PartialOrd)]
#[allow(dead_code, non_camel_case_types)]
enum TokenType {
    BLANK        = -2,
    EOF          = -1,
    NEWLINE      = 0,
    NUMBER       = 1,
    IDENT        = 2,
    STRING       = 3,
    PLUS         = 101,
    MINUS        = 102,
    STAR         = 103,
    SLASH        = 104,
    PERCENT      = 105,
    CARET        = 106,
    AMPERSAND    = 107,
    PIPE         = 108,
    EQUAL        = 201,
    EQUAL_EQUAL  = 202,
    NOT_EQUAL    = 203,
    LESS         = 204,
    LESS_EQUAL   = 205,
    GREATER      = 206,
    GREATER_EQUAL= 207,
    NOT          = 208,
    LEFT_PAREN   = 301,
    RIGHT_PAREN  = 302,
    LET          = 401,
    IF           = 402,
    ELSE         = 403,
    WHILE        = 404,
    FOR          = 405,
    END          = 406,
    THEN         = 407,
    DO           = 408,
    LABEL        = 409,
    GOTO         = 410,
}

impl TokenType {
    fn display(&self) -> String {
        match self {
            TokenType::BLANK         => String::from("BLANK"),
            TokenType::EOF           => String::from("EOF"),
            TokenType::NEWLINE       => String::from("NEWLINE"),
            TokenType::NUMBER        => String::from("NUMBER"),
            TokenType::IDENT         => String::from("IDENT"),
            TokenType::STRING        => String::from("STRING"),
            TokenType::PLUS          => String::from("PLUS"),
            TokenType::MINUS         => String::from("MINUS"),
            TokenType::STAR          => String::from("STAR"),
            TokenType::SLASH         => String::from("SLASH"),
            TokenType::PERCENT       => String::from("PERCENT"),
            TokenType::CARET         => String::from("CARET"),
            TokenType::AMPERSAND     => String::from("AMPERSAND"),
            TokenType::PIPE          => String::from("PIPE"),
            TokenType::EQUAL         => String::from("EQUAL"),
            TokenType::EQUAL_EQUAL   => String::from("EQUAL_EQUAL"),
            TokenType::NOT_EQUAL     => String::from("NOT_EQUAL"),
            TokenType::LESS          => String::from("LESS"),
            TokenType::LESS_EQUAL    => String::from("LESS_EQUAL"),
            TokenType::GREATER       => String::from("GREATER"),
            TokenType::GREATER_EQUAL => String::from("GREATER_EQUAL"),
            TokenType::NOT           => String::from("NOT"),
            TokenType::LEFT_PAREN    => String::from("LEFT_PAREN"),
            TokenType::RIGHT_PAREN   => String::from("RIGHT_PAREN"),
            TokenType::LET           => String::from("LET"),
            TokenType::IF            => String::from("IF"),
            TokenType::ELSE          => String::from("ELSE"),
            TokenType::WHILE         => String::from("WHILE"),
            TokenType::FOR           => String::from("FOR"),
            TokenType::END           => String::from("END"),
            TokenType::THEN          => String::from("THEN"),
            TokenType::DO            => String::from("DO"),
            TokenType::LABEL         => String::from("LABEL"),
            TokenType::GOTO          => String::from("GOTO"),
        }
    }

    fn copy(&self) -> TokenType {
        match self {
            TokenType::BLANK         => TokenType::BLANK,
            TokenType::EOF           => TokenType::EOF,
            TokenType::NEWLINE       => TokenType::NEWLINE,
            TokenType::NUMBER        => TokenType::NUMBER,
            TokenType::IDENT         => TokenType::IDENT,
            TokenType::STRING        => TokenType::STRING,
            TokenType::PLUS          => TokenType::PLUS,
            TokenType::MINUS         => TokenType::MINUS,
            TokenType::STAR          => TokenType::STAR,
            TokenType::SLASH         => TokenType::SLASH,
            TokenType::PERCENT       => TokenType::PERCENT,
            TokenType::CARET         => TokenType::CARET,
            TokenType::AMPERSAND     => TokenType::AMPERSAND,
            TokenType::PIPE          => TokenType::PIPE,
            TokenType::EQUAL         => TokenType::EQUAL,
            TokenType::EQUAL_EQUAL   => TokenType::EQUAL_EQUAL,
            TokenType::NOT_EQUAL     => TokenType::NOT_EQUAL,
            TokenType::LESS          => TokenType::LESS,
            TokenType::LESS_EQUAL    => TokenType::LESS_EQUAL,
            TokenType::GREATER       => TokenType::GREATER,
            TokenType::GREATER_EQUAL => TokenType::GREATER_EQUAL,
            TokenType::NOT           => TokenType::NOT,
            TokenType::LEFT_PAREN    => TokenType::LEFT_PAREN,
            TokenType::RIGHT_PAREN   => TokenType::RIGHT_PAREN, 
            TokenType::LET           => TokenType::LET,
            TokenType::IF            => TokenType::IF,
            TokenType::ELSE          => TokenType::ELSE,
            TokenType::WHILE         => TokenType::WHILE,
            TokenType::FOR           => TokenType::FOR,
            TokenType::END           => TokenType::END,
            TokenType::THEN          => TokenType::THEN,
            TokenType::DO            => TokenType::DO,
            TokenType::LABEL         => TokenType::LABEL,
            TokenType::GOTO          => TokenType::GOTO,
        }
    }
}

struct Token {
    text: String,
    kind: TokenType,
}

impl Token {
    fn new(text: String, kind: TokenType) -> Token {
        Token { text, kind }
    }

    fn copy(&self) -> Token {
        Token::new(self.text.clone(), self.kind.copy())
    }
}

struct Lexer {
    source: String,
    pos: usize,
    char: char,
}

impl Lexer {
    fn new(source: String) -> Lexer {
        let mut lexer = Lexer {
            source, 
            pos: 0, 
            char: '\0',
        };
        lexer.init();
        lexer
    }

    fn init(&mut self) {
        self.pos = 0;
        self.char = self.source.chars().nth(self.pos).unwrap();
    }

    fn next(&mut self) {
        self.pos += 1;
        if self.pos >= self.source.len() {
            self.char = '\0';
        } else {
            self.char = self.source.chars().nth(self.pos).unwrap();
        }
    }

    fn peek(&self) -> char {
        if self.pos + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.pos + 1).unwrap()
        }
    }

    fn skip_blank(&mut self) {
        while self.char.is_whitespace() {
            self.next();
        }
    }

    fn get_token(&mut self) -> Token {
        let mut text = String::new();
        #[allow(unused_assignments)]
        let mut kind = TokenType::BLANK;
        self.skip_blank();
        match self.char {
            '\0' => { text = String::from("EOF"); kind = TokenType::EOF; }
            '\n' => { text = String::from("newline"); kind = TokenType::NEWLINE; }
            '0'..='9' => {
                while self.char.is_digit(10) {
                    text.push(self.char);
                    self.next();
                }
                kind = TokenType::NUMBER;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                while self.char.is_alphanumeric() || self.char == '_' {
                    text.push(self.char);
                    self.next();
                }
                match text.as_str() {
                    "let"   => { kind = TokenType::LET; }
                    "if"    => { kind = TokenType::IF; }
                    "else"  => { kind = TokenType::ELSE; }
                    "while" => { kind = TokenType::WHILE; }
                    "for"   => { kind = TokenType::FOR; }
                    "end"   => { kind = TokenType::END; }
                    "then"  => { kind = TokenType::THEN; }
                    "do"    => { kind = TokenType::DO; }
                    "label" => { kind = TokenType::LABEL; }
                    "goto"  => { kind = TokenType::GOTO; }
                    _       => { kind = TokenType::IDENT; }
                }
            }
            '"' => {
                self.next();
                while self.char != '"' {
                    text.push(self.char);
                    self.next();
                }
                self.next();
                kind = TokenType::STRING;
            }
            '+' => { text = String::from("+"); kind = TokenType::PLUS; }
            '-' => { text = String::from("-"); kind = TokenType::MINUS; }
            '*' => { text = String::from("*"); kind = TokenType::STAR; }
            '/' => { text = String::from("/"); kind = TokenType::SLASH; }
            '%' => { text = String::from("%"); kind = TokenType::PERCENT; }
            '^' => { text = String::from("^"); kind = TokenType::CARET; }
            '&' => { text = String::from("&"); kind = TokenType::AMPERSAND; }
            '|' => { text = String::from("|"); kind = TokenType::PIPE; }
            '=' => {
                if self.peek() == '=' {
                    self.next();
                    text = String::from("==");
                    kind = TokenType::EQUAL_EQUAL;
                } else {
                    text = String::from("=");
                    kind = TokenType::EQUAL;
                }
            }
            '!' => {
                if self.peek() == '=' {
                    self.next();
                    text = String::from("!=");
                    kind = TokenType::NOT_EQUAL;
                } else {
                    text = String::from("!");
                    kind = TokenType::NOT;
                }
            }
            '<' => {
                if self.peek() == '=' {
                    self.next();
                    text = String::from("<=");
                    kind = TokenType::LESS_EQUAL;
                } else {
                    text = String::from("<");
                    kind = TokenType::LESS;
                }
            }
            '>' => {
                if self.peek() == '=' {
                    self.next();
                    text = String::from(">=");
                    kind = TokenType::GREATER_EQUAL;
                } else {
                    text = String::from(">");
                    kind = TokenType::GREATER;
                }
            }
            '(' => { text = String::from("("); kind = TokenType::LEFT_PAREN; }
            ')' => { text = String::from(")"); kind = TokenType::RIGHT_PAREN; }
            _ => { panic!("Unknown character")}
        }
        self.next();
        Token::new(text, kind)
    }
}

struct Parser {
    lexer: Lexer,
    current: Token,
    peek: Token,
    symbols: HashMap<String, i32>,
    sym_addr: i32,
    labels: HashMap<String, i32>,
    main_buffer: String,
    line_number: i32,
    condition_checking: bool,
    in_func: bool,
    func_buffer: String,
    condition_buffer: String,
}

impl Parser {
    fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer, 
            current: Token::new(String::from(""), TokenType::BLANK), 
            peek: Token::new(String::from(""), TokenType::BLANK), 
            symbols: HashMap::new(), 
            sym_addr: 0, 
            labels: HashMap::new(),
            main_buffer: String::new(),
            line_number: 0,
            condition_checking: false,
            in_func: false,
            func_buffer: String::new(),
            condition_buffer: String::new(),
        };
        parser.next();
        parser.next();
        parser
    }

    fn next(&mut self) {
        self.current = self.peek.copy();
        self.peek = self.lexer.get_token();
    }

    fn check_token(&mut self, kind: TokenType) -> bool {
        if self.current.kind == kind {
            true 
        } else {
            false
        }
    }

    fn _match(&mut self, kind: TokenType) {
        if self.current.kind == kind {
            self.next();
        } else {
            panic!("match error: {} != {}", self.current.text, kind.display())
        }
    }

    fn code_gen(&mut self, code: String) {
        if self.condition_checking {
            self.condition_buffer.push_str(&format!("{}\n", code));
        } 
        else if self.in_func {
            self.func_buffer.push_str(&format!("{}\n", code));
        }
        else {
            self.main_buffer.push_str(&format!("{}\n", code));
        }
        self.line_number += 5;
    }

    fn operator_check(&mut self) -> bool {
        match self.current.kind {
            TokenType::PLUS | TokenType::MINUS | TokenType::STAR | TokenType::SLASH | TokenType::PERCENT | TokenType::CARET | TokenType::AMPERSAND | TokenType::PIPE => true,
            _ => false,
        }
    }

    fn expression(&mut self) {
        if self.check_token(TokenType::NUMBER) {
            self.code_gen(format!("set r0 {} r0", self.current.text));
            self._match(TokenType::NUMBER);
        } else if self.check_token(TokenType::IDENT) {
            if self.symbols.contains_key(&self.current.text) {
                self.code_gen(format!("set bp {} bp", self.symbols.get(&self.current.text).unwrap()));
                self.code_gen(format!("addi ram 0 r0"));
                self._match(TokenType::IDENT);
            } else {
                panic!("Undefined variable {}", self.current.text);
            }
        } else {
            panic!("Expected number or identifier for expression, got '{}'", self.current.text);
        }
        while self.operator_check() {
            let op = self.current.text.clone();
            self.next();
            if self.check_token(TokenType::NUMBER) {
                self.code_gen(format!("set r1 {} r1", self.current.text));
                self._match(TokenType::NUMBER);
            } else if self.check_token(TokenType::IDENT) {
                if self.symbols.contains_key(&self.current.text) {
                    self.code_gen(format!("set bp {} bp", self.symbols.get(&self.current.text).unwrap()));
                    self.code_gen(format!("addi ram 0 r1"));
                    self._match(TokenType::IDENT);
                } else {
                    panic!("Undefined variable {}", self.current.text);
                }
            } else {
                panic!("Expected number or identifier for expression, got {}", self.current.text);
            }
            match op.as_str() {
                "+" => self.code_gen(format!("add r0 r1 r0")),
                "-" => self.code_gen(format!("sub r0 r1 r0")),
                "*" => self.code_gen(format!("mul r0 r1 r0")),
                "/" => self.code_gen(format!("div r0 r1 r0")),
                "%" => self.code_gen(format!("mod r0 r1 r0")),
                "^" => self.code_gen(format!("xor r0 r1 r0")),
                "&" => self.code_gen(format!("and r0 r1 r0")),
                "|" => self.code_gen(format!("or r0 r1 r0")),
                _ => panic!("Unknown operator {}", op),
            }
        }
    }

    fn condition(&mut self) {
        self.condition_checking = true;
        self.expression();
        self.code_gen(format!("addi r0 0 r2"));
        let condition = self.current.text.clone();
        self.next();
        self.expression();
        self.code_gen(format!("addi r2 0 r1"));
        match condition.as_str() {
            "==" => self.code_gen(format!("jeq r1 r0 {}", self.line_number + 10)),
            "!=" => self.code_gen(format!("jne r1 r0 {}", self.line_number + 10)),
            "<" => self.code_gen(format!("jlt r1 r0 {}", self.line_number + 10)),
            ">" => self.code_gen(format!("jgt r1 r0 {}", self.line_number + 10)),
            "<=" => self.code_gen(format!("jle r1 r0 {}", self.line_number + 10)),
            ">=" => self.code_gen(format!("jge r1 r0 {}", self.line_number + 10)),
            _ => panic!("Unknown condition {}", condition),
        }
        self.condition_checking = false;
    }

    fn program(&mut self) {
        while self.current.kind != TokenType::EOF {
            if self.check_token(TokenType::LET) {
                self._match(TokenType::LET);
                let var_name = self.current.text.clone();
                self._match(TokenType::IDENT);
                if self.symbols.contains_key(&var_name) {
                    panic!("Variable {} already exists", var_name);
                } else {
                    self.symbols.insert(var_name.clone(), self.sym_addr);
                    self.sym_addr += 1;
                }
                self._match(TokenType::EQUAL);
                self.expression();
                self.code_gen(format!("set bp {} bp", self.symbols.get(&var_name).unwrap()));
                self.code_gen(format!("addi r0 0 ram"));
            }
            else if self.check_token(TokenType::IF) {
                self._match(TokenType::IF);
                self.condition();
                let else_jmp_line = self.line_number;
                self.line_number += 5;
                self._match(TokenType::THEN);
                self.in_func = true;
                self.program();
                self._match(TokenType::IF);
                self.in_func = false;
                let end_of_if_line = self.line_number;
                self.line_number = else_jmp_line;
                self.code_gen(format!("{}", self.condition_buffer));
                self.code_gen(format!("jmp 0 0 {}", end_of_if_line));
                self.code_gen(format!("{}", self.func_buffer));
                self.line_number = end_of_if_line + 5;
                self.condition_buffer = String::new();
                self.func_buffer = String::new();
            }
            else if self.check_token(TokenType::WHILE) {
                self._match(TokenType::WHILE);
                let condition_loop_line = self.line_number;
                self.condition();
                let else_jmp_line = self.line_number;
                self.line_number += 5;
                self._match(TokenType::DO);
                self.in_func = true;
                self.program();
                self._match(TokenType::WHILE);
                self.in_func = false;
                let end_of_while_line = self.line_number;
                self.line_number = else_jmp_line;
                self.code_gen(format!("{}", self.condition_buffer));
                self.code_gen(format!("jmp 0 0 {}", end_of_while_line + 5));
                self.code_gen(format!("{}", self.func_buffer));
                self.line_number = end_of_while_line + 5;
                self.code_gen(format!("jmp 0 0 {}", condition_loop_line));
                self.condition_buffer = String::new();
                self.func_buffer = String::new();
            }
            else if self.check_token(TokenType::FOR) {}
            else if self.check_token(TokenType::END) { self.next(); break; }
            else if self.check_token(TokenType::IDENT) {
                let var_name = self.current.text.clone();
                self._match(TokenType::IDENT);
                if !self.symbols.contains_key(&var_name) {
                    panic!("Variable {} does not exist", var_name);
                }
                self._match(TokenType::EQUAL);
                self.expression();
                self.code_gen(format!("set bp {} bp", self.symbols.get(&var_name).unwrap()));
                self.code_gen(format!("addi r0 0 ram"));
            }
            else if self.check_token(TokenType::NEWLINE) { self.next(); }
            else if self.check_token(TokenType::LABEL) {
                self._match(TokenType::LABEL);
                let label_name = self.current.text.clone();
                self._match(TokenType::IDENT);
                if self.labels.contains_key(&label_name) {
                    panic!("Label {} already exists", label_name);
                } else {
                    self.labels.insert(label_name.clone(), self.line_number);
                }
            }
            else if self.check_token(TokenType::GOTO) {
                self._match(TokenType::GOTO);
                let label_name = self.current.text.clone();
                self._match(TokenType::IDENT);
                if !self.labels.contains_key(&label_name) {
                    panic!("Label {} does not exist", label_name);
                }
                self.code_gen(format!("jmp 0 0 {}", self.labels.get(&label_name).unwrap()));
            }
            else {
                panic!("Unexpected token: {}", self.current.text)
            }
        }
    }
}

fn read_file_to_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}

fn main() {
    let time = Instant::now();
    let source = read_file_to_string("src/input.bas").unwrap();
    let mut _lexer = Lexer::new(source);
    let mut parser = Parser::new(_lexer);
    parser.program();
    println!("{}\nDone parsing!\nTime taken: {:?}", parser.main_buffer, time.elapsed());
}