use std::time::Instant;

use fractran::program::enumerate::Enumerator;

fn main() {

    for sz in 1..18 {
        let time = Instant::now();

        let mut enumerator = Enumerator::new(sz);
        enumerator.enumerate(1, 0);

        println!("BBF({sz}) >= {} ({:?}) | {:?}", enumerator.best_steps, enumerator.counts, time.elapsed());

        // let mut i = 0;
        // while let Some(_) = enumerator.expand_last() {
        //     i+=1;
        // }

        // println!("SZ {sz} | Programs {} | Programs Examined {i} | {:?}", enumerator.programs.len(), time.elapsed());
        // // enumerator.print_programs();
    }
}