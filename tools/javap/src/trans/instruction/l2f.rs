use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct L2F;

impl Instruction for L2F {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::l2f,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
