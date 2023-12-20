use std:: fs;

use engine_part::EnginePart;
use map_row::MapRow;

mod map_row;
mod engine_part;

struct GraphMap {
    rows: Vec<MapRow>,
    // retrived: Vec<usize>,
}

impl GraphMap {
    fn new(rows: Vec<MapRow>) -> GraphMap {
        GraphMap {
            rows: rows,
            // retrived: Vec::new(),
        }
    }

    fn calculate_engine_rates(&mut self, row_idx: usize, pos: &usize) -> i64 {
        let bellow_index = Self::get_bellow_row_idx(row_idx, 140);
        let mut parts: Vec<&EnginePart> = Vec::new();

        if row_idx > 0 {
            let upper_index = Self::get_upper_row_idx(row_idx);
            parts.append(self.get_adjacent_row_part_values(upper_index, pos).as_mut());
        }
        parts.append(self.get_current_row_parts(row_idx, pos).as_mut());

        if bellow_index < self.rows.len() {
            parts.append(
                self.get_adjacent_row_part_values(bellow_index, pos)
                    .as_mut(),
            );
        }
        let unique = Self::get_unique_parts(parts);
        if unique.len() != 2 {
            return 0;
        }
        return unique[0].value as i64 * unique[1].value as i64;
    }

    fn get_unique_parts(parts: Vec<&EnginePart>) -> Vec<&EnginePart> {
        let mut unique: Vec<&EnginePart> = Vec::new();
        let mut computed: Vec<usize> = Vec::new();

        for eg in parts  {
            if computed.contains(&eg.id) == false{
                computed.push(eg.id);
                unique.push(eg);
            }
        }

        return unique;
    }

    fn get_adjacent_row_part_values(&self, row_idx: usize, pos: &usize) -> Vec<&EnginePart> {
        let row = &self.rows[row_idx];
        let min_collision_idx: usize;
        let next_position = pos + 1;
        let mut parts: Vec<&EnginePart> = Vec::new();

        if pos == &0 {
            min_collision_idx = 0;
        } else {
            min_collision_idx = pos - 1;
        }
        let mut before = row.check_collision(&min_collision_idx);
        parts.append(before.as_mut());
        parts.append(row.check_collision(pos).as_mut());
        parts.append(row.check_collision(&next_position).as_mut());

        return parts;
    }

    fn get_current_row_parts(&self, row_idx: usize, pos: &usize) -> Vec<&EnginePart> {
        let row = &self.rows[row_idx];
        let min_collision_idx: usize;
        let next_position = pos + 1;
        let mut parts: Vec<&EnginePart> = Vec::new();

        if pos == &0 {
            min_collision_idx = 0;
        } else {
            min_collision_idx = pos - 1;
        }

        parts.append(row.check_collision(&min_collision_idx).as_mut());
        parts.append(row.check_collision(&next_position).as_mut());

        return parts;
    }

    // fn sum_parts(&mut self, row_idx: usize, pos: &usize) -> i32 {
    //     let mut result = 0;
    //     let bellow_index = Self::get_bellow_row_idx(row_idx, 140);
    //     let min_collision_idx: usize;
    //     let next_position = pos + 1;
    //     if pos == &0 {
    //         min_collision_idx = 0;
    //     } else {
    //         min_collision_idx = pos - 1;
    //     }
    //     if row_idx > 0 {
    //         let upper_index = Self::get_upper_row_idx(row_idx);
    //         result += self.sum_row_parts(upper_index, &min_collision_idx);
    //         result += self.sum_row_parts(upper_index, pos);
    //         result += self.sum_row_parts(upper_index, &next_position);
    //     }
    //     result += self.sum_row_parts(row_idx, &min_collision_idx);
    //     result += self.sum_row_parts(row_idx, &next_position);

    //     if bellow_index < self.rows.len() {
    //         result += self.sum_row_parts(bellow_index, &min_collision_idx);
    //         result += self.sum_row_parts(bellow_index, pos);
    //         result += self.sum_row_parts(bellow_index, &next_position);
    //     }

    //     return result;
    // }

    // fn sum_row_parts(&mut self, row_idx: usize, pos: &usize) -> i32 {
    //     let mut result = 0;
    //     let row = &self.rows[row_idx];
    //     let parts = row.check_collision(pos);
    //     for eg in parts {
    //         if self.retrived.contains(&eg.id) == false {
    //             self.retrived.push(eg.id);
    //             result = result + eg.value;
    //         }
    //     }
    //     return result;
    // }

    fn get_upper_row_idx(idx: usize) -> usize {
        if idx == 0 {
            return 0;
        }
        return idx - 1;
    }

    fn get_bellow_row_idx(idx: usize, max: usize) -> usize {
        if idx >= max {
            return max;
        }
        return idx + 1;
    }
}

fn main() {
    let file_path = "/advent2023-learning/advent_03/input.txt";
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let graph = input.split('\n');

    let mut vec_rows: Vec<MapRow> = Vec::new();
    let mut engine_map: Vec<Vec<usize>> = Vec::new();
    let mut ep_id: usize = 1;
    for row in graph {
        let mut mapped_row: Vec<EnginePart> = Vec::new();
        let mut symbols_row: Vec<usize> = Vec::new();
        let mut start_idx = 0;
        let mut end_idx = 0;
        let mut is_building_number = false;
        for (i, c) in row.chars().into_iter().enumerate() {
            if c.is_digit(10) {
                if is_building_number == false {
                    start_idx = i;
                    end_idx = i;
                    is_building_number = true;
                } else {
                    end_idx = i;
                }
                if i == row.len() - 1 {
                    let value = row[start_idx..end_idx + 1].parse::<i32>().unwrap();
                    mapped_row.push(EnginePart::new(ep_id, start_idx, end_idx, value));
                    ep_id = ep_id + 1;
                }
            } else {
                if is_building_number {
                    let value = row[start_idx..end_idx + 1].parse::<i32>().unwrap();
                    mapped_row.push(EnginePart::new(ep_id, start_idx, end_idx, value));
                    start_idx = 0;
                    end_idx = 0;
                    is_building_number = false;
                    ep_id = ep_id + 1;
                }
                if c == '*' {
                    symbols_row.push(i);
                }
            }
        }
        engine_map.push(symbols_row);
        vec_rows.push(MapRow::new(mapped_row));
    }
    let mut full_map = GraphMap::new(vec_rows);
    let mut result = 0;
    for (row_idx, symbol_row) in engine_map.iter().enumerate() {
        for pos in symbol_row {
            result += full_map.calculate_engine_rates(row_idx, pos);
        }
    }
    println!("{result}");
}
