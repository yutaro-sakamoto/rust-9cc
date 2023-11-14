pub type Assembly = Vec<Instruction>;

pub enum Instruction {
    Push(Operand),
    Pop(Operand),
    Ret,
    Add(Operand, Operand),
    Sub(Operand, Operand),
    Mul(Operand, Operand),
    Idiv(Operand),
    Neg(Operand),
    Cpo,
    Movzb(Operand, Operand),
    Mov(Operand, Operand),
    Cmp(Operand, Operand),
    Sete(Operand, Operand),
    Setne(Operand, Operand),
    Setl(Operand, Operand),
    Setle(Operand, Operand),
}
pub enum Operand {
    Register(Register),
    Immediate(i32),
    Memory(Register),
}

pub enum Register {
    RAX,
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
}
