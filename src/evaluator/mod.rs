mod boolean;
pub mod tokenizer;

#[derive(Debug, Clone)]
pub struct AST {
    pub root: Option<Box<ASTNode>>
}

impl AST {
    fn setRoot(&mut self, root: &ASTNode) {
        self.root = Some(Box::new(root.clone()));
    }
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub nType: String,
    pub content: String,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>
}

pub struct Evaluator {
    pub expression: Vec<tokenizer::Token>,
    pub tree: AST
}

impl Evaluator {
    pub fn getAST(&self) -> AST {
        self.tree.clone()
    }
    
    pub fn buildAST(&mut self) {
        self.tree = AST { root: Some(Box::new(self.parseExpr(self.expression.clone()))) };
    }

    fn searchPeak(&self, expression: &Vec<tokenizer::Token>) -> u16 {
        let mut maxVal: u8 = 0;
        let mut pos: u16 = 0;

        for (index, token) in expression.iter().enumerate() {
            if let Some(&currPrec) = tokenizer::precedences.get(&token.tType) {
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

    fn parseExpr(&self, expression: Vec<tokenizer::Token>) -> ASTNode {
        let mut root: ASTNode = ASTNode { nType: "".to_string(), content: "".to_string(), left: None, right: None };

        if expression.is_empty() {
            return root;
        }

        let index = self.searchPeak(&expression);
        let peak = &expression[index as usize];

        if peak.tType == tokenizer::TokenType::BOOL {
            root.nType = "bool".to_string();
            root.content = peak.content.clone();
        }

        if peak.tType == tokenizer::TokenType::BLOCK {
            return self.parseExpr(peak.expression.clone());
        }

        if peak.tType == tokenizer::TokenType::NOT {
            let rhs: Vec<tokenizer::Token> = expression[(index + 1) as usize..].to_vec();
            root.nType = "unaryOp".to_string();
            root.content = peak.content.clone();
            root.right = Some(Box::new(self.parseExpr(rhs)));
            return root;
        }

        if tokenizer::operatorTypes.contains(&peak.tType) && peak.tType != tokenizer::TokenType::NOT {
            
            root.nType = "binaryOp".to_string();
            root.content = peak.content.clone();

            let lhs: Vec<tokenizer::Token> = expression[..index as usize].to_vec();
            let rhs: Vec<tokenizer::Token> = expression[((index+1) as usize)..].to_vec();
            root.left = Some(Box::new(self.parseExpr(lhs)));
            root.right = Some(Box::new(self.parseExpr(rhs)));

        }
        root
    }

    pub fn evaluate(&self, node: &ASTNode) -> String {
        match node.content.as_str() {
            _ if node.nType.as_str() == "bool" => { return node.content.clone() },
            "&&" => { return boolean::AND(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "||" => { return boolean::OR(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "!" => { return boolean::NOT(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "->" => { return boolean::IMPLIES(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "<-" => { return boolean::CONVERSE(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "<->" => { return boolean::EQUIVALENCE(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "^" => { return boolean::XOR(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "!&&" => { return boolean::NAND(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) },
            "!||" => { return boolean::NOR(self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.left).unwrap()), self.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&node.right).unwrap())) }
            _ => {}
        }
        return "".to_string();
    }
}
