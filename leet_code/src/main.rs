fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    println!("{:?}", s);
    reverse_string(&mut s);
    println!("{:?}", s);

    let mut ss = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    println!("{:?}", ss);
    reverse_string(&mut ss);
    println!("{:?}", ss);

    let mut sss = vec!['H', 'a'];
    println!("{:?}", sss);
    reverse_string(&mut sss);
    println!("{:?}", sss);
}
 
pub fn reverse_string(s: &mut Vec<char>) {
    let len = s.len();
    println!("len : {len}");
    for i in 0..len / 2 {
        println!("index: {i}");
        s.swap(0 + i, len - 1 - i);
    }
}
