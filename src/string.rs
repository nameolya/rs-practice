pub fn run() {
    let _hello = "hello";
    let mut greeting = String::from("hello ");

    println!("{}", greeting);
    println!("_hello length: {}", _hello.len());
    println!("greeting length: {}", greeting.len());

    greeting.push('w');
    greeting.push_str("orld!");

    println!("greeting: {}", greeting);

    println!("Contains world: {}", greeting.contains("world"));

    println!("Replace: {}", greeting.replace("world", "there"));

    for word in greeting.split_whitespace() {
        println!("{}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    assert_eq!(10, s.capacity());

}