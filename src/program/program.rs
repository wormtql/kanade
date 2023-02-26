use std::fmt::{Display, Formatter};
use smallvec::SmallVec;
use crate::env::env::Env;
use crate::instruction::instruction::Instruction;

pub struct Program {
    pub instructions: SmallVec<[Box<dyn Instruction>; 32]>
}

impl Clone for Program {
    fn clone(&self) -> Self {
        let instructions = self.instructions.iter().map(|item| item.instruction_clone()).collect();
        Program {
            instructions
        }
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            instructions: SmallVec::new(),
        }
    }

    pub fn add_instruction(&mut self, inst: Box<dyn Instruction>) {
        self.instructions.push(inst);
    }

    pub fn execute(&self, env: &mut Env) {
        for inst in self.instructions.iter() {
            inst.execute(env);
        }
    }
}

impl Display for Program {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for inst in self.instructions.iter() {
            write!(f, "{}\n", inst)?;
        }
        Ok(())
    }
}