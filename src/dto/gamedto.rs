use crate::dto::boarddto::BoardDTO;

pub struct GameDTO {
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub board_dto: BoardDTO
}
