use std::collections::HashMap;
use crate::evaluator::{AST, ASTNode, Evaluator};
use crate::evaluator::tokenizer::{Token, TokenType, Tokenizer, checkSyntax, mapToBool, parseBlocks};

pub fn commandFinder(line: &Vec<Token>) -> String {
    if line.is_empty() {
        return "Es wurde nichts übergeben!".to_string();
    }
    match line[0].tType {
        TokenType::EXPRESSION => { return commandEvaluate(line) },
        TokenType::COMMAND => { 
            if line[0].content == "table".to_string() { return commandTable(line); }
            else { return "".to_string(); }
        },
        _ => { return "Das ist ein unbekannter Befehl. Nutze einen der gegebenen".to_string(); }
    }
}

pub fn commandHelp() -> String {
    return "Diese Terminal-Applikation soll zum evaluieren von booleschen Formeln dienen. Zudem gibt es nützliche Befehle, welche z.B. die konjunktive oder disjunktive Normalform einer Formel wiedergeben. \r\nWird in einem Befehl nach <AUSDRUCK> gefragt, handelt es sich hierbei, um eine boolesche Formel innerhalb von Rechtecksklammern (also die hier: [])\r\n\nBEFEHLE:\r\t\n<AUSDRUCK> <VARIABLE1> <0/1> <VARIABLE2> <0/1> ... <VARIABLEn> <0/1>                 Evaluiert den gegebenen Ausdruck mit den gegebenen Variablenbelegungen\r\t\ntable <AUSDRUCK>                                                                     Gibt die Wahrheitswertetabelle der Formel wieder".to_string()
}

fn commandEvaluate(line: &Vec<Token>) -> String {
    let mut idToBool: HashMap<String, String> = HashMap::new();
    let mut i: u16 = 1;
    let expr = line[0].clone();

    while i < (line.len() as u16) {
        if line[i as usize].tType == TokenType::EOL {
            break;
        } 
        if (i+1) == (line.len() as u16) {
            return format!("Es ist ein Fehler an der Stelle {} aufgetreten. Vielleicht fehlt hier ein boolean für die korrespondierende Variable", i);
            
        } 
        if line[i as usize].tType == TokenType::IDENTIFIER && line[(i+1) as usize].tType == TokenType::BOOL {
           idToBool.insert(line[i as usize].content.clone(), line[(i+1) as usize].content.clone());
           i += 2;
        }
        else {
            i += 1;
            return format!("Es ist ein Fehler an der Stelle {} aufgetreten (Inhalt: {}). Vielleicht wurde die Reihenfolge von <VARIABLE> <0/1> nicht eingehalten oder es wurde irgendwas unerwartetes gefunden", i, line[i as usize].content.clone());
        }
    }

    if checkSyntax(&expr.expression) {
        let mut tokens = expr.expression;
        mapToBool(&mut tokens, idToBool);
        tokens = parseBlocks(&mut tokens);

        let mut ev = Evaluator {
            expression: tokens.clone(),
            tree: AST { root: None }
        };

        ev.buildAST();
        let a = ev.getAST();

        let result = ev.evaluate(&<Option<Box<ASTNode>> as Clone>::clone(&a.root).unwrap());

        return format!("Ergebnis: {}", result);
        } else {
            return "Es ist etwas mit der Syntax des Ausdruckes falsch!".to_string();
        }
}

fn commandTable(line: &Vec<Token>) -> String {
    return "".to_string();
}
