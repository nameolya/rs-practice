use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1,2,3,4,5];
    
    println!("Single value: {}", numbers[0]);

    numbers[2] = 20;
    println!("{:?}", numbers);

    println!("Array length: {}", numbers.len());

    println!("This array occupies {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

}