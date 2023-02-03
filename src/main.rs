struct User {
    name: String,
    age: u32,
    email: String,
}

impl User {
    fn say_hello(&self) {
        println!("hello, i am {}, {} years old, my email is {}", self.name, self.age, self.email);
    }

    fn change_age(&mut self, age: u32) {
        self.age = age;
    }
}

fn main() {
    let mut someone = User {
        name: String::from("hellen"),
        age: 14,
        email: String::from("hellen@gmail.com"),
    };

    someone.change_age(20);
    someone.say_hello();
}

// fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
//     if a > b {
//         a
//     } else {
//         b
//     }
// }

// fn main () {
//     println!("{}", largest(1000, 2))
// }