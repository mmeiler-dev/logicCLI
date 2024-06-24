use std::ptr::null;
use std::fmt::{self, write};
use std::collections::HashMap;

static operators: [&str; 9] = ["&&", "||", "!", "->", "<-", "<->", "^", "!&&", "!||"];
static commands: [&str; 9] = ["exit", "table", "valid", "satis", "semcons", "cnf", "dnf", "latex", "tree"];
static operatorTypes [tokenType; 9] = [tokenType::NOT, tokenType::AND, tokenType::OR, tokenType::IMPLIES, tokenType::CONVERSE, tokenType::EQUIVALENCE, tokenType::XOR, tokenType::NOR];
static precedences: HashMap<tokenType, u8> = HashMap::from([(tokenType::BOOL, 0), (tokenType::BLOCK, 0), (tokenType::OP)]);


fn main() {
    let mut tok = Tokenizer { line: "[!b <-> (0 || 1)]".to_string(), index: 0, tokens: vec![], current: '\0' };

    tok.tokenize();
    tok.print();

    for token in tok.tokens {
        if token.tType == tokenType::EXPRESSION {
            if checkSyntax(&token.expression) {
                println!(" ");
                println!("everything is ok!");
            }
            else {
                println!(" ");
                println!("something in the logic is false");
            }
        }
    }

    println!("{:?}", AST { root: Some(ASTNode { nType: "bool".to_string() , content: "1".to_string(), left: None, right: None }) });
}

#[derive(PartialEq, Debug)]
enum tokenType {
    EOL,
    ERROR,
    IDENTIFIER,
    COMMAND,
    BOOL,
    EXPRESSION,
    LPAREN,
    RPAREN,
    BLOCK,
    COLON,

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

#[derive(Debug)]
struct Token {
    tType: tokenType,
    content: String,
    expression: Vec<Token>,
    start: u16,
    end: u16
}

struct Tokenizer {
    line: String,
    index: u16,
    tokens: Vec<Token>,
    current: char
}

fn checkSyntax(expression: &Vec<Token>) -> bool {
    let mut i: u16 = 0;
    let nullToken = Token { tType: tokenType::EOL, content: "".to_string(), expression: vec![], start: 0, end: 0 };  

    while i < (expression.len() as u16) { 
        let mut curr = &expression[i as usize];

        let next = if i + 1 < (expression.len() as u16) {
            &expression[(i+1) as usize]
        } else {
            &nullToken
        };

        //println!("{}", i);
        match curr {
            _ if curr.tType == tokenType::BOOL => {
                if (*next).tType == tokenType::EOL || (*next).tType == tokenType::OPERATOR || (*next).tType == tokenType::RPAREN {
                    i += 1;
                    continue;
                } 
                return false;
            }
            _ if curr.tType == tokenType::IDENTIFIER => {
                if (*next).tType == tokenType::EOL || (*next).tType == tokenType::OPERATOR || (*next).tType == tokenType::RPAREN {
                    i += 1;
                    continue;
                }
                return false;
            } 
            _ if curr.tType == !tokenType::NOT => {
                if (curr.content != "!".to_string()) && ((*next).tType == tokenType::BOOL || (*next).tType == tokenType::IDENTIFIER || ((*next).tType == tokenType::OPERATOR && ((*next).content == "!".to_string())) || (*next).tType == tokenType::LPAREN) {
                    i += 1;
                    continue;
                }
                else if (curr.content == "!".to_string()) && ((*next).tType == tokenType::BOOL || (*next).tType == tokenType::IDENTIFIER || (*next).tType == tokenType::LPAREN) {
                    i += 1;
                    continue;
                }
                return false
            }
            _ if curr.tType == tokenType::LPAREN => {
                if (*next).tType == tokenType::LPAREN || (*next).tType == tokenType::BOOL || (*next).tType == tokenType::IDENTIFIER || (*next).content == "!".to_string() {
                    i += 1;
                    continue;
                }
                return false;
            }
            _ if curr.tType == tokenType::RPAREN => {
                if (((*next).tType == tokenType::OPERATOR) && ((*next).content != "!".to_string())) || (*next).tType == tokenType::RPAREN || (*next).tType == tokenType::EOL {
                    i += 1;
                    continue;
                } 
                return false;
            }
            _ if curr.tType == tokenType::EOL => { break }
            _ => { return false; }
        }
    }

    true
}

impl fmt::Display for tokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            tokenType::EOL => write!(f, "EOL"),
            tokenType::ERROR => write!(f, "ERROR"),
            tokenType::IDENTIFIER => write!(f, "IDENTIFIER"),
            tokenType::COMMAND => write!(f, "COMMAND"),
            tokenType::BOOL => write!(f, "BOOL"),
            tokenType::OPERATOR => write!(f, "OPERATOR"),
            tokenType::EXPRESSION => write!(f, "EXPRESSION"),
            tokenType::LPAREN => write!(f, "LPAREN"),
            tokenType::RPAREN => write!(f, "RPAREN"),
            tokenType::BLOCK => write!(f, "BLOCK"),
            tokenType::COLON => write!(f, "COLON")
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("[{}] {} {:?} {}..{} ", self.tType, self.content, self.expression, self.start, self.end))
    }
}

impl Tokenizer {
    fn print(&self) {
        for token in &self.tokens {
            if token.tType == tokenType::EXPRESSION || token.tType == tokenType::BLOCK {
                print!("[{}] {:?} {}..{}", token.tType, token.expression, token.start, token.end);
            }
            else {
                print!("{}", token);
            }
        }
    }

