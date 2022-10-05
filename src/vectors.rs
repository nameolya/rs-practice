use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    
    println!("Single value: {}", numbers[0]);

    numbers[2] = 20;
    numbers.push(6);
    println!("{:?}", numbers);

    numbers.pop();
    println!("{:?}", numbers);

    println!("Vector length: {}", numbers.len());

    println!("This vector occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    for x in numbers.iter_mut() {
        *x *=2;
    }

    println!("Numbers vec: {:?}", numbers);

}