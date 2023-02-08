fn reverse_string(s: &mut String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let mut s1 = String::from("any string");

    let _s2 = reverse_string(&mut s1);

    println!("{}", _s2)
}