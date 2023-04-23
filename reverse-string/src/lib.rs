use unicode_segmentation::UnicodeSegmentation; 

pub fn reverse(input: &str) -> String {
    let input_chars = input.graphemes(true).rev();
    input_chars.collect::<String>()
}
