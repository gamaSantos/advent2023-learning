pub struct EnginePart {
    pub id: usize,
    start: usize,
    end: usize,
    pub value: i32,
}

impl EnginePart {
    pub fn new(id: usize, start: usize, end: usize, value: i32) -> EnginePart {
        EnginePart {
            id: id,
            start: start,
            end: end,
            value: value,
        }
    }

    pub fn in_bounds(&self, pos: &usize) -> bool {
        let r = pos >= &self.start && pos <= &self.end;
        return r;
    }
}