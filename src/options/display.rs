pub struct DisplayOptions {
    padding_left: u16,
    padding_right: u16,
    padding_top: u16,
    padding_bottom: u16,
    inner_border_horizontal: u16,
    inner_border_vertical: u16,
    outer_border_horizontal: u16,
    outer_border_vertical: u16,
    inner_horizontal_char: char,
    inner_vertical_char: char,
    inner_intersection_char: char,
    outer_horizontal_char: char,
    outer_vertical_char: char,
    outer_intersection_char: char,
    center_board: bool
}

impl DisplayOptions {
    pub fn new() -> DisplayOptions {
        DisplayOptions {
            padding_left: 1,
            padding_right: 1,
            padding_top: 1, 
            padding_bottom: 1,
            inner_border_horizontal: 1,
            inner_border_vertical: 1,
            outer_border_horizontal: 1,
            outer_border_vertical: 1,
            inner_horizontal_char: '-',
            inner_vertical_char: '|',
            inner_intersection_char: '+',
            outer_horizontal_char: '-',
            outer_vertical_char: '|',
            outer_intersection_char: '+',
            center_board: true
        }
    }

    pub fn get_padding_left(&self) -> u16 {
        self.padding_left
    }

    pub fn get_padding_right(&self) -> u16 {
        self.padding_right
    }

    pub fn get_padding_top(&self) -> u16 {
        self.padding_top
    }
    
    pub fn get_padding_bottom(&self) -> u16 {
        self.padding_bottom
    }

    pub fn get_inner_border_horizontal(&self) -> u16 {
        self.inner_border_horizontal 
    }

    pub fn get_inner_border_vertical(&self) -> u16 {
        self.inner_border_vertical 
    }

    pub fn get_outer_border_horizontal(&self) -> u16 {
        self.outer_border_horizontal 
    }

    pub fn get_outer_border_vertical(&self) -> u16 {
        self.outer_border_vertical 
    }

    pub fn get_inner_horizontal_char(&self) -> char {
        self.inner_horizontal_char 
    }

    pub fn get_inner_vertical_char(&self) -> char {
        self.inner_vertical_char 
    }

    pub fn get_inner_intersection_char(&self) -> char {
        self.inner_intersection_char 
    }

    pub fn get_outer_horizontal_char(&self) -> char {
        self.outer_horizontal_char 
    }

    pub fn get_outer_vertical_char(&self) -> char {
        self.outer_vertical_char 
    }

    pub fn get_outer_intersection_char(&self) -> char {
        self.outer_intersection_char 
    }

    pub fn get_center_board(&self) -> bool {
        self.center_board 
    }

    pub fn set_inner_border_horizontal(&mut self, inner_border_horizontal: u16) {
        self.inner_border_horizontal = inner_border_horizontal;
    }
}
