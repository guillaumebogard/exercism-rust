use int_enum::IntEnum;
use enum_iterator::IntoEnumIterator;

#[repr(usize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black  = 0,
    Brown  = 1,
    Red    = 2,
    Orange = 3,
    Yellow = 4,
    Green  = 5,
    Blue   = 6,
    Violet = 7,
    Grey   = 8,
    White  = 9,
}

impl std::fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value()
}

pub fn value_to_color_string(value: usize) -> String {
    let result = ResistorColor::from_int(value);

    if result.is_err() {
        return String::from("value out of range");
    } else {
        return result.unwrap().to_string();
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut vector = vec![];
    let mut iterator = ResistorColor::into_enum_iter();
    let mut item = iterator.next();

    while item != None {
        vector.push(item.unwrap());
        item = iterator.next();
    }
    vector
}
