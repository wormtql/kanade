use kanade::env::env::Env;
use kanade::instruction::instructions::vadd::VAdd;
use kanade::program::program::Program;

fn main() {
    let mut program = Program::new();
    program.add_instruction(Box::new(VAdd::new_64(0, 1, 2)));

    let mut env = Env::new();
    env.set_double(1, 1.0);
    env.set_double(2, 3.0);
    program.execute(&mut env);

    println!("{}", env.get_double(0));
    println!("{}", program);
}
