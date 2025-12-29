use crate::program::arr2d::SVec2d;
const PRIMES: [i32; 19] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67];

#[derive(Clone)]
pub struct FractranProgram {
    pub state: Vec<u64>,
    pub rules: SVec2d,
}

impl FractranProgram {
    pub fn find_rule(&self) -> Option<usize> {
        for i in 0..self.rules.height {
            let rule = self.rules.get_row(i);
            let mut valid = true;

            for (j,&c) in rule.into_iter().enumerate() {
                if c < 0 {
                    if (-c as u64) > self.state[j] {
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                return Some(i)
            }
        }
        None
    }

    pub fn find_rule_info(&self, info: &mut Vec<usize>, cat: bool) -> Option<usize> {
        for i in 0..self.rules.height {
            let rule = self.rules.get_row(i);
            let mut valid = true;

            for (j, &c) in rule.into_iter().enumerate() {
                if c < 0 {
                    if (-c as u64) > self.state[j] {
                        if cat {
                            info[j] -= 1;
                        } else {
                            info[j] += 1;
                        }
                        valid = false;
                        break;
                    }
                }
            }

            if valid {
                return Some(i)
            }
        }
        None
    }

    pub fn step_info(&mut self, info: &mut Vec<usize>, cat: bool) -> bool {
        let rulew = self.find_rule_info(info, cat);
        if let Some(rulei) = rulew {
            let rule = self.rules.get_row(rulei);
            for (i,v) in self.state.clone().iter().enumerate() {
                self.state[i] = v.wrapping_add_signed(rule[i] as i64);
            }

            return true;
        } else {
            return false;
        }
    }

    pub fn step(&mut self) -> bool {
        let rulew = self.find_rule();
        if let Some(rulei) = rulew {
            let rule = self.rules.get_row(rulei);
            for (i,v) in self.state.clone().iter().enumerate() {
                self.state[i] = v.wrapping_add_signed(rule[i] as i64);
            }

            return true;
        } else {
            return false;
        }
    }

    pub fn cnt_step(&mut self) -> Option<usize> {
        let rulew = self.find_rule()?;
        let rule = self.rules.get_row(rulew);
        for (i, v) in self.state.clone().iter().enumerate() {
            self.state[i] = v.wrapping_add_signed(rule[i] as i64);
        }
        return Some(rulew);
    }

    pub fn sim(&mut self, max: u64) -> Option<u64> {
        let mut steps = 0;
        while self.step() {
            steps += 1;

            if steps >= max {
                return None
            }
        }

        Some(steps)
    }

    pub fn cnt_sim(&mut self, max: u64) -> Vec<u64> {
        let mut state_cnts = vec![0; self.rules.height];
        let mut steps = 0;

        while let Some(st) = self.cnt_step() {
            state_cnts[st] += 1;
            steps += 1;

            if steps >= max {
                return state_cnts;
            }
        }

        state_cnts
    }

    pub fn new(state: Vec<u64>, rules: SVec2d) -> FractranProgram {
        FractranProgram { state, rules }
    }

    pub fn sz(&self) -> usize {
        self.rules.elements().iter().map(|&a| a.abs() as usize).sum::<usize>() + self.rules.height
    }
}

impl From<String> for FractranProgram {
    fn from(value: String) -> Self {
        let fracs = value[1..value.len()-1].split(", ");

        let mut rules = SVec2d::default();

        for (idy, frac) in fracs.enumerate() {
            if idy > rules.height - 1 { rules.new_row(); }

            let mut items = frac.split("/");

            let mut num = items.next().expect("invalid program").parse::<i32>().expect("invalid program");
            let mut den = items.next().expect("invalid program").parse::<i32>().expect("invalid program");

            for (idx,prime) in PRIMES.iter().enumerate() {
                while num % prime == 0 {
                    if idx > rules.width - 1 { rules.incwidth(); }

                    num /= prime;
                    rules.set(*rules.get(idx, idy) + 1, idx, idy);
                }
                while den % prime == 0 {
                    if idx > rules.width - 1 { rules.incwidth(); }

                    den /= prime;
                    rules.set(*rules.get(idx, idy) - 1, idx, idy);
                }
            }
        }

        let mut svec = vec![1];
        svec.extend(vec![0;rules.width - 1]);
        return FractranProgram::new(svec, rules)
    }
}

impl ToString for FractranProgram {
    fn to_string(&self) -> String {
        format!("{:?}|{:?}", self.rules.width, self.rules.elements())
    }
}