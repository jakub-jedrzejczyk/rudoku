use crate::logic::board::Board;
use crate::dto::gamedto::GameDTO;

pub struct Game {
    cursor_x: u16,
    cursor_y: u16,
    board: Board
}

impl Game {
    pub fn new(
        cursor_x: u16,
        cursor_y: u16,
        board: Board,
    ) -> Game {
        return Game {
            cursor_x,
            cursor_y,
            board,
        };
    }

    pub fn get_size_x(&self) -> u16 {
        self.board.size_x
    }

    pub fn get_size_y(&self) -> u16 {
        self.board.size_y
    }

    pub fn get_cursor_x(&self) -> u16 {
        self.cursor_x
    }

    pub fn get_cursor_y(&self) -> u16 {
        self.cursor_y
    }

    pub fn as_dto(&self) -> GameDTO {
        return GameDTO {
            cursor_x: self.cursor_x,
            cursor_y: self.cursor_y,
            board_dto: self.board.as_dto(),
        };
    }
}