    fn forward(&mut self) {
        self.index += 1;
        if self.index < self.line.chars().count() as u16 {
            self.current = self.line.chars().nth(self.index as usize).unwrap();
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
            return Token{tType: tokenType::COMMAND, content: identifier, expression: vec![], start: start, end: self.index-1};
        }

        Token{tType: tokenType::IDENTIFIER, content: identifier, expression: vec![], start: start, end: self.index-1}
    }

    fn makeNumber(&mut self) -> Token {
        let mut boolean = String::new();
        boolean.push(self.current);
        let start: u16 = self.index;

        self.forward();

        while self.current.is_numeric() {
            boolean.push(self.current);
            self.forward();
        }

        if boolean.len() == 1 {
            return Token{tType: tokenType::BOOL, content: boolean, expression: vec![], start: start, end: start};
        }

        Token{tType: tokenType::ERROR, content: boolean, expression: vec![], start, end: self.index-1}
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
            return Token{tType: tokenType::OPERATOR, content: operator, expression: vec![], start: start, end: self.index-1};
        }

        Token{tType: tokenType::ERROR, content: operator, expression: vec![], start: start, end: self.index-1}
    }

    fn makeExpression(&mut self) -> Token {
        let mut expression: Vec<Token> = vec![];
        let mut exprStr = String::new();
        let start: u16 = self.index;

        self.forward();

        while self.current != ']' {
            exprStr.push(self.current);
            self.forward()
        }

        let mut tokenizer = Tokenizer { line: exprStr , index: 0, tokens: vec![], current: '\0' };
        tokenizer.tokenize();
        expression = tokenizer.tokens;

        Token { tType: tokenType::EXPRESSION, content: "".to_string(), expression: expression, start: start, end: self.index }
    }

    fn tokenize(&mut self) {
        if self.line.len() == 0 {
            return
        }
        self.current = self.line.chars().nth(self.index as usize).unwrap();

        while self.current != '\0' {
            match self.current {
                _ if self.current.is_alphabetic() => {
                    let identifier = self.makeIdentifier();
                    self.tokens.push(identifier);
                }
                _ if self.current.is_numeric() => {
                    let number = self.makeNumber();
                    self.tokens.push(number);
                }
                _ if "&|!-^<".contains(self.current) => {
                    let operator = self.makeOperator();
                    self.tokens.push(operator);
                }
                _ if self.current == '[' => {
                    let expression = self.makeExpression();
                    self.tokens.push(expression);
                }
                _ if self.current == ':' => {
                    self.tokens.push(Token { tType: tokenType::COLON, content: ":".to_string(), expression: vec![], start: self.index, end: self.index });
                    self.forward();
                }
                _ if self.current == '(' => {
                    self.tokens.push(Token { tType: tokenType::LPAREN, content: "(".to_string(), expression: vec![], start: self.index, end: self.index });
                    self.forward();
                }
                _ if self.current == ')' => {
                    self.tokens.push(Token { tType: tokenType::RPAREN, content: ")".to_string(), expression: vec![], start: self.index, end: self.index });
                    self.forward();
                }
                _ => self.forward() 
            }
        }

        self.tokens.push(Token { tType: tokenType::EOL, content: "".to_string(), expression: vec![], start: self.index, end: self.index })
    }
}

fn NOT(bool1: String) -> String {
    if bool1 == "1".to_string() {
        return "0".to_string()
    }
    "1".to_string()
}

fn AND(bool1: String, bool2: String) -> String {
    if bool1 == "1".to_string() && (bool1 == bool2) {
        return "1".to_string()
    }
    "0".to_string()
}

fn OR(bool1: String, bool2: String) -> String {
    if bool1 == "1".to_string() || bool2 == "1".to_string() {
        return "1".to_string()
    }
    "0".to_string()
}

fn IMPLIES(bool1: String, bool2: String) -> String {
    return OR(NOT(bool1.clone()), bool2.clone())
}

fn CONVERSE(bool1: String, bool2: String) -> String {
    return IMPLIES(bool2.clone(), bool1.clone())
}

fn EQUIVALENCE(bool1: String, bool2: String) -> String {
    return AND(IMPLIES(bool1.clone(), bool2.clone()), IMPLIES(bool2.clone(), bool1.clone()))
}

fn XOR(bool1: String, bool2: String) -> String {
    return NOT(EQUIVALENCE(bool1.clone(), bool2.clone()))
}

fn NAND(bool1: String, bool2: String) -> String {
    return NOT(AND(bool1.clone(), bool2.clone()))
}

fn NOR(bool1: String, bool2: String) -> String {
    return NOT(OR(bool1.clone(), bool2.clone()))
}

#[derive(Debug)]
struct AST<'a> {
    root: Option<ASTNode<'a>>
}

#[derive(Debug)]
struct ASTNode<'a> {
    nType: String,
    content: String,
    left: Option<&'a ASTNode<'a>>,
    right: Option<&'a ASTNode<'a>>
}

impl ASTNode {
    fn addLeft(&mut self, left: ASTNode) {
        self.left = left;
    }

    fn addRight(&mut self, right: ASTNode) {
        self.right = right;
    }
}

struct Evaluator {
    tokens: Vec<Token>
}

impl Evaluator {
    fn buildTree() -> AST {
        AST { root: None }
    } 
}
