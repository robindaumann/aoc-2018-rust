
fn main() {
    let mut v = Vec::new();

    v.push(1);

    unsafe {
        println!("{}", v.get_unchecked(100000)); 
    } 
}
