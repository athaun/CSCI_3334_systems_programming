fn concat_strings(s1: &String, s2: &String) -> String {
    return s1.clone() + s2;
}

fn main() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}