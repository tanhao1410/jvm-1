use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Ishl;

impl Instruction for Ishl {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::ishl,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
