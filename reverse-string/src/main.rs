use unicode_segmentation::UnicodeSegmentation;

fn main() {
    println!("{}", reverse_string("uüu"));
}

fn reverse_string(input: &str) -> String {
    let input_chars = input.graphemes(true).rev();
    input_chars.collect::<String>()
}
