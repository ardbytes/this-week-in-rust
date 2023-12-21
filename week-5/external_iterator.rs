fn contains(v: &Vec<i32>, needle: i32) -> bool {
    for x in v {
        if *x == needle {
            return true;
        }
    }
    false
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4];
    dbg!(contains(&v, 4));
}
