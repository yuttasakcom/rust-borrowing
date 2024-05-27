struct Person {
    name: String,
}

fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    let mut s = "Ping".to_string();
    println!("Before Mutate: s = {}", s);
    mutate(&mut s);
    println!("After Mutate: s = {}", s);

    let mut p = Person {
        name: String::from("Yo"),
    };
    println!("Before Mutate: p.name = {}", p.name);
    mutate_struct(&mut p);
    println!("After Mutate: p.name = {}", p.name);
}

fn change(str: &mut String) {
    str.push_str(", world");
}

fn mutate(s: &mut String) {
    *s = "Pong".to_string();
}

fn mutate_struct(p: &mut Person) {
    p.name.push_str(" Yuttasak");
}
