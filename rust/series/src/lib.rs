pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut substr: String = String::new();
    let mut idx: usize = 0;

    if len > digits.len() {
        return ret;
    }
    while idx < len{
        substr.push(digits.chars().nth(idx).unwrap());
        idx += 1;
    }
    while idx < digits.len() + 1 {
        ret.push(substr.clone());
        if idx == digits.len() {
            break;
        }
        substr.push(digits.chars().nth(idx).unwrap());
        substr.remove(0);
        idx += 1;
    }

    return ret;
}
