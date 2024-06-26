use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::iter::from_fn;
use std::{fmt, result, usize};

static operators: [&str; 9] = ["&&", "||", "!", "->", "<-", "<->", "^", "!&&", "!||"];
static commands: [&str; 9] = ["exit", "table", "valid", "satis", "semcons", "cnf", "dnf", "latex", "tree"];
static operatorTypes: [TokenType; 9] = [
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

static precedences: Lazy<HashMap<TokenType, u8>> = Lazy::new(|| {
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

fn main() {
    let mut tok = Tokenizer {
        line: "[1 && (0 || b)]".to_string(),
        index: 0,
        tokens: vec![],
        current: '\0'
    };

    tok.tokenize();
    //printTokens(&tok.tokens);

        for mut token in tok.tokens {
        if token.tType == TokenType::EXPRESSION {
            if checkSyntax(&token.expression) {
                let mut tokens = token.expression;
                mapToBool(&mut tokens, HashMap::from([("b".to_string(), "0".to_string())]));
                tokens = parseBlocks(&mut tokens);
                //printTokens(&tokens);
                //println!("h");
                let mut ev = Evaluator {
                    expression: tokens.clone(),
                    tree: AST { root: None }
                };

                ev.buildAST();
                let a = ev.getAST();

                //println!("{:?}",ev.expression.clone());

                println!("{:#?}", a);
 
                    //println!("everything is ok!");
            } else {
                println!(" ");
                println!("something in the logic is false");
            }
        }
    }

    // println!("{:?}", AST { root: Some(ASTNode { n_type: "bool".to_string() , content: "1".to_string(), left: None, right: None }) });

    let mut ast = AST { root: None };
    let root = ASTNode {
        nType: "bool".to_string(),
        content: "1".to_string(),
        left: None,
        right: None
    };

    ast.setRoot(&root);

    //println!("{:#?}", ast);
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum TokenType {
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

#[derive(Clone, Debug)]
struct Token {
    tType: TokenType,
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

fn checkSyntax(expression: &Vec<Token>) -> bool {
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
            TokenType::COLON => write!(f, "COLON"),
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
            exprStr.push(self.current);
            self.forward()
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

    fn tokenize(&mut self) {
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

            if self.current == '0' || self.current == '1' {
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
                ':' => self.tokens.push(Token { tType: TokenType::COLON, content: self.current.to_string(), expression: vec![], start: self.index, end: self.index }),
                _ => self.tokens.push(Token { tType: TokenType::ERROR, content: self.current.to_string(), expression: vec![], start: self.index, end: self.index })
            }

            self.forward();
        }

        self.tokens.push(Token { tType: TokenType::EOL, content: "".to_string(), expression: vec![], start: self.index, end: self.index });
    }
}

fn mapToBool(expression: &mut Vec<Token>, idToBool: HashMap<String, String>) {
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

fn parseBlocks(expression: &mut Vec<Token>) -> Vec<Token> {
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

#[derive(Debug, Clone)]
struct AST {
    root: Option<ASTNode>
}

impl AST {
    fn setRoot(&mut self, root: &ASTNode) {
        self.root = Some(root.clone());
    }
}

#[derive(Debug, Clone)]
struct ASTNode {
    nType: String,
    content: String,
    left: Option<Box<ASTNode>>,
    right: Option<Box<ASTNode>>
}

struct Evaluator {
    expression: Vec<Token>,
    tree: AST
}

impl Evaluator {
    fn getAST(&self) -> AST {
        self.tree.clone()
    }
    
    fn buildAST(&mut self) {
        self.tree = AST { root: Some(self.parseExpr(self.expression.clone())) };
    }

    fn searchPeak(&self, expression: &Vec<Token>) -> u16 {
        let mut maxVal: u8 = 0;
        let mut pos: u16 = 0;

        for (index, token) in expression.iter().enumerate() {
            if let Some(&currPrec) = precedences.get(&token.tType) {
                if currPrec > maxVal {
                    maxVal = currPrec;
                    pos = index as u16;
                }
            } else {
                println!("Warning: Token type {:?} not found in precedences", token.tType);
            }
        }
        pos
    }

    fn parseExpr(&self, expression: Vec<Token>) -> ASTNode {
        let mut root: ASTNode = ASTNode { nType: "".to_string(), content: "".to_string(), left: None, right: None };

        if expression.is_empty() {
            return root;
        }

        let index = self.searchPeak(&expression);
        let peak = &expression[index as usize];

        if peak.tType == TokenType::BOOL {
            root.nType = "bool".to_string();
            root.content = peak.content.clone();
        }

        if peak.tType == TokenType::BLOCK {
            return self.parseExpr(peak.expression.clone());
        }

        if peak.tType == TokenType::NOT {
            let rhs: Vec<Token> = expression[(index + 1) as usize..].to_vec();
            root.nType = "unaryOp".to_string();
            root.content = peak.content.clone();
            root.right = Some(Box::new(self.parseExpr(rhs)));
            return root;
        }

        if operatorTypes.contains(&peak.tType) && peak.tType != TokenType::NOT {
            
            root.nType = "binaryOp".to_string();
            root.content = peak.content.clone();

            let lhs: Vec<Token> = expression[..index as usize].to_vec();
            let rhs: Vec<Token> = expression[((index+1) as usize)..].to_vec();
            root.left = Some(Box::new(self.parseExpr(lhs)));
            root.right = Some(Box::new(self.parseExpr(rhs)));

        }
        root
    }
}
