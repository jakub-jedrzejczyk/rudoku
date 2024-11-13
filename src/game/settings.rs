pub struct Settings {
    field_height: i16,
    field_width: i16,
    inner_border_horizontal: i16,
    inner_border_vertical: i16,
    outer_border_horizontal: i16,
    outer_border_vertical: i16,
    inner_horizontal_char: char,
    inner_vertical_char: char,
    inner_intersection_char: char,
    outer_horizontal_char: char,
    outer_vertical_char: char,
    outer_intersection_char: char
}

impl Settings {
    pub fn new() -> Settings {
        Settings {
            field_height: 3,
            field_width: 3,
            inner_border_horizontal: 1,
            inner_border_vertical: 1,
            outer_border_horizontal: 1,
            outer_border_vertical: 1,
            inner_horizontal_char: '-',
            inner_vertical_char: '|',
            inner_intersection_char: '+',
            outer_horizontal_char: '═',
            outer_vertical_char: '║',
            outer_intersection_char: '╬'
        }
    }

    pub fn get_field_height(&self) -> i16 {
        self.field_height 
    }

    pub fn get_field_width(&self) -> i16 {
        self.field_width 
    }

    pub fn get_inner_border_horizontal(&self) -> i16 {
        self.inner_border_horizontal 
    }

    pub fn get_inner_border_vertical(&self) -> i16 {
        self.inner_border_vertical 
    }

    pub fn get_outer_border_horizontal(&self) -> i16 {
        self.outer_border_horizontal 
    }

    pub fn get_outer_border_vertical(&self) -> i16 {
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

    pub fn set_field_height(&mut self, field_height: i16) {
        self.field_height = field_height;
    }

    pub fn set_field_width(&mut self, field_width: i16) {
        self.field_width = field_width;
    }

    pub fn set_inner_border_horizontal(&mut self, inner_border_horizontal: i16) {
        self.inner_border_horizontal = inner_border_horizontal;
    }
}
