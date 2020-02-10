// Since intrinsic values like ints go on the stack, we'll use a very simple
//  structure which will be allocated on the heap.
struct SomeThing {
    tiny: i8,
}

fn main() {
    println!("\n=== Rust's ownership model implies borrowing  ===");

    let x = SomeThing { tiny: 74 };

    println!("- x is SomeThing type allocated on the heap.");
    println!("\ttiny value in x is {}", x.tiny);

    println!("\n- Get tiny's value by borrowing x");
    // Rust's Borrow operator, &, _is not_ a reference operator (e.g., C++)
    println!("\tvalue returned is {}", get_tiny_by_borrowing(&x));

    println!("\n- Back in main, x still owns the object b/c the function only borrowed it");
    println!("\ttiny value in x is {}", x.tiny);

    println!("\n- Get tiny's value by stealing x");
    println!("\tvalue returned is {}", get_tiny_by_stealing(x));

    println!(
        "\n- Back in main, x is no longer valid & will cause an compilation error. \
         The shows why, generally, functions should borrow from callers."
    );
    // NOTE: uncomment this line to see the compilation error, E0382: value borrowed here after move
    // println!("\ttiny value in x is {}", x.tiny);
}

// This function uses the Borrow operator to require callers to give it temporary ownership
fn get_tiny_by_borrowing(s: &SomeThing) -> i8 {
    return s.tiny;
}

// This function does not use the Borrow operator which results in it, in effect, stealing the object.
//  When a caller passes a SomeThing to this function, it takes ownership of the object. Once this function
//  completes, the object no longer has an owner, and will be disposed.
fn get_tiny_by_stealing(s: SomeThing) -> i8 {
    return s.tiny;
}
