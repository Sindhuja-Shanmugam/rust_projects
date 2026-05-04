// use std::any::type_name;
// use std::fmt::Debug;

// fn find_type_name<T:Debug>(v:&T){
//     println!("{}",type_name::<T>());
//     println!("{:?}",v);
// }

// fn main() {
//     let mut a;
//     a="Helo".to_string();
//     let b:&str="H";
//     find_type_name(&a);
//     find_type_name(&b);
//     println!("{}",a);
//     println!("{}",b);
//     //find_type_name(&b);
// }


mod to_do;

fn main() {
    println!("Running main file");

    // call function from to_do.rs
    to_do::main();
}