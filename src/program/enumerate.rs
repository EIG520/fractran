// use crate::program::{arr2d::Vec2d};

use crate::{deciders::Decision, program::{arr2d::SVec2d, program::FractranProgram}};

pub struct Enumerator {
    pub szmax: usize,
    pub program: FractranProgram,
    pub count: u64,
    pub best_steps: u64,
    pub counts: Vec<u64>
}

impl Enumerator {
    pub fn enumerate(&mut self, sz: usize, depth: usize) {
            // println!("{depth}:{}", self.program.to_string());
        // let marked = self.program.rules.elements().len() >= 8 && self.program.rules.width == 4 && self.program.rules.elements()[0..8] == [-1,3,0,0,0,-1,2,0];

        // if marked {
        //     println!("MARK ({sz}): {}", self.program.to_string());
        // }

        if self.program.check_ordered() == Decision::Extraneous { return; }
        if sz > self.szmax { return; }

        if sz == self.szmax {
            self.count += 1;


            match self.big_check(sz) {
                Decision::Halt(st) | Decision::EHalt(st) => {
                    if st as u64 > self.best_steps {
                        self.best_steps = st as u64;
                        // println!("NEW {sz} CHAMP ({st}): {}", self.program.to_string());
                    }
                },
                Decision::Unsure => {
                    println!("HOLDOUT: {}", self.program.to_string());
                }
                _ => {}
            }

            return;
        }

        let mut lelx = self.program.rules.lwidth;

        if lelx == 0 {
            lelx = self.program.rules.width;
        }

        lelx -= 1;
        let lely = self.program.rules.height - 1;
        let lelv = *self.program.rules.get(lelx, lely);

        if lelv <= 0 {
            self.program.rules.set(lelv - 1, lelx, lely);
            self.enumerate(sz + 1, depth + 1);
            self.program.rules.set(lelv, lelx, lely);
        }
        if lelv >= 0 {
            self.program.rules.set(lelv + 1, lelx, lely);
            self.enumerate(sz + 1, depth + 1);
            self.program.rules.set(lelv, lelx, lely);
        }


        if lelx < self.program.rules.width - 1 {
            self.program.rules.push_last();
            self.enumerate(sz, depth + 1);
            self.program.rules.pop_last();
        } else if lelv != 0 || lelx == 0 {

            self.program.rules.incwidth();

            self.enumerate(sz, depth + 1);

            self.program.rules.decwidth();
        } else {
            self.count += 1;

            // if marked {
            //     println!("{}: {:?}", self.program.to_string(), self.big_check(self.szmax));
            // }

            match self.big_check(sz) {
                Decision::Halt(_) | Decision::Unsure => {
                    self.program.rules.new_row();
                    self.enumerate(sz+1, depth + 1);
                    self.program.rules.rem_row();
                },
                Decision::Forever | Decision::Extraneous | Decision::EHalt(_) => { }
            }
        }
    }

    pub fn new(szmax: usize) -> Self {
        Self {
            szmax,
            program: FractranProgram::new(vec![1], SVec2d::default()),
            best_steps: 0,
            count: 0,
            counts: vec![0;10]
        }
    }
}

// pub struct Enumerator {
//     pub programs: Vec<RuleHolder>,
//     active: Vec<RuleHolder>,
//     szmax: usize,
//     best_steps: u64
// }

// impl Enumerator {
//     pub fn expand_last(&mut self) -> Option<()> {
//         let mut program = self.active.pop()?;
//         let tomod = program.tomod().clone();


//         // println!("PROGRAM: {} | {:?} | {:?}", program.to_vec2d().width, program.to_vec2d().elements, program.open);
//         if program.new && program.absum() == self.szmax {
//             program.dec = Some(program.to_fractran().big_check(self.szmax));

//             if let Some(Decision::Halt(st)) = program.dec {
//                 if st as u64 > self.best_steps || self.best_steps > 5 && st as u64 == self.best_steps {
//                     println!("NEW CHAMP ({st}): {:?}|{:?} [AT: {}]", program.elements_bottom.width, program.to_vec2d().elements, self.programs.len());
//                     self.best_steps = st as u64;
//                 }
//             } else if let Some(Decision::Unsure) = program.dec {
//                 // Holdout
//                 println!("HOLDOUT: {:?}|{:?} [AT: {}]", program.elements_bottom.width, program.to_vec2d().elements, self.programs.len());
//                 self.programs.push(program.clone());
//             } else {
//                 // Forever
//                 // for p in self.programs.clone() {
//                 //     if p.to_vec2d() == program.to_vec2d() {
//                 //         println!("DUPLICATE: {} | {:?} ", program.to_vec2d().width, program.to_vec2d().elements)
//                 //     }
//                 // }
//             }

