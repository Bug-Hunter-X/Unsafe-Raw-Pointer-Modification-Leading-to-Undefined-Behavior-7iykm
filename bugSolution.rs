fn main() {
    let mut v = vec![1, 2, 3];
    let first_element_ptr = v.as_mut_ptr();
    unsafe {
        *first_element_ptr = 10; // Modification is limited to the first element
    }
    println!("The first element is: {}", v[0]); // Correctly prints 10

    // safer approach using get_mut method which will handle bound checks
    if let Some(first_element) = v.get_mut(0) {
        *first_element = 20;
        println!("The first element is: {}", v[0]); // Correctly prints 20
    }
} 