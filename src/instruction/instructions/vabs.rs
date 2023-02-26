use rand::{Rng, thread_rng};
use crate::common::condition::Condition;
use crate::env::env::Env;
use crate::instruction::instruction::{Instruction, InstructionDispatch, InstructionName};

pub struct VAbsDispatch;

impl InstructionDispatch for VAbsDispatch {
    fn execute(instruction: &Instruction, env: &mut Env) {
        if instruction.f32 {
            let value = env.get_single(instruction.src2);
            env.set_single(instruction.dest, value);
        } else {
            let value = env.get_double(instruction.src2);
            env.set_double(instruction.dest, value);
        }
    }

    fn get_string(instruction: &Instruction) -> String {
        let mut result = String::from("VABS");
        if instruction.f32 {
            result += &format!(".F32 S{}, S{}", instruction.dest, instruction.src2);
        } else {
            result += &format!(".F64 D{}, D{}", instruction.dest, instruction.src2);
        }
        result
    }

    fn random() -> Instruction {
        Instruction {
            name: InstructionName::VABS,
            dest: thread_rng().gen::<usize>() % 16,
            src1: 0,
            src2: thread_rng().gen::<usize>() % 16,
            cond: Condition::AL,
            f32: false
        }
    }
}
