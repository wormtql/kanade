use std::fmt::Display;
use rand::{Rng, thread_rng};
use crate::common::condition::Condition;
use crate::env::env::Env;
use crate::instruction::instructions::vadd::VAddDispatch;
use strum_macros::{EnumCount};
use strum::{EnumCount as EnumCountT};
use crate::instruction::instruction::InstructionName::VADD;
use crate::instruction::instructions::vabs::VAbsDispatch;

pub trait InstructionDispatch {
    fn execute(instruction: &Instruction, env: &mut Env);

    fn get_string(instruction: &Instruction) -> String;

    fn random() -> Instruction;
}

#[derive(Clone, EnumCount, FromPrimitive, ToPrimitive, Eq, PartialEq)]
pub enum InstructionName {
    NOP,
    VADD,
    VABS,
}

#[derive(Clone)]
pub struct Instruction {
    pub name: InstructionName,
    pub dest: usize,
    pub src1: usize,
    pub src2: usize,
    pub cond: Condition,
    pub f32: bool,
}

impl Instruction {
    pub fn execute(&self, env: &mut Env) {
        match self.name {
            InstructionName::VADD => {
                VAddDispatch::execute(&self, env);
            },
            _ => {}
        }
    }

    pub fn to_string(&self) -> String {
        match self.name {
            InstructionName::VADD => {
                VAddDispatch::get_string(&self)
            },
            InstructionName::VABS => VAbsDispatch::get_string(&self),
            InstructionName::NOP => String::from("NOP"),
        }
    }

    pub fn nop() -> Self {
        Instruction {
            name: InstructionName::NOP,
            dest: 0,
            src1: 0,
            src2: 0,
            cond: Condition::EQ,
            f32: false
        }
    }

    pub fn random() -> Instruction {
        let index = thread_rng().gen::<usize>() % InstructionName::COUNT;
        let name: InstructionName = num::FromPrimitive::from_usize(index).unwrap();

        match name {
            InstructionName::VADD => VAddDispatch::random(),
            _ => Instruction::nop(),
        }
    }
}