//             return Some(());
//         }

//         if program.absum() >= self.szmax { return Some(()); }


//         let row = tomod.last_row();
//         let last_row = tomod.height() - 1;
//         let last_idx = row.len() - 1;

//         let mut nonzero = false;
//         for &j in row {
//             if j != 0 { nonzero = true; }
//         }

//         let mut nfir = row[0] == -1;
//         for &j in row.iter().skip(1) {
//             if j < 0 { nfir = false }
//         }

//         // Option 1: increase last number
//         let curr = tomod.get(last_idx, last_row);
//         let dir = curr.signum();
//         let first = program.elements_top.height() == 1 && program.open == RHStatus::Top;
//         // let mut cont = true;

//         if last_idx >= 2 {
//             // cont = !first || curr.abs() < *tomod.get(last_idx - 1, last_row);

//             if (first && *curr < *tomod.get(last_idx - 1, last_row)) {
//                 return Some(());
//             }
//         }
        

//         if dir == 0 {
//             let mut p1 = program.clone();
//             p1.new = true;
//             p1.dec = None;
//             let pt1 = p1.tomod();
//             pt1.set(1, last_idx, last_row);
//             self.active.push(p1);

//             if !first {
//                 let mut p11 = program.clone();
//                 p11.new = true;
//                 p11.dec = None;
//                 let pt11 = p11.tomod();
//                 pt11.set(-1, last_idx, last_row);
//                 self.active.push(p11);
//             }
//         } else if !(first && last_idx == 0) {
//             let mut p1 = program.clone();
//             p1.new = true;
//             p1.dec = None;
//             let pt1 = p1.tomod();
//             pt1.set(curr + dir, last_idx, last_row);
//             self.active.push(p1);
//         }
        

//         // Option 2: move right
//         if last_idx < tomod.width - 1 {
//             let mut p2 = program.clone();
//             p2.new = false;
//             p2.dec = None;
//             let pt2 = p2.tomod();
//             pt2.elements.push(0);
//             self.active.push(p2);
//         } else {
//             let mut p3 = program.clone();
//             p3.new = true;
//             p3.dec = None;

//             match program.open {
//                 RHStatus::Top => {p3.elements_bottom.width += 1;}
//                 RHStatus::Bottom => {p3.elements_bottom.incwidth(0);}
//             }
//             p3.elements_top.incwidth(0);

//             let pt3 = p3.tomod();
//             pt3.set(1, last_idx + 1, last_row);
//             self.active.push(p3);

//             if !first {
//                 let mut p4 = program.clone();
//                 p4.new = true;
//                 p4.dec = None;
//                 match program.open {
//                     RHStatus::Top => {p4.elements_bottom.width += 1;}
//                     RHStatus::Bottom => {p4.elements_bottom.incwidth(0);}
//                 }
//                 p4.elements_top.incwidth(0);
//                 let pt4 = p4.tomod();
//                 pt4.set(-1, last_idx + 1, last_row);
//                 self.active.push(p4);
//             }

//             // Option 3: new row
//             if nonzero {
//                 let bc = if let Some(t) = program.dec {
//                     t
//                 } else {
//                     let bce = program.to_fractran().big_check(self.szmax);
//                     program.dec = Some(bce);
//                     bce
//                 };

//                 if ((program.open == RHStatus::Top || bc != Decision::Forever) && bc != Decision::Extraneous) && !(!first && nfir) {
//                     let mut p5 = program.clone();
//                     p5.new = false;
//                     let pt5 = p5.tomod();
//                     pt5.elements.push(0);
//                     self.active.push(p5);

//                     // if program.open == RHStatus::Top {
//                     //     println!("EVIL {:?}|{:?}", p5.elements_bottom.width, p5.to_vec2d().elements);
//                     // }
//                 }
//             }
//         }

//         // Option 4: change sides
//         if program.open == RHStatus::Top && !(!first && nfir){
//             let mut p5 = program.clone();
//             p5.new = false;
//             p5.open = RHStatus::Bottom;

