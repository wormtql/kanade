use std::fmt::{Display, Formatter};
use crate::common::condition::Condition;
use crate::env::env::Env;
use crate::instruction::instruction::Instruction;

#[derive(Clone)]
pub struct VAdd {
    pub cond: Condition,
    pub dest: usize,
    pub src1: usize,
    pub src2: usize,
    pub f32: bool,
}

impl VAdd {
    pub fn new_32(dest: usize, src1: usize, src2: usize) -> Self {
        Self {
            cond: Condition::AL,
            dest,
            src1,
            src2,
            f32: true
        }
    }

    pub fn new_64(dest: usize, src1: usize, src2: usize) -> Self {
        Self {
            cond: Condition::AL,
            dest,
            src1,
            src2,
            f32: false,
        }
    }
}

impl Instruction for VAdd {
    fn execute(&self, env: &mut Env) {
        if self.f32 {
            let src1 = env.get_single(self.src1);
            let src2 = env.get_single(self.src2);
            env.set_single(self.dest, src1 + src2);
        } else {
            let src1 = env.get_double(self.src1);
            let src2 = env.get_double(self.src2);
            env.set_double(self.dest, src1 + src2);
        }
    }
}

impl Display for VAdd {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "VADD")?;
        if self.f32 {
            write!(f, ".F32 S{}, S{}, S{}", self.dest, self.src1, self.src2)?;
        } else {
            write!(f, ".F64 D{}, D{}, D{}", self.dest, self.src1, self.src2)?;
        }

        Ok(())
    }
}
