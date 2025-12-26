use crate::program::arr2d::Vec2d;
const PRIMES: [i32; 19] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67];

#[derive(Clone)]
pub struct FractranProgram {
    pub state: Vec<u64>,
    pub rules: Vec2d<i8>,
}

impl FractranProgram {
    pub fn find_rule(&self) -> Option<usize> {
        for i in 0..self.rules.height() {
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
        for i in 0..self.rules.height() {
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

    pub fn new(state: Vec<u64>, rules: Vec2d<i8>) -> FractranProgram {
        FractranProgram { state, rules }
    }

    pub fn sz(&self) -> usize {
        self.rules.elements.iter().map(|&a| a.abs() as usize).sum::<usize>() + self.rules.height()
    }
}

impl From<String> for FractranProgram {
    fn from(value: String) -> Self {
        let fracs = value[1..value.len()-1].split(", ");

        let mut rules: Vec<i8> = vec![];

        for frac in fracs {
            let mut items = frac.split("/");

            let mut num = items.next().expect("invalid program").parse::<i32>().expect("invalid program");
            let mut den = items.next().expect("invalid program").parse::<i32>().expect("invalid program");

            let mut rule = vec![0;20];

            for (idx,prime) in PRIMES.iter().enumerate() {
                while num % prime == 0 {
                    num /= prime;
                    rule[idx] += 1;
                }
                while den % prime == 0 {
                    den /= prime;
                    rule[idx] -= 1;
                }
            }

            println!("{:?}", rule);


            rules.extend(rule);
        }

        let mut svec = vec![1];
        svec.extend(vec![0;19]);
        return FractranProgram::new(svec, Vec2d::new(rules,20))
    }
}

impl ToString for FractranProgram {
    fn to_string(&self) -> String {
        format!("{:?}|{:?}", self.rules.width, self.rules.elements)
    }
}