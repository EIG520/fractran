use crate::{deciders::Decision, program::program::FractranProgram};

#[derive(Clone, PartialEq, Eq)]
pub struct Vec2d<T> {
    pub elements: Vec<T>,
    pub width: usize
}

impl<T> Vec2d<T> {
    pub fn add_row(&mut self, row: Vec<T>) {
        assert_eq!(row.len(), self.width);

        self.elements.extend(row);
    }
    
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.elements[y * self.width + x]
    }
    pub fn set(&mut self, v: T, x: usize, y: usize) {
        self.elements[y * self.width + x] = v;
    }

    pub fn get_row(&self, y: usize) -> &[T] {
        &self.elements[y * self.width..(y+1) * self.width]
    }

    pub fn last_row(&self) -> &[T] {
        &self.elements[self.width * (self.height() - 1)..]
    }

    pub fn pop_row(&mut self) -> Vec<T> {
        self.elements.split_off(self.elements.len() - self.width)
    }

    pub fn height(&self) -> usize {
        (self.elements.len() + self.width - 1) / self.width
    }

    pub fn new(elements: Vec<T>, width: usize) -> Vec2d<T> {
        Vec2d { elements, width }
    }
}

impl <T: Clone> Vec2d<T> {
    pub fn incwidth(&mut self, default: T) {
        let mut nvec: Vec<T> = vec![];

        for i in 0..self.height() {
            let row = self.get_row(i);
            nvec.extend(row.to_vec());
            nvec.push(default.clone());
        }

        self.elements = nvec;
        self.width += 1;
    }

    pub fn decwidth(&mut self) {
        let mut nvec: Vec<T> = vec![];
        for i in 0..self.height() {
            let row = self.get_row(i);
            nvec.extend(row.to_vec());
            nvec.pop();
        }
        self.elements = nvec;
        self.width -= 1;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RHStatus {
    Top,
    Bottom,
}

#[derive(Clone)]
pub struct RuleHolder {
    pub elements_top: Vec2d<i8>,
    pub elements_bottom: Vec2d<i8>,
    pub open: RHStatus,
    pub new: bool,
    pub dec: Option<Decision>,
}

impl RuleHolder {
    pub fn tomod(&mut self) -> &mut Vec2d<i8> {
        match self.open {
            RHStatus::Top => &mut self.elements_top,
            RHStatus::Bottom => &mut self.elements_bottom
        }
    }

    pub fn to_vec2d(&self) -> Vec2d<i8> {
        let mut nvec: Vec<i8> = vec![];
        
        let lr = self.elements_top.last_row();

        nvec.extend(lr);
        nvec.extend(vec![0; self.elements_top.width - lr.len()]);


        for i in 2..self.elements_top.height() {
            nvec.extend(self.elements_top.get_row(self.elements_top.height() - i));
        }

        let mut use_bottom = true;
        if self.elements_bottom.height() == 1 {
            let mut all_zero = true;
            for &k in self.elements_bottom.last_row() {
                all_zero &= k == 0;
            }
            use_bottom = !all_zero;
        }

        if use_bottom {
            for i in 0..(self.elements_bottom.height() - 1) {
                nvec.extend(self.elements_bottom.get_row(i));
            }

            let lrb = self.elements_bottom.last_row();

            nvec.extend(lrb);
            nvec.extend(vec![0; self.elements_bottom.width - lrb.len()]);
        }
        Vec2d::new(nvec, self.elements_top.width)
    }

    pub fn to_fractran(&self) -> FractranProgram {
        let mut state = vec![1];
        state.extend(vec![0; self.elements_bottom.width - 1]);

        FractranProgram { state, rules: self.to_vec2d() }
    }

    pub fn absum(&self) -> usize {
        let v = self.elements_bottom.elements.iter().map(|a| a.abs() as usize).sum::<usize>() 
            + self.elements_top.elements.iter().map(|a| a.abs() as usize).sum::<usize>()
            + self.elements_bottom.height()
            + self.elements_top.height();

        if self.open == RHStatus::Top {
            return v - 1;
        } else {
            return v;
        }
    }
}

impl Default for RuleHolder {
    fn default() -> Self {
        Self {
            elements_top: Vec2d::new(vec![-1], 1),
            elements_bottom: Vec2d::new(vec![0], 1),
            open: RHStatus::Top,
            new: true,
            dec: None,
        }
    }
}