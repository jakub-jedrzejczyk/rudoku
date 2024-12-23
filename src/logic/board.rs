use crate::logic::region::Region;
use crate::logic::cell::Cell;
use crate::dto::boarddto::BoardDTO;

pub struct Board {
    pub size_x: u16,
    pub size_y: u16,
    pub grid: Vec<Vec<Cell>>,
    pub regions: Vec<Region>,
}

impl Board {
    pub fn new_standard(size_x: u16, size_y: u16, regions_x: u16, regions_y: u16) -> Board {
        let mut grid = Vec::with_capacity(size_x as usize);
        for _ in 0..size_x {
            let mut row = Vec::with_capacity(size_y as usize);
            for _ in 0..size_y {
                row.push(Cell::new_empty());
            }
            grid.push(row);
        }

        let mut regions = Vec::with_capacity(regions_x as usize);
        for x in 0..regions_x {
            for y in 0..regions_y {
                regions.push(Region::new_square(
                    (x * size_x / regions_x) as usize,
                    (y * size_y / regions_y) as usize,
                    (size_x / regions_x) as usize,
                    (size_y / regions_y) as usize,
                ));
            }
        }

        return Board {
            size_x,
            size_y,
            grid,
            regions,
        };
    }

    pub fn as_dto(&self) -> BoardDTO {
        let mut grid_dto = Vec::with_capacity(self.size_x as usize);
        for row in &self.grid {
            let mut row_dto = Vec::with_capacity(self.size_y as usize);
            for cell in row {
                row_dto.push(cell.as_dto());
            }
            grid_dto.push(row_dto);
        }

        let mut regions_dto = Vec::with_capacity(self.regions.len());
        for region in &self.regions {
            regions_dto.push(region.as_dto());
        }

        return BoardDTO {
            size_x: self.size_x,
            size_y: self.size_y,
            grid: grid_dto,
            regions: regions_dto,
        };
    }
}
