use std::collections::HashSet;

use z3::{SatResult, Solver, ast::Int};

use crate::{deciders::Decision, program::{arr2d::Vec2d, program::FractranProgram}};

impl FractranProgram {
    pub fn check_div1(&self) -> Decision {
        let rule = self.rules.last_row();
        let mut has_neg = false;

        for &n in rule {
            if n<0 { has_neg = true; break }
        }

        if !has_neg {
            return Decision::Forever;
        }

        Decision::Unsure
    }
}

impl FractranProgram {
    pub fn check_covered(&self) -> Decision {
        let rlast = self.rules.last_row();
        for i in 0..self.rules.height-1 {
            let mut cov = true;
            let rule = self.rules.get_row(i);

            for (&rc,&rl) in rule.iter().zip(rlast) {
                if rl >= 0 && rc < 0 { cov = false; break; }
                if rl < 0 && rc < rl { cov = false; break; }
            }
            if cov { return Decision::Extraneous }
        }
        Decision::Unsure
    }
}

impl FractranProgram {
    pub fn check_ordered(&self) -> Decision {
        for x in 2..self.rules.width {
            let mut c = true;

            for y in 0..self.rules.height-1 {
                if enum_val(*self.rules.get(x, y)) > enum_val(*self.rules.get(x-1, y)) { return Decision::Extraneous; }
                if enum_val(*self.rules.get(x, y)) < enum_val(*self.rules.get(x-1, y)) { c=false; break; }
            }

            let y = self.rules.height - 1;
            if self.rules.lwidth > x && c {
                if enum_val(*self.rules.get(x, y)) > enum_val(*self.rules.get(x-1, y)) { return Decision::Extraneous; }
            }
        }

        return Decision::Unsure;
    }
}

fn enum_val(n: i8) -> i8 {
    if n < 0 { -2*n+1 }
    else { 2*n }
}

impl FractranProgram {
    pub fn check_completable(&self, szmax: usize, sz: usize) -> Decision {
        let mut need_pos = 0;
        let mut need_neg = 0;
        for i in 1..self.rules.width {
            let mut has_pos = false;
            let mut has_neg = false;
            for j in 0..self.rules.height {
                let val = *self.rules.get(i,j);

                has_pos |= val > 0;
                has_neg |= val < 0;
            }

            if has_pos && !has_neg { need_neg += 1; }
            if has_neg && !has_pos { need_pos += 1; }
        }

        let mut has2 = false;
        for i in 0..self.rules.height {
            if *self.rules.get(0,i) != -1 {continue;}

            let mut good = true;
            for j in 1..self.rules.width {
                if *self.rules.get(j,i)<0 {good = false; break; }
            }
            if good { has2 = true; break; }
        }

        let mut total = need_neg + need_pos;

        if has2 {
            if need_neg > 0 { total += 1; }
            else if need_pos > 0 { total += 2; }
        } else if need_neg > 0 {
            total += 3;
        } else {
            total += 2;
        }

        if total + sz > szmax { return Decision::Extraneous }
        return Decision::Unsure;
    }
}

impl FractranProgram {
    pub fn check_translate_cycle(&mut self, limit: u32) -> Decision {
        self.state = vec![1];
        self.state.extend(vec![0; self.rules.width - 1]);

        let mut dog = self.state.clone();
        let mut mouse = self.state.clone();
        let mut cat = self.state.clone();
        let mut key_cols = vec![0;self.rules.width];

        for i in 0..limit {
            self.state = mouse;
            if !self.step_info(&mut key_cols, false) {
                for &el in &self.state {
                    if el != 0 { return Decision::Halt(i); }
                }
                return Decision::EHalt(i)
            }
            mouse = self.state.clone();

            if i % 3 < 2 {
                self.state = cat;
                self.step_info(&mut key_cols, true);
                cat = self.state.clone();
            } else {
                self.state = dog;
                self.step();
                dog = self.state.clone();

                let mut valid = true;

                for j in 0..self.rules.width {
                    if let Some(a) = mouse[j].checked_sub(cat[j]) {
                        if let Some(b) = cat[j].checked_sub(dog[j]) {
                            if a != b {
                                valid = false;
                                break;
                            }

                            if key_cols[j] > 0 && a != 0 {
                                valid = false;
                                break;
                            }
                        } else { valid = false; break; }
                    } else { valid = false; break; }
                }

                if valid {
                    return Decision::Forever;
                }
            }
        }

        return Decision::Unsure;
    }
}

