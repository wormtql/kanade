use std::fmt::{Display, Formatter};
use std::ptr::null_mut;
use rand::{Rng, thread_rng};
use smallvec::SmallVec;
use crate::env::env::Env;
use crate::instruction::instruction::{Instruction, InstructionName};
use strum::{EnumCount};

#[derive(Clone)]
pub struct Program {
    pub instructions: SmallVec<[Instruction; 32]>
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: SmallVec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub fn execute(&self, env: &mut Env) {
        for inst in self.instructions.iter() {
            inst.execute(env);
        }
    }

    pub fn mutate_opcode(&mut self) {
        let mut candidates: SmallVec<[*mut Instruction; 32]> = SmallVec::new();
        for inst in self.instructions.iter_mut() {
            if inst.name != InstructionName::NOP {
                candidates.push(inst as *mut Instruction);
            }
        }

        let instruction_index = thread_rng().gen::<usize>() % candidates.len();
        let inst = unsafe { &mut *candidates[instruction_index] };

        let inst_len = InstructionName::COUNT;
        let index = thread_rng().gen::<usize>() % (inst_len - 1) + 1;
        let new_name: InstructionName = num::FromPrimitive::from_usize(index).unwrap();
        inst.name = new_name;
    }

    pub fn mutate_operand(&mut self) {
        let mut candidates: SmallVec<[*mut Instruction; 32]> = SmallVec::new();
        for inst in self.instructions.iter_mut() {
            if inst.name != InstructionName::NOP {
                candidates.push(inst as *mut Instruction);
            }
        }

        let instruction_index = thread_rng().gen::<usize>() % candidates.len();
        let inst = unsafe { &mut *candidates[instruction_index] };

        // todo
    }

    pub fn mutate_swap(&mut self) {
        if self.instructions.len() <= 1 {
            return;
        }

        let index1 = thread_rng().gen::<usize>() % self.instructions.len();
        let left_count = index1;
        let right_count = self.instructions.len() - left_count - 1;

        let flag = thread_rng().gen::<usize>() % (self.instructions.len() - 1);
        let index2;
        if flag < left_count {
            index2 = thread_rng().gen::<usize>() % left_count;
        } else {
            index2 = thread_rng().gen::<usize>() % right_count + index1 + 1;
        }

        self.instructions.swap(index1, index2);
    }

    pub fn mutate_instruction(&mut self) {
        let index = thread_rng().gen::<usize>() % self.instructions.len();
        let random = Instruction::random();
        self.instructions[index] = random;
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for inst in self.instructions.iter() {
            write!(f, "{}\n", inst.to_string())?;
        }
        Ok(())
    }
}