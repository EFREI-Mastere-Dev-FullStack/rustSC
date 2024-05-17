pub fn get_char(val: f64) -> char {
    let char_res = match val.abs() {
        v if v < 0.1 => '.',
        v if v < 0.2 => '#', // wall
        v if v < 0.6 => '.',
        v if v < 0.9 => '0',
        v if v <= 1.0 => ' ',
        _ => panic!("unexpected value")
    };
    char_res
}