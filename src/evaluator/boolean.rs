pub fn NOT(bool1: String) -> String {
    if bool1 == "1".to_string() {
        return "0".to_string()
    }
    "1".to_string()
}

pub fn AND(bool1: String, bool2: String) -> String {
    if bool1 == "1".to_string() && (bool1 == bool2) {
        return "1".to_string()
    }
    "0".to_string()
}

pub fn OR(bool1: String, bool2: String) -> String {
    if bool1 == "1".to_string() || bool2 == "1".to_string() {
        return "1".to_string()
    }
    "0".to_string()
}

pub fn IMPLIES(bool1: String, bool2: String) -> String {
    return OR(NOT(bool1.clone()), bool2.clone())
}

pub fn CONVERSE(bool1: String, bool2: String) -> String {
    return IMPLIES(bool2.clone(), bool1.clone())
}

pub fn EQUIVALENCE(bool1: String, bool2: String) -> String {
    return AND(IMPLIES(bool1.clone(), bool2.clone()), IMPLIES(bool2.clone(), bool1.clone()))
}

pub fn XOR(bool1: String, bool2: String) -> String {
    return NOT(EQUIVALENCE(bool1.clone(), bool2.clone()))
}

pub fn NAND(bool1: String, bool2: String) -> String {
    return NOT(AND(bool1.clone(), bool2.clone()))
}

pub fn NOR(bool1: String, bool2: String) -> String {
    return NOT(OR(bool1.clone(), bool2.clone()))
}
