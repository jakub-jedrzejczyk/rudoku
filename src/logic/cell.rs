use crate::dto::celldto::CellDTO;

pub struct Cell {
    correct_value: u8,
    is_predefined: bool,
    user_value: u8
}

impl Cell {
    pub fn new_empty() -> Cell {
        return Cell {
            correct_value: 0,
            is_predefined: false,
            user_value: 0
        }
    }

    pub fn new(correct_value: u8, is_visible: bool, user_value: u8) -> Cell {
        return Cell {
            correct_value,
            is_predefined: is_visible,
            user_value
        }
    } 

    pub fn get_value(&self) -> u8 {
        if self.is_predefined {
            return self.correct_value;
        }

        return self.user_value;
    }

    pub fn get_char(&self) -> char {
        let labels = [' ', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

        let index = if self.is_predefined { self.correct_value } else { self.user_value };

        return labels[index as usize];
    }

    pub fn set_correct_value(&mut self, v: u8) {
        self.correct_value = v;
    }

    pub fn set_user_value(&mut self, v: u8) {
        self.user_value = v;
    }

    pub fn set_predefined(&mut self, v: bool) {
        self.is_predefined = v;
    }

    pub fn as_dto(&self) -> CellDTO {
        return CellDTO {
            c: self.get_char(),
        }
    }
}
