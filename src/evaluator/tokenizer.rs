use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::{fmt, usize};

static operators: [&str; 9] = ["&&", "||", "!", "->", "<-", "<->", "^", "!&&", "!||"];
static commands: [&str; 9] = ["exit", "table", "valid", "satis", "semcons", "cnf", "dnf", "latex", "tree"];
pub static operatorTypes: [TokenType; 9] = [
    TokenType::NOT, TokenType::AND, TokenType::OR, TokenType::IMPLIES, TokenType::CONVERSE, TokenType::EQUIVALENCE, TokenType::XOR, TokenType::NAND, TokenType::NOR
];

static operatorMap: Lazy<HashMap<String, TokenType>> = Lazy::new(|| {
    HashMap::from([
        ("&&".to_string(), TokenType::AND),
        ("||".to_string(), TokenType::OR),
        ("!".to_string(), TokenType::NOT),
        ("->".to_string(), TokenType::IMPLIES),
        ("<-".to_string(), TokenType::CONVERSE),
        ("<->".to_string(), TokenType::EQUIVALENCE),
        ("^".to_string(), TokenType::XOR),
        ("!&&".to_string(), TokenType::NAND),
        ("!||".to_string(), TokenType::NOR)
    ])
});

pub static precedences: Lazy<HashMap<TokenType, u8>> = Lazy::new(|| {
    HashMap::from([
        (TokenType::EOL, 0),
        (TokenType::BOOL, 0),
        (TokenType::BLOCK, 0),
        (TokenType::NOT, 1),
        (TokenType::AND, 3),
        (TokenType::OR, 4),
        (TokenType::IMPLIES, 5),
        (TokenType::CONVERSE, 5),
        (TokenType::EQUIVALENCE, 6),
        (TokenType::XOR, 2),
        (TokenType::NAND, 2),
        (TokenType::NOR, 2)
    ])
});


#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum TokenType {
    EOL,
    ERROR,
    IDENTIFIER,
    COMMAND,
    BOOL,
    EXPRESSION,
    LPAREN,
    RPAREN,
    BLOCK,

    NOT,
    AND,
    OR,
    IMPLIES,
    CONVERSE,
    EQUIVALENCE,
    XOR,
    NAND,
    NOR,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub tType: TokenType,
    pub content: String,
    pub expression: Vec<Token>,
    pub start: u16,
    pub end: u16
}

pub struct Tokenizer {
    pub line: String,
    pub index: u16,
    pub tokens: Vec<Token>,
    pub current: char
}

fn printTokens(tokens: &Vec<Token>) {
    for token in tokens {
        if token.tType == TokenType::EXPRESSION || token.tType == TokenType::BLOCK {
            print!("[{}] [ ", token.tType);
            for tok in &token.expression {
                print!("{} ", tok);
            }
            print!("] {}..{}",token.start,token.end);
        } else {
            print!("{}", token);
        }
    }
}

