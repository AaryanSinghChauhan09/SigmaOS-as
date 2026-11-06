//! Assembly instruction model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Mov, Add, Sub, Mul, Div, And, Or, Xor, Shl, Shr,
    Cmp, Jmp, Je, Jne, Jg, Jl, Jge, Jle, Call, Ret,
    Push, Pop, Nop,
}

impl Opcode {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "mov" => Some(Self::Mov), "add" => Some(Self::Add), "sub" => Some(Self::Sub),
            "mul" => Some(Self::Mul), "div" => Some(Self::Div), "and" => Some(Self::And),
            "or" => Some(Self::Or), "xor" => Some(Self::Xor), "shl" => Some(Self::Shl),
            "shr" => Some(Self::Shr), "cmp" => Some(Self::Cmp), "jmp" => Some(Self::Jmp),
            "je" => Some(Self::Je), "jne" => Some(Self::Jne), "jg" => Some(Self::Jg),
            "jl" => Some(Self::Jl), "jge" => Some(Self::Jge), "jle" => Some(Self::Jle),
            "call" => Some(Self::Call), "ret" => Some(Self::Ret), "push" => Some(Self::Push),
            "pop" => Some(Self::Pop), "nop" => Some(Self::Nop),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operand {
    Register(String),
    Immediate(i64),
    Label(String),
    Memory(Box<Operand>), // indirect
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl Instruction {
    pub fn new(opcode: Opcode, operands: Vec<Operand>) -> Self {
        Self { opcode, operands }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opcode_parsing() {
        assert_eq!(Opcode::from_str("mov"), Some(Opcode::Mov));
        assert_eq!(Opcode::from_str("ADD"), Some(Opcode::Add));
        assert!(Opcode::from_str("bogus").is_none());
    }
}
