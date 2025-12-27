use crate::program::enumerate::Enumerator;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Decision {
    Forever,
    Unsure,
    Halt(u32),
    EHalt(u32),
    Extraneous
}

impl Enumerator {
    pub fn big_check(&mut self, sz: usize) -> Decision {
        self.counts[0] += 1;
        let chk_div1 = self.program.check_div1();
        if chk_div1 != Decision::Unsure {
            return chk_div1;
        }
        self.counts[1] += 1;
        let chk_completable = self.program.check_completable(self.szmax, sz);
        if chk_completable != Decision::Unsure {
            return chk_completable;
        }
        self.counts[2] += 1;
        let chk_ordered = self.program.check_ordered();
        if chk_ordered != Decision::Unsure {
            return chk_ordered;
        }
        self.counts[3] += 1;
        let chk_covered = self.program.check_covered();
        if chk_covered != Decision::Unsure {
            return chk_covered;
        }
        self.counts[4] += 1;
        let chk_sim = self.program.check_translate_cycle(10000);
        if chk_sim != Decision::Unsure {
            return chk_sim;
        }
        self.counts[5] += 1;
        for i in 1..5 {
            let chk_graph = self.program.check_graph(i);
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