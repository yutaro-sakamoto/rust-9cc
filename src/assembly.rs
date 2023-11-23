use std::fmt;

pub type Assembly = Vec<Instruction>;

pub fn print_assembly_code(assembly: &Assembly) {
    for instruction in assembly {
        print_single_instruction(instruction);
    }
}

pub fn print_single_instruction(instruction: &Instruction) {
    println!("  {:?}", instruction);
}

pub enum Instruction {
    Push(Operand),
    Pop(Operand),
    Ret,
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand),
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

pub enum Register {
    RAX,
    RBP,
    RDI,
    RSP,
    AL,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::RAX => write!(f, "rax"),
            Register::RBP => write!(f, "rbp"),
            Register::RDI => write!(f, "rdi"),
            Register::RSP => write!(f, "rsp"),
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
