pub fn verse(n: u32) -> String {
    if n == 0 {
        return "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string();
    }
    let mut s1 = String::new();
    let mut s2 = String::new();
    if n > 1 {
        s1 = "s".to_string();
    }
    else {
        s1 = "".to_string();
    }
    if n != 2 {
        s2 = "s".to_string();
    }
    else {
        s2 = "".to_string();
    }
    let line_1 = format!("{n} bottle{s1} of beer on the wall, {n} bottle{s1} of beer.");
    let mut k = String::new();
    let mut o = String::new();
    if 0 == n - 1 {
        k = "no more".to_string();
        o = "it".to_string();
    }
    else {
        k = (n - 1).to_string();
        o = "one".to_string();
    }
    let line_2 = format!("Take {o} down and pass it around, {k} bottle{s2} of beer on the wall.");
    return format!("{}\n{}\n", line_1, line_2);
}

pub fn sing(start: u32, end: u32) -> String {
    return (end..start+1).rev().map(|n| verse(n)).collect::<Vec<String>>().join("\n");
}
