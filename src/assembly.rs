use std::fmt;

pub type Assembly = Vec<Instruction>;

pub fn print_assembly_code(assembly: &Assembly) {
    for instruction in assembly {
        println!("  {:?}", instruction);
    }
}

#[derive(Clone, Copy)]
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
        }
    }
}

#[derive(Clone, Copy)]
pub enum Operand {
    Register(Register),
    Immediate(i32),
    Memory(Register),
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

#[derive(Clone, Copy)]
pub enum Register {
    RAX,
    RBP,
    RDI,
    RSI,
    RDX,
    RCX,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    AL,
}

impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::RAX => write!(f, "rax"),
            Register::RBP => write!(f, "rbp"),
            Register::RDI => write!(f, "rdi"),
            Register::RSI => write!(f, "rsi"),
            Register::RDX => write!(f, "rdx"),
            Register::RCX => write!(f, "rcx"),
            Register::R8 => write!(f, "r8"),
            Register::R9 => write!(f, "r9"),
            Register::R10 => write!(f, "r10"),
            Register::R11 => write!(f, "r11"),
            Register::R12 => write!(f, "r12"),
            Register::R13 => write!(f, "r13"),
            Register::R14 => write!(f, "r14"),
            Register::R15 => write!(f, "r15"),
            Register::AL => write!(f, "al"),
        }
    }
}
