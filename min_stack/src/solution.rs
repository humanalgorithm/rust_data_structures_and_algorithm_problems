use std::cmp;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MinStack {
    stack: Vec<(i32, i32)>,
}

impl MinStack {
    pub fn new() -> Self {
        return Self { stack: Vec::new() };
    }

    pub fn push(&mut self, val: i32) {
        if self.stack.is_empty() {
            self.stack.push((val, val));
            return;
        }
        let this_min = cmp::min(val, self.stack.last().unwrap().1);
        self.stack.push((val, this_min));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn top(&self) -> i32 {
        return self.stack.last().unwrap().0;
    }

    pub fn get_min(&self) -> i32 {
        return self.stack.last().unwrap().1;
    }
}
