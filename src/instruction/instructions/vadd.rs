use std::fmt::{Display, Formatter};
use rand::{Rng, thread_rng};
use crate::common::condition::Condition;
use crate::env::env::Env;
use crate::instruction::instruction::{Instruction, InstructionDispatch, InstructionName};

#[derive(Clone)]
pub struct VAddDispatch {}

impl VAddDispatch {
    pub fn new_32(dest: usize, src1: usize, src2: usize) -> Instruction {
        Instruction {
            name: InstructionName::VADD,
            cond: Condition::AL,
            dest,
            src1,
            src2,
            f32: true
        }
    }

    pub fn new_64(dest: usize, src1: usize, src2: usize) -> Instruction {
        Instruction {
            name: InstructionName::VADD,
            cond: Condition::AL,
            dest,
            src1,
            src2,
            f32: false,
        }
    }
}

impl InstructionDispatch for VAddDispatch {
    fn execute(instruction: &Instruction, env: &mut Env) {
        if instruction.f32 {
            let src1 = env.get_single(instruction.src1);
            let src2 = env.get_single(instruction.src2);
            env.set_single(instruction.dest, src1 + src2);
        } else {
            let src1 = env.get_double(instruction.src1);
            let src2 = env.get_double(instruction.src2);
            env.set_double(instruction.dest, src1 + src2);
        }
    }

    fn get_string(instruction: &Instruction) -> String {
        let mut result = String::from("VADD");
        if instruction.f32 {
            result += &format!(".F32 S{}, S{}, S{}", instruction.dest, instruction.src1, instruction.src2);
        } else {
            result += &format!(".F64 D{}, D{}, D{}", instruction.dest, instruction.src1, instruction.src2);
        }
        result
    }

    fn random() -> Instruction {
        Instruction {
            name: InstructionName::VADD,
            dest: thread_rng().gen::<usize>() % 16,
            src1: thread_rng().gen::<usize>() % 16,
            src2: thread_rng().gen::<usize>() % 16,
            cond: Condition::AL,
            f32: false
        }
    }
}
