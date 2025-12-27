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

#[derive(Clone)]
pub struct SVec2d {
    elements: Vec<i8>,
    pub width: usize,
    pub lwidth: usize,
    twidth: usize,
    pub height: usize,
}

impl SVec2d {
    pub fn new_row(&mut self) {
        self.height += 1;
        self.lwidth = 1;
        if self.theight() >= self.height { return; }

        self.elements.extend(vec![0; self.twidth]);
    }

    pub fn size(&mut self) -> usize {
        self.elements.len()
    }

    pub fn rem_row(&mut self) {
        for i in 0..self.width {
            self.set(0, i, self.height - 1);
        }

        self.lwidth = self.width;
        self.height -= 1;
    }

    pub fn incwidth(&mut self) {
        self.width += 1;
        self.lwidth += 1;
        if self.twidth >= self.width { return; }
        self.twidth *= 2;

        let mut nvec: Vec<i8> = vec![];
        for i in 0..self.theight() {
            let row = self.get_trow(i);
            nvec.extend(row.to_vec());
            nvec.extend(vec![0; row.len()]);
        }

        self.elements = nvec;
    }

    pub fn push_last(&mut self) {
        self.lwidth += 1;
    }

    pub fn pop_last(&mut self) {
        self.set(0, self.width - 1, self.height - 1);
        self.lwidth -= 1;
    }

    pub fn decwidth(&mut self) {
        for i in 0..self.height {
            self.set(0, self.width - 1, i);
        }

        self.lwidth -= 1;
        self.width -= 1;
    }

    pub fn get(&self, x: usize, y: usize) -> &i8 {
        &self.elements[y * self.twidth + x]
    }
    pub fn set(&mut self, v: i8, x: usize, y: usize) {
        self.elements[y * self.twidth + x] = v;
    }

    pub fn get_row(&self, y: usize) -> &[i8] {
        &self.elements[y * self.twidth..y * self.twidth + self.width]
    }

    pub fn get_trow(&self, y: usize) -> &[i8] {
        &self.elements[y * self.twidth..y * self.twidth + self.twidth]
    }

    pub fn last_row(&self) -> &[i8] {
        &self.elements[self.twidth * (self.height - 1)..self.twidth * (self.height - 1) + self.width]
    }

    pub fn elements(&self) -> Vec<i8> {
        let mut nvec = vec![];
        for i in 0..self.height {
            nvec.extend(self.get_row(i));
        }
        nvec
    }

    fn theight(&mut self) -> usize {
        (self.elements.len() + self.width - 1) / self.width
    }

    pub fn new(elements: Vec<i8>, width: usize) -> Self {
        SVec2d {
            elements: elements.clone(),
            width,
            lwidth: width,
            twidth: width,
            height: (elements.len() + width - 1) / width
        }
    }
}

impl Default for SVec2d {
    fn default() -> Self {
        Self {
            elements: vec![0; 400],
            width: 1,
            lwidth: 1,
            twidth: 20,
            height: 1
        }
    }
}

// #[derive(Clone, Copy, PartialEq, Eq, Debug)]
// pub enum RHStatus {
//     Top,
//     Bottom,
// }

// #[derive(Clone)]
// pub struct RuleHolder {
//     pub elements_top: Vec2d<i8>,
//     pub elements_bottom: Vec2d<i8>,
//     pub open: RHStatus,
//     pub new: bool,
//     pub dec: Option<Decision>,
// }

// impl RuleHolder {
//     pub fn tomod(&mut self) -> &mut Vec2d<i8> {
//         match self.open {
//             RHStatus::Top => &mut self.elements_top,
//             RHStatus::Bottom => &mut self.elements_bottom
//         }
//     }

//     pub fn to_vec2d(&self) -> Vec2d<i8> {
//         let mut nvec: Vec<i8> = vec![];
        
//         let lr = self.elements_top.last_row();

//         nvec.extend(lr);
//         nvec.extend(vec![0; self.elements_top.width - lr.len()]);


//         for i in 2..self.elements_top.height() {
//             nvec.extend(self.elements_top.get_row(self.elements_top.height() - i));
//         }

//         let mut use_bottom = true;
//         if self.elements_bottom.height() == 1 {
//             let mut all_zero = true;
//             for &k in self.elements_bottom.last_row() {
//                 all_zero &= k == 0;
//             }
//             use_bottom = !all_zero;
//         }

//         if use_bottom {
//             for i in 0..(self.elements_bottom.height() - 1) {
//                 nvec.extend(self.elements_bottom.get_row(i));
//             }

//             let lrb = self.elements_bottom.last_row();

//             nvec.extend(lrb);
//             nvec.extend(vec![0; self.elements_bottom.width - lrb.len()]);
//         }
//         Vec2d::new(nvec, self.elements_top.width)
//     }

//     pub fn to_fractran(&self) -> FractranProgram {
//         let mut state = vec![1];
//         state.extend(vec![0; self.elements_bottom.width - 1]);

//         FractranProgram { state, rules: self.to_vec2d() }
//     }

//     pub fn absum(&self) -> usize {
//         let v = self.elements_bottom.elements.iter().map(|a| a.abs() as usize).sum::<usize>() 
//             + self.elements_top.elements.iter().map(|a| a.abs() as usize).sum::<usize>()
//             + self.elements_bottom.height()
//             + self.elements_top.height();

//         if self.open == RHStatus::Top {
//             return v - 1;
//         } else {
//             return v;
//         }
//     }
// }

// impl Default for RuleHolder {
//     fn default() -> Self {
//         Self {
//             elements_top: Vec2d::new(vec![-1], 1),
//             elements_bottom: Vec2d::new(vec![0], 1),
//             open: RHStatus::Top,
//             new: true,
//             dec: None,
//         }
//     }
// }