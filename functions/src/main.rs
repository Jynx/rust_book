fn main() {
    println!("Hello, world!");
    another_function(4, 10);
}

fn another_function(x: i32, y: i32) {
    println!("value: {}", x);
    println!("value of y : {}", y);
    println!("return value : {}", return_five());
    println!("plus one value: {}", plus_or_minus_one(10));
    println!("crazy loop val!: {}", crazy_loop());
    array_example();
}

// return keyword not necessary in rust unless early returning
fn return_five() -> i32 {
    5
}

fn plus_or_minus_one(x: i32) -> i32 {
   let return_val = if x < 10 {
    x + 1
   } else {
    x - 1
   };
   return_val
}

fn crazy_loop() -> i32 {
   let mut counter = 0;

   return loop {
       counter += 1;

       if counter == 10 {
           break counter + 5;
       }
   } 
}

fn array_example() {
    let sample_array: [i32; 5] = [0, 2, 3, 4, 10];
    for num in sample_array.iter() {
        println!("the array value is: {}", num);
    }
}