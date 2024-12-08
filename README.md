# Rust: Unsafe Raw Pointer Modification Leading to Undefined Behavior
This repository demonstrates a potential bug in Rust involving unsafe code and mutable borrows.
The `bug.rs` file shows how directly manipulating a vector through a raw pointer (`as_mut_ptr()`) after obtaining a mutable reference can cause undefined behavior, even if it seemingly works in simple cases. The `bugSolution.rs` file provides a corrected version that avoids undefined behavior by managing memory access carefully.  This is a subtle error and a good example of how unsafe Rust code needs meticulous handling.