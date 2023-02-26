use std::fmt::Display;
use crate::env::env::Env;

pub trait InstructionClone {
    fn instruction_clone(&self) -> Box<dyn Instruction>;
}

pub trait Instruction: InstructionClone + Display {
    fn execute(&self, env: &mut Env);
}

impl<T: Clone + Instruction + 'static> InstructionClone for T {
    fn instruction_clone(&self) -> Box<dyn Instruction> {
        Box::new(self.clone())
    }
}
