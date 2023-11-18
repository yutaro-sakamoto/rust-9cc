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

#[derive(Clone, Copy)]
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

pub fn rax() -> Operand {
    Operand::Register(Register::RAX)
}

pub fn m_rax() -> Operand {
    Operand::Memory(Register::RAX)
}

pub fn rbp() -> Operand {
    Operand::Register(Register::RBP)
}

pub fn m_rbp() -> Operand {
    Operand::Memory(Register::RBP)
}

pub fn rdi() -> Operand {
    Operand::Register(Register::RDI)
}

pub fn m_rdi() -> Operand {
    Operand::Memory(Register::RDI)
}

pub fn rsi() -> Operand {
    Operand::Register(Register::RSI)
}

pub fn m_rsi() -> Operand {
    Operand::Memory(Register::RSI)
}

pub fn rdx() -> Operand {
    Operand::Register(Register::RDX)
}

pub fn m_rdx() -> Operand {
    Operand::Memory(Register::RDX)
}

pub fn rcx() -> Operand {
    Operand::Register(Register::RCX)
}

pub fn m_rcx() -> Operand {
    Operand::Memory(Register::RCX)
}

pub fn r8() -> Operand {
    Operand::Register(Register::R8)
}

pub fn m_r8() -> Operand {
    Operand::Memory(Register::R8)
}

pub fn r9() -> Operand {
    Operand::Register(Register::R9)
}

pub fn m_r9() -> Operand {
    Operand::Memory(Register::R9)
}

pub fn r10() -> Operand {
    Operand::Register(Register::R10)
}

pub fn m_r10() -> Operand {
    Operand::Memory(Register::R10)
}

pub fn r11() -> Operand {
    Operand::Register(Register::R11)
}

pub fn m_r11() -> Operand {
    Operand::Memory(Register::R11)
}

pub fn r12() -> Operand {
    Operand::Register(Register::R12)
}

pub fn m_r12() -> Operand {
    Operand::Memory(Register::R12)
}

pub fn r13() -> Operand {
    Operand::Register(Register::R13)
}

pub fn m_r13() -> Operand {
    Operand::Memory(Register::R13)
}

pub fn r14() -> Operand {
    Operand::Register(Register::R14)
}

pub fn m_r14() -> Operand {
    Operand::Memory(Register::R14)
}

pub fn r15() -> Operand {
    Operand::Register(Register::R15)
}

pub fn m_r15() -> Operand {
    Operand::Memory(Register::R15)
}

pub fn al() -> Operand {
    Operand::Register(Register::AL)
}

pub fn m_al() -> Operand {
    Operand::Memory(Register::AL)
}
