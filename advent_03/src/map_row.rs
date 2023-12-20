use crate::engine_part::EnginePart;

pub struct MapRow {
    parts: Vec<EnginePart>,
}

impl MapRow {
    pub fn new(parts: Vec<EnginePart>) -> MapRow {
        MapRow { parts: parts }
    }

    pub fn check_collision(&self, pos: &usize) -> Vec<&EnginePart> {
        let mut result: Vec<&EnginePart> = Vec::new();

        for p in self.parts.iter() {
            let is_in_bounds = p.in_bounds(pos);
            if is_in_bounds {
                result.push(p);
            }
        }
        return result;
    }
}