fn contains(v: &Vec<i32>, needle: i32) -> bool {
    v.iter().position(|x| {
        *x == needle 
    }).is_some()
}

fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4];
    dbg!(contains(&v, 4));
}
