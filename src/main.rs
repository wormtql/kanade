use kanade::env::env::Env;
use kanade::instruction::instructions::vabs::VAbsDispatch;
use kanade::instruction::instructions::vadd::VAddDispatch;
use kanade::instruction::instruction::InstructionDispatch;
use kanade::program::program::Program;

fn main() {
    let mut program = Program::new();
    program.add_instruction(VAddDispatch::new_64(0, 1, 2));
    program.add_instruction(VAbsDispatch::random());

    let mut env = Env::new();
    env.set_double(1, 1.0);
    env.set_double(2, 3.0);
    program.execute(&mut env);

    println!("{}", env.get_double(0));
    println!("{}", program);

    let mut p2 = program.clone();
    p2.mutate_opcode();
    println!("{}", p2);
}
