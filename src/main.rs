use std::rc::Rc;

fn main() {
    let data = Rc::new("Hello World!");
    let data_clone = Rc::clone(&data);
    let data_clone2 = Rc::clone(&data);

    println!("Original: {}", data);
    println!("Clone 1: {}", data_clone);
    println!("Clone 2: {}", data_clone2);

    println!("Reference Count: {}", Rc::strong_count(&data));

    take_ownership(data_clone);

    println!("Reference Count: {}", Rc::strong_count(&data));
}

fn take_ownership(data: Rc<&str>) {
    println!("Data in function: {}", data);
}
