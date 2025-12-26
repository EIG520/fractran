use std::env;
use std::time::Instant;

use fractran::program::program::FractranProgram;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut program = FractranProgram::from(args[1].clone());
    let intvl: u64 = args[2].parse().expect("invalid interval");
    let time = Instant::now();
    let mut i: u64 = 0;
    while program.step() {
        i += 1;

        if i % intvl == 0 {
            println!("Step: {i} | State: {:?} | Time: {:?}", program.state, time.elapsed());
        }
    }

    println!("{i}");

    // let mut prgm = FractranProgram::new(vec![1,0,0],
    // Vec2d::new(vec![
    //     1,-1,1,
    //     -1,0,1,
    //     1,1,-1
    // ], 3));

    // for _ in 0..10 {
    //     println!("{:?}", prgm.state);
    //     prgm.step();
    // }
}
