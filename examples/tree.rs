use std::cell::RefCell;

fn main() {
    let data = RefCell::new(42);

    {
        let mut data_ref_mut = data.borrow_mut();
        *data_ref_mut += 1;
    }

    let data_ref: std::cell::Ref<'_, i32> = data.borrow();

    println!("DAta: {}", data_ref)
}
