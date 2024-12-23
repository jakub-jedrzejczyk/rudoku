use crate::dto::regiondto::RegionDTO;

pub struct Region {
    pub cells: Vec<(usize, usize)>,
}

impl Region {
    pub fn new_square(start_x: usize, start_y: usize, size_x: usize, size_y: usize) -> Region {
        let mut cells = Vec::with_capacity(size_x * size_y);
        for x in start_x..(start_x + size_x) {
            for y in start_y..(start_y + size_y) {
                cells.push((x, y));
            }
        }
        return Region {
            cells,
        };
    }

    pub fn as_dto(&self) -> RegionDTO {
        return RegionDTO {
            size: self.cells.len(),
            cells: self.cells.clone(),
        };
    }
}
