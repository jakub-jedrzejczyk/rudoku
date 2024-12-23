use crate::dto::celldto::CellDTO;
use crate::dto::regiondto::RegionDTO;

pub struct BoardDTO {
    pub size_x: u16,
    pub size_y: u16,
    pub grid: Vec<Vec<CellDTO>>,
    pub regions: Vec<RegionDTO>,
}