pub fn checkSyntax(expression: &Vec<Token>) -> bool {
    let mut i: u16 = 0;
    let nullToken = Token { 
        tType: TokenType::EOL, 
        content: "".to_string(), 
        expression: vec![], 
        start: 0, 
        end: 0 
    };

    while i < expression.len() as u16 {
        let curr = &expression[i as usize];

        let next = if i + 1 < expression.len() as u16 {
            &expression[(i + 1) as usize]
        } else {
            &nullToken
        };

        match curr.tType {
            TokenType::BOOL => {
                if next.tType == TokenType::EOL || operatorTypes.contains(&next.tType) || next.tType == TokenType::RPAREN {
                    i += 1;
                    continue;
                }
                return false;
            }
            TokenType::IDENTIFIER => {
                if next.tType == TokenType::EOL || operatorTypes.contains(&next.tType) || next.tType == TokenType::RPAREN {
                    i += 1;
                    continue;
                }
                return false;
            }
            _ if operatorTypes.contains(&curr.tType) => {
                if (curr.tType != TokenType::NOT) && (next.tType == TokenType::BOOL || next.tType == TokenType::IDENTIFIER || operatorTypes.contains(&next.tType) || next.tType == TokenType::LPAREN) {
                    i += 1;
                    continue;
                } else if (curr.tType == TokenType::NOT) && (next.tType == TokenType::BOOL || next.tType == TokenType::IDENTIFIER || next.tType == TokenType::LPAREN) {
                    i += 1;
                    continue;
                }
                return false
            }
            TokenType::LPAREN => {
                if next.tType == TokenType::LPAREN || next.tType == TokenType::BOOL || next.tType == TokenType::IDENTIFIER || next.tType == TokenType::NOT {
                    i += 1;
                    continue;
                }
                return false;
            }
            TokenType::RPAREN => {
                if (operatorTypes.contains(&curr.tType) && (next.tType != TokenType::NOT)) || next.tType == TokenType::RPAREN || next.tType == TokenType::EOL {
                    i += 1;
                    continue;
                }
                return false;
            }
            TokenType::EOL => { break }
            _ => { return false; }
        }
    }

    true
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::EOL => write!(f, "EOL"),
            TokenType::ERROR => write!(f, "ERROR"),
            TokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            TokenType::COMMAND => write!(f, "COMMAND"),
            TokenType::BOOL => write!(f, "BOOL"),
            TokenType::EXPRESSION => write!(f, "EXPRESSION"),
            TokenType::LPAREN => write!(f, "LPAREN"),
            TokenType::RPAREN => write!(f, "RPAREN"),
            TokenType::BLOCK => write!(f, "BLOCK"),
            TokenType::NOT => write!(f, "NOT"),
            TokenType::AND => write!(f, "AND"),
            TokenType::OR => write!(f, "OR"),
            TokenType::IMPLIES => write!(f, "IMPLIES"),
            TokenType::CONVERSE => write!(f, "CONVERSE"),
            TokenType::EQUIVALENCE => write!(f, "EQUIVALENCE"),
            TokenType::XOR => write!(f, "XOR"),
            TokenType::NAND => write!(f, "NAND"),
            TokenType::NOR => write!(f, "NOR")
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("[{}] {} {:?} {}..{} ", self.tType, self.content, self.expression, self.start, self.end))
    }
}

impl Tokenizer {
   fn forward(&mut self) {
        if self.index < self.line.len() as u16 {
            self.index += 1;
            self.current = if self.index < self.line.len() as u16 {
                self.line.chars().nth(self.index as usize).unwrap_or('\0')
            } else {
                '\0'
            };
        } else {
            self.current = '\0';
        }
    }

    fn makeIdentifier(&mut self) -> Token {
        let mut identifier = String::new();
        identifier.push(self.current);
        let start: u16 = self.index;

        self.forward();
        while self.current.is_alphanumeric() {
            identifier.push(self.current);
            self.forward();
        }

        if commands.contains(&identifier.as_str()) {
            return Token {
                tType: TokenType::COMMAND,
                content: identifier,
                expression: vec![],
                start,
                end: self.index
            }
        }

        Token {
            tType: TokenType::IDENTIFIER,
            content: identifier,
            expression: vec![],
            start,
            end: self.index
        }
    }

    fn makeOperator(&mut self) -> Token {
        let mut operator = String::new();
        operator.push(self.current);
        let start: u16 = self.index;

        self.forward();
        while "&|!-^<>".contains(self.current) {
            operator.push(self.current);
            self.forward();
        }

        if operators.contains(&operator.as_str()) {
            return Token {
                tType: operatorMap.get(&operator).unwrap().clone(),
                content: operator,
                expression: vec![],
                start,
                end: self.index-1
            }
        }

        Token {
            tType: TokenType::ERROR,
            content: operator,
            expression: vec![],
            start,
            end: self.index
        }
    }

    fn makeExpression(&mut self) -> Token {
        let mut expression: Vec<Token> = vec![];
        let mut exprStr = String::new();
        let start: u16 = self.index;

        self.forward();

        while self.current != ']' {
            if self.current == '\0' {
                break;
            }
            exprStr.push(self.current);
            self.forward()
        }

        if self.current == '\0' {
            println!("Da hat jemand die schließende Klammer von der Klammer an der Stelle {} vergessenfür.", start);
            return Token {
                tType: TokenType::ERROR,
                content: exprStr,
                expression: vec![],
                start,
                end: self.index
            };
        }

        let mut tokenizer = Tokenizer { 
            line: exprStr, 
            index: 0, 
            tokens: vec![], 
            current: '\0' 
        };
        
        tokenizer.tokenize();
        expression = tokenizer.tokens;

        Token { 
            tType: TokenType::EXPRESSION, 
            content: "".to_string(), 
            expression, 
            start, 
            end: self.index 
        }
    }

