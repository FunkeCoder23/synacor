#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Opcode {
    ///   halt: 0
    ///
    ///   stop execution and terminate the program
    HALT,

    /// set: 1 a b
    ///
    ///   set register \<a> to the value of \<b>
    SET,

    /// push: 2 a
    ///
    ///   push value \<a> onto the stack
    PUSH,

    /// pop: 3 a
    ///
    ///   remove the top element from the stack and write it into register \<a>; empty stack = error
    POP,

    /// eq: 4 a b c
    ///
    ///   set register \<a> to 1 if \<b> is equal to \<c>; set it to 0 otherwise
    EQ,

    /// gt: 5 a b c
    ///
    ///   set register \<a> to 1 if \<b> is greater than \<c>; set it to 0 otherwise
    GT,

    /// jmp: 6 a
    ///
    ///   jump to \<a>
    JMP,

    /// jnz: 7 a b
    ///
    ///   if value \<a> is nonzero, jump to \<b>
    JNZ,

    /// jz: 8 a b
    ///
    ///   if value \<a> is zero, jump to \<b>
    JZ,

    /// add: 9 a b c
    ///
    ///   assign into register \<a> the sum of \<b> and \<c> (modulo 32768)
    ADD,

    /// mult: 10 a b c
    ///
    ///   store into register \<a> the product of \<b> and \<c> (modulo 32768)
    MULT,

    /// mod: 11 a b c
    ///
    ///   store into register \<a> the remainder of \<b> divided by \<c>
    MOD,

    /// and: 12 a b c
    ///
    ///   stores into register \<a> the bitwise and of \<b> and \<c>
    AND,

    /// or: 13 a b c
    ///
    ///   stores into register \<a> the bitwise or of \<b> and \<c>
    OR,

    /// not: 14 a b
    ///
    ///   stores 15-bit bitwise inverse of \<b> in register \<a>
    NOT,

    /// rmem: 15 a b
    ///
    ///   read memory at address \<b> and write it to register \<a>
    RMEM,

    /// wmem: 16 a b
    ///
    ///   write the value from \<b> into memory at address \<a>
    WMEM,

    /// call: 17 a
    ///
    ///   write the address of the next instruction to the stack and jump to value \<a>
    CALL,

    /// ret: 18
    ///
    ///   remove the top element from the stack and jump to it; empty stack = halt
    RET,

    /// out: 19 a
    ///
    /// write the character represented by ascii code \<a> to the terminal
    OUT,

    /// in: 20 a
    ///
    ///   read a character from the terminal and write its ascii code to \<a>; it can be assumed that once input starts, it will continue until a newline is encountered;
    ///   this means that you can safely read whole lines from the keyboard and trust that they will be fully read
    IN,

    /// noop: 21
    ///
    ///   no operation
    NOOP,

    /// UNK(u16): ~
    ///
    /// unknown opcode
    UNK(u16),
}

#[allow(dead_code)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u16> for Opcode {
    fn from(v: u16) -> Self {
        match v {
            0 => return Opcode::HALT,
            1 => return Opcode::SET,
            2 => return Opcode::PUSH,
            3 => return Opcode::POP,
            4 => return Opcode::EQ,
            5 => return Opcode::GT,
            6 => return Opcode::JMP,
            7 => return Opcode::JNZ,
            8 => return Opcode::JZ,
            9 => return Opcode::ADD,
            10 => return Opcode::MULT,
            11 => return Opcode::MOD,
            12 => return Opcode::AND,
            13 => return Opcode::OR,
            14 => return Opcode::NOT,
            15 => return Opcode::RMEM,
            16 => return Opcode::WMEM,
            17 => return Opcode::CALL,
            18 => return Opcode::RET,
            19 => return Opcode::OUT,
            20 => return Opcode::IN,
            21 => return Opcode::NOOP,
            val => return Opcode::UNK(val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HALT;
        assert_eq!(opcode, Opcode::HALT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HALT);
        assert_eq!(instruction.opcode, Opcode::HALT);
    }
}