impl FractranProgram {
    pub fn check_linear_invariant(&self) -> Decision {
        let mut v = vec![];

        for i in 0..self.rules.width {
            v.push(Int::fresh_const(&format!("r{i}")));
        }

        let mut lones = HashSet::new();
        let solver = Solver::new();

        for ri in 0..self.rules.height {
            let row = self.rules.get_row(ri);
            let mut exp = z3::ast::Int::from_i64(0);

            let mut conds = 0;
            let mut lone = 0;

            for (i,&n) in row.iter().enumerate() {
                exp += &v[i] * n;

                if n < 0 {
                    conds -= n;
                    lone = i;
                }
            }

            if conds == 1 {
                lones.insert(lone);
            }

            solver.assert(exp.ge(0));
        }

        for i in 0..self.rules.width {
            if !lones.contains(&i) {
                solver.assert(v[i].le(0));
            }
        }

        solver.assert(v[0].gt(0));

        if solver.check() == SatResult::Sat {
            return Decision::Forever;
        }

        return Decision::Unsure;
    }
}

impl FractranProgram {
    pub fn check_strong_lin_comb(&self) -> Decision {
        // Simulate to find the min for each col
        let mins = self.clone().cnt_sim(1000);
        
        // Create consts for each col, asserting mins
        let solver = Solver::new();
        let mut n = vec![];
        for y in 0..self.rules.height {
            n.push(Int::fresh_const(&format!("n{y}")));
            solver.assert(n[y].ge(mins[y]));
        }

        // Find prime exps given application counts for each rule
        let mut rs = vec![];
        for x in 0..self.rules.width {
            let mut rw = z3::ast::Int::from_i64( if x == 0 {1} else {0} );

            for y in 0..self.rules.height {
                let v = *self.rules.get(x,y);
                rw += v * n[y].clone()
            }
            solver.assert(rw.ge(0));
            rs.push(rw);
        }

        // assert the existence of a halting configuration
        for y in 0..self.rules.height {
            let mut asr = vec![];

            for x in 0..self.rules.width {
                let v = *self.rules.get(x,y);
                if v < 0 {
                    asr.push(rs[x].lt(-v));
                }
            }
            solver.assert(z3::ast::Bool::or(&asr));
        }

        // return decision
        if solver.check() == SatResult::Unsat {
            return Decision::Forever;
        } else {
            println!("{:?}", solver.get_assertions());
            println!("{:?}", solver.get_model());
        }
        return Decision::Unsure;
    }
}

impl FractranProgram {
    pub fn check_graph(&mut self, exp_lim: u64) -> Decision {
        let mut queue = Vec2d::new(vec![1], self.rules.width);
        let mut seen = HashSet::new();
        queue.elements.extend(vec![0; self.rules.width - 1]);

        while queue.height() > 0 {
            let state = queue.pop_row();
            self.state = state;

            if let Some(rulei) = self.find_rule() {
                // rare case where I like vec of vecs over vec2d
                let mut news = vec![vec![]];
                let rule = self.rules.get_row(rulei);

                for i in 0..self.rules.width {
                    let mut nval = self.state[i].wrapping_add_signed(rule[i] as i64);

                    while nval >= 2*exp_lim {
                        nval -= exp_lim;
                    }

                    for nw in &mut news {
                        nw.push(nval);
                    }

                    if self.state[i] >= exp_lim && nval < exp_lim {
                        for nwi in 0..news.len() {
                            let mut nw = news[nwi].clone();
                            let indx = nw.len()-1;
                            nw[indx] += exp_lim;
                            news.push(nw);
                        }
                    }
                }

                for new in news {
                    if seen.contains(&new) { continue; }
                    seen.insert(new.clone());

                    queue.add_row(new);
                }
            } else {
                return Decision::Unsure;
            }
        }

        return Decision::Forever;
    }
}