//             p5.elements_top.elements.extend(vec![0;p5.elements_bottom.width - last_idx - 1]);
//             self.active.push(p5);
//         }
        
//         Some(())
//     }

//     pub fn print_active(&self) {
//         println!("ACTIVE:");
//         for rh in &self.active {
//             print!(">>> ");
//             let v2d = rh.to_vec2d();

//             println!("{} : {:?}", v2d.width, v2d.elements);
//         }
//         println!();
//     }

//     pub fn print_programs(&self) {
//         println!("FOUND:");
//         for rh in &self.programs {
//             print!(">>> ");
//             let v2d = rh.to_vec2d();

//             println!("{} : {:?}", v2d.width, v2d.elements);
//         }
//         println!();
//     }

//     pub fn new(szmax: usize) -> Enumerator {
//         Enumerator {
//             programs: vec![],
//             active: vec![RuleHolder::default()],
//             szmax,
//             best_steps: 0
//         }
//     }
// }

// pub struct Enumerator {
//     programs: Vec<Vec2d<i8>>,
//     active: Vec<Vec2d<i8>>,
//     szmax: usize
// }

// impl Enumerator {
//     pub fn expand_last(&mut self) -> Option<()> {
//         let program = self.active.pop()?;

//         if program.elements.iter().map(|i| i.abs() as usize).sum::<usize>() >= self.szmax {
//             self.programs.push(program);
//             return Some(());
//         }

//         let last_row = program.height() -1;
//         let row = program.get_row(last_row);
//         let mut last_idx = 0;
//         for (idx, &n) in row.iter().enumerate() {
//             if n != 0 {
//                 last_idx = idx;
//             }
//         }

//         let frow = program.get_row(0);
//         let mut flast_idx = 0;
//         for (idx, &n) in frow.iter().enumerate() {
//             if n != 0 {
//                 flast_idx = idx;
//             }
//         }

//         // Option 1: increase the last number
//         let curr = program.get(last_idx, last_row);
//         let dir = curr.signum();

//         let mut prog_new_1 = program.clone();
//         prog_new_1.set(curr + dir, last_idx, last_row);
//         self.active.push(prog_new_1);

//         let curr2 = program.get(flast_idx, 0);
//         let dir2 = curr2.signum();
//         let mut prog_new_11 = program.clone();
//         prog_new_11.set(curr2 + dir2, flast_idx, 0);
//         self.active.push(prog_new_11);

//         // Option 2: add new row to end
//         let mut prog_new_2 = program.clone();
//         prog_new_2.add_row(vec![0; prog_new_2.width]);
//         self.active.push(prog_new_2);
        
//         // Option 3: add new row to start
//         let mut nvec = vec![0; program.width];
//         nvec.extend(program.elements.clone());
//         let prog_new_3 = Vec2d::new(nvec, program.width);
//         self.active.push(prog_new_3);

//         // Option 4: increase width to the right
//         if last_idx < program.width - 1 {
//             let mut prog_new_4 = program.clone();

//             prog_new_4.set(1, last_idx + 1, last_row);
//             self.active.push(prog_new_4);

//             let mut prog_new_42 = program.clone();
//             prog_new_42.set(-1, last_idx + 1, last_row);
//             self.active.push(prog_new_42);
//         } else {
//             let mut nvec2 = vec![];

//             for i in 0..program.height() {
//                 let row = program.get_row(i);
//                 nvec2.extend(row.to_vec());
//                 nvec2.push(0);
//             }

//             let prog_new_5 = Vec2d::new(nvec2, program.width + 1);
//             self.active.push(prog_new_5);
//         }

//         self.programs.push(program);

//         Some(())
//     }

//     pub fn print_active(&self) {
//         println!("ACTIVE:");
//         for v2d in &self.active {
//             println!(">>> {} | {:?}", v2d.width, v2d.elements);
//         }
//         println!();
//     }

//     pub fn print_programs(&self) {
//         println!("COMPLETE:");
//         for v2d in &self.active {
//             println!(">>> {} | {:?}", v2d.width, v2d.elements);
//         }
//         println!();
//     }

//     pub fn new(szmax: usize) -> Self {
//         let mut new = Self::default();
//         new.szmax = szmax;

//         new
//     }
// }

// impl Default for Enumerator {
//     fn default() -> Self {
//         Self {
//             programs: vec![],
//             active: vec![Vec2d::new(vec![-1], 1)],
//             szmax: 999999
//         }
//     }
// }