use std::fmt;

pub type Assembly = Vec<Instruction>;

pub fn print_assembly_code(assembly: &Assembly) {
    for instruction in assembly {
        print_single_instruction(instruction);
    }
}

pub fn print_single_instruction(instruction: &Instruction) {
    match instruction {
        Instruction::Label(_) => println!("{:?}", instruction),
        _ => println!("  {:?}", instruction),
    }
}

#[derive(Clone)]
pub enum Instruction {
    Push(Operand),
    Pop(Operand),
    Ret,
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand),
    IMul(Operand, Operand),
    Idiv(Operand),
    Neg(Operand),
    Cqo,
    Movzb(Operand, Operand),
    Mov(Operand, Operand),
    Cmp(Operand, Operand),
    Sete(Operand),
    Setne(Operand),
    Setl(Operand),
    Setle(Operand),
    Je(String),
    Jmp(String),
    Label(String),
    Comment(String),
    Call(String),
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Push(o) => write!(f, "push {:?}", o),
            Instruction::Pop(o) => write!(f, "pop {:?}", o),
            Instruction::Ret => write!(f, "ret"),
            Instruction::Add(o1, o2) => write!(f, "add {:?}, {:?}", o1, o2),
            Instruction::Sub(o1, o2) => write!(f, "sub {:?}, {:?}", o1, o2),
            Instruction::Mul(o) => write!(f, "mul {:?}", o),
            Instruction::IMul(o1, o2) => write!(f, "imul {:?}, {:?}", o1, o2),
            Instruction::Idiv(o) => write!(f, "idiv {:?}", o),
            Instruction::Neg(o) => write!(f, "neg {:?}", o),
            Instruction::Cqo => write!(f, "cqo"),
            Instruction::Movzb(o1, o2) => write!(f, "movzb {:?}, {:?}", o1, o2),
            Instruction::Mov(o1, o2) => write!(f, "mov {:?}, {:?}", o1, o2),
            Instruction::Cmp(o1, o2) => write!(f, "cmp {:?}, {:?}", o1, o2),
            Instruction::Sete(o) => write!(f, "sete {:?}", o),
            Instruction::Setne(o) => write!(f, "setne {:?}", o),
            Instruction::Setl(o) => write!(f, "setl {:?}", o),
            Instruction::Setle(o) => write!(f, "setle {:?}", o),
            Instruction::Je(label) => write!(f, "je {}", label),
            Instruction::Jmp(label) => write!(f, "jmp {}", label),
            Instruction::Label(label) => write!(f, "{}:", label),
            Instruction::Comment(comment) => write!(f, "// {}", comment),
            Instruction::Call(label) => write!(f, "call {}", label),
        }
    }
}

pub fn push(operand: Operand) -> Instruction {
    Instruction::Push(operand)
}

pub fn pop(operand: Operand) -> Instruction {
    Instruction::Pop(operand)
}

pub fn ret() -> Instruction {
    Instruction::Ret
}

pub fn add(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::Add(operand1, operand2)
}

pub fn sub(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::Sub(operand1, operand2)
}

pub fn mul(operand: Operand) -> Instruction {
    Instruction::Mul(operand)
}

pub fn imul(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::IMul(operand1, operand2)
}

pub fn idiv(operand: Operand) -> Instruction {
    Instruction::Idiv(operand)
}

pub fn cqo() -> Instruction {
    Instruction::Cqo
}

pub fn neg(operand: Operand) -> Instruction {
    Instruction::Neg(operand)
}

pub fn movzb(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::Movzb(operand1, operand2)
}

pub fn mov(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::Mov(operand1, operand2)
}

pub fn cmp(operand1: Operand, operand2: Operand) -> Instruction {
    Instruction::Cmp(operand1, operand2)
}

pub fn sete(operand: Operand) -> Instruction {
    Instruction::Sete(operand)
}

pub fn setne(operand: Operand) -> Instruction {
    Instruction::Setne(operand)
}

pub fn setl(operand: Operand) -> Instruction {
    Instruction::Setl(operand)
}

pub fn setle(operand: Operand) -> Instruction {
    Instruction::Setle(operand)
}

pub fn je(label: String) -> Instruction {
    Instruction::Je(label)
}

pub fn jmp(label: String) -> Instruction {
    Instruction::Jmp(label)
}

pub fn label(label: String) -> Instruction {
    Instruction::Label(label)
}

pub fn comment(comment: &str) -> Instruction {
    Instruction::Comment(comment.to_string())
}

pub fn call(label: String) -> Instruction {
    Instruction::Call(label)
}

#[derive(Clone)]
pub enum Operand {
    Register(Register),
    Immediate(i32),
    Memory(Register),
}

pub fn immediate(value: i32) -> Operand {
    Operand::Immediate(value)
}

impl fmt::Debug for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{:?}", r),
            Operand::Immediate(i) => write!(f, "{}", i),
            Operand::Memory(r) => write!(f, "[{:?}]", r),
        }
    }
}

#[derive(Clone)]
pub enum Register {
    RAX,
    RBP,
    RDI,
    RSI,
    RSP,
    RDX,
    RCX,
    R8,
    R9,
    AL,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::RAX => write!(f, "rax"),
            Register::RBP => write!(f, "rbp"),
            Register::RDI => write!(f, "rdi"),
            Register::RSI => write!(f, "rsi"),
            Register::RSP => write!(f, "rsp"),
            Register::RDX => write!(f, "rdx"),
            Register::RCX => write!(f, "rcx"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::AL => write!(f, "al"),
        }
    }
}

pub fn rax() -> Operand {
    Operand::Register(Register::RAX)
}

pub fn m_rax() -> Operand {
    Operand::Memory(Register::RAX)
}

pub fn rbp() -> Operand {
    Operand::Register(Register::RBP)
}

pub fn rdi() -> Operand {
    Operand::Register(Register::RDI)
}

pub fn rsp() -> Operand {
    Operand::Register(Register::RSP)
}

pub fn al() -> Operand {
    Operand::Register(Register::AL)
}
