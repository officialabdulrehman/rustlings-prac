// clippy3.rs
// Here's a couple more easy Clippy fixes, so you can see its utility.

use std::mem::swap;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(..) = my_option {
        my_option.unwrap();
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    println!("This Vec is empty, see? {:?}", vec![1, 2, 3, 4, 5].clear());

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    swap(&mut value_a, &mut value_b);

    println!("value a: {}; value b: {}", value_a, value_b);
}
