use crate::program::program::FractranProgram;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Decision {
    Forever,
    Unsure,
    Halt(u32),
    Extraneous
}

impl FractranProgram {
    pub fn big_check(&mut self, szmax: usize) -> Decision {   
        let chk_div1 = self.check_div1();
        if chk_div1 != Decision::Unsure {
            return chk_div1;
        }

        let chk_completable = self.check_completable(szmax);
        if chk_completable != Decision::Unsure {
            return chk_completable;
        }

        let chk_ordered = self.check_ordered();
        if chk_ordered != Decision::Unsure {
            return chk_ordered;
        }

        let chk_covered = self.check_covered();
        if chk_covered != Decision::Unsure {
            return chk_covered;
        }

        let chk_sim = self.clone().check_translate_cycle(10000);
        if chk_sim != Decision::Unsure {
            return chk_sim;
        }

        for i in 1..5 {
            let chk_graph = self.clone().check_graph(i);
            if chk_graph != Decision::Unsure {
                return chk_graph;
            }
        }


        // let chk_linvar = self.check_linear_invariant();
        // if chk_linvar != Decision::Unsure {
        //     return chk_linvar;
        // }


        Decision::Unsure
    }
}

pub mod agnostic;