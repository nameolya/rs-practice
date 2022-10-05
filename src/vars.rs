pub fn run() {
    let name = "Miki";
    let mut age = 4;

    println!("My name is {} and I am {}", name, age);

    age = 5;

    println!("My name is {} and I am {}", name, age);

    let ( my_name, my_age ) = ("Miki", 4);
    println!("{} is {}", my_name, my_age);

}