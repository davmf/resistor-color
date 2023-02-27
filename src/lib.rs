use int_enum::IntEnum;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Copy, Clone, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(_color: ResistorColor) -> u32 {
    _color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    ResistorColor::from_int(value)
        .map_or("",
                |result|
        match result {
            ResistorColor::Black => "Black",
            ResistorColor::Brown => "Brown",
            ResistorColor::Red => "Red",
            ResistorColor::Orange => "Orange",
            ResistorColor::Yellow => "Yellow",
            ResistorColor::Green => "Green",
            ResistorColor::Blue => "Blue",
            ResistorColor::Violet => "Violet",
            ResistorColor::Grey => "Grey",
            ResistorColor::White => "White",
        }
    }


pub fn colors() -> Vec<ResistorColor> {
    unimplemented!("return a list of all the colors ordered by resistance")
}
