pub fn strings<const N: usize>(data: [&str; N]) -> Vec<String> {
    data.iter().map(|s| s.to_string()).collect()
}