    pub fn tokenize(&mut self) {
        self.current = self.line.chars().nth(self.index as usize).unwrap();
        while self.current != '\0' {

            if self.current.is_whitespace() {
                self.forward();
                continue;
            }

            if self.current.is_alphabetic() {
                let identifier = self.makeIdentifier();
                self.tokens.push(identifier);
                continue;
            }

            if (self.current == '0') || (self.current == '1') {
                self.tokens.push(Token { 
                    tType: TokenType::BOOL,
                    content: self.current.to_string(),
                    expression: vec![],
                    start: self.index,
                    end: self.index
                });
                self.forward();
                continue;
            }

            if self.current == '[' {
                let expression = self.makeExpression();
                self.tokens.push(expression);
                self.forward();
                continue;
            }

            if "&|!-^<".contains(self.current) {
                let operator = self.makeOperator();
                self.tokens.push(operator);
                continue;
            }

            match self.current {
                '(' => self.tokens.push(Token { tType: TokenType::LPAREN, content: self.current.to_string(), expression: vec![], start: self.index, end: self.index }),
                ')' => self.tokens.push(Token { tType: TokenType::RPAREN, content: self.current.to_string(), expression: vec![], start: self.index, end: self.index }),
                _ => self.tokens.push(Token { tType: TokenType::ERROR, content: self.current.to_string(), expression: vec![], start: self.index, end: self.index })
            }

            self.forward();
        }

        self.tokens.push(Token { tType: TokenType::EOL, content: "".to_string(), expression: vec![], start: self.index, end: self.index });
    }
}

pub fn mapToBool(expression: &mut Vec<Token>, idToBool: HashMap<String, String>) {
    let exp = expression.clone();
    for (i, token) in exp.iter().enumerate().clone() {
        if (token.tType == TokenType::IDENTIFIER) && (idToBool.contains_key(&token.content)) {
            expression[i as usize].tType = TokenType::BOOL;
            expression[i as usize].content = (*idToBool.get(&token.content).unwrap().clone()).to_string();
        }
    }
}

fn getBlock(entryPoint: u16, expression: Vec<Token>) -> (bool, (Token, u16)) {
    let mut exprStack: Vec<Token> = vec![];
    let mut i: u16 = entryPoint+1;
    let exprStart: u16 = expression[entryPoint as usize].start;
    let mut exprEnd: u16 = 0;
    let mut open: u16 = 1;

    while i < (expression.len() as u16) {
        let curr = &expression[i as usize];
        if open > 0 && curr.tType == TokenType::LPAREN {
            open += 1;
            exprStack.push(curr.clone());
            i += 1;
            continue;
        }
        else if open > 0 && curr.tType == TokenType::RPAREN {
            open -= 1;
            if open != 0 {
                exprStack.push(curr.clone());
                i += 1;
                continue;
            }
            else {
                exprEnd = curr.end;
                return (true, (Token { tType: TokenType::BLOCK, content: "".to_string(), expression: parseBlocks(&mut exprStack), start: exprStart, end: exprEnd }, i));
            }
        }
        else {
            exprStack.push(curr.clone());
            i += 1;
        }
    }
    return (false, (Token { tType: TokenType::BLOCK, content: "".to_string(), expression: vec![], start: 0, end: 0 }, 0));
}

pub fn parseBlocks(expression: &mut Vec<Token>) -> Vec<Token> {
    let mut i: u16 = 0;
        
    while i < (expression.len() as u16) {
        let curr = &expression[i as usize];
        if curr.tType == TokenType::LPAREN {
            let result = getBlock(i, expression.to_vec());
            if result.0 {
                expression[i as usize] = (result.1).0;
                for _ in 0..(result.1).1-i {
                    expression.remove((i+1) as usize);
                }
            }
        }
        i += 1;
    }
    return expression.clone();
}

