pub fn padding(num: usize, length: usize) -> String {
    let mut output = num.to_string();
    while output.len() < length {
        output = format!("0{}", output);
    }
    output
}
