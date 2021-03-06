#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct If_Icmple;

impl Instruction for If_Icmple {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::if_icmple,
            icp: 0,
            wide: false,
        };

        (info, pc + 3)
    }
}
