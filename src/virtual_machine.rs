type Register = u64;
type Memory = Vec<u64>;

pub struct VirtualMachine {
    registers: Registers,
    memory: Memory,
}
