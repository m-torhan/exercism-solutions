pub fn is_armstrong_number(num: u32) -> bool {
    let mut v = Vec::<u128>::new();
    let mut i = 0;
    let mut n = u128::from(num);
    while n > 0 {
        v.push(n % 10);
        n /= 10;
        i += 1;
    }
    let mut s = 0;
    while v.len() > 0 {
        s += v.pop().expect("ERROR").pow(i);
        if s > u128::from(num) {
            return false;
        }
    }
    return s == u128::from(num);
}
