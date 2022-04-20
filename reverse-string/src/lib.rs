use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    let iter = input.graphemes(true);

    for unicode in iter.rev() {
        reversed.push_str(unicode);
    }
    reversed
}
