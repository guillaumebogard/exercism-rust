#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list == [] && _second_list == [] {
        return Comparison::Equal;
    }

    if _first_list == [] {
        return Comparison::Sublist;
    }

    if _second_list == [] {
        return Comparison::Superlist;
    }

    if _second_list.len() >= _first_list.len() {
        let mut sl_i = 0;

        while sl_i < _second_list.len() {
            let mut sl_j = sl_i;
            let mut fl_i = 0;

            while _second_list[sl_j] == _first_list[fl_i] {
                sl_j += 1;
                fl_i += 1;
                if fl_i == _first_list.len() {
                    if sl_j == _second_list.len() && sl_i == 0 {
                        return Comparison::Equal;
                    } else {
                        return Comparison::Sublist;
                    }
                }
                if sl_j == _second_list.len() {
                    break;
                }
            }
            sl_i += 1;
        }
    } else {
        let mut fl_i = 0;

        while fl_i < _first_list.len() {
            let mut fl_j = fl_i;
            let mut sl_i = 0;

            while _first_list[fl_j] == _second_list[sl_i] {
                fl_j += 1;
                sl_i += 1;
                if sl_i == _second_list.len() {
                    if fl_j == _first_list.len() && fl_i == 0 {
                        return Comparison::Equal;
                    } else {
                        return Comparison::Superlist;
                    }
                }
                if fl_j == _first_list.len() {
                    break;
                }
            }
            fl_i += 1;
        }
    }
    Comparison::Unequal
}
