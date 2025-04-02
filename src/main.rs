use std::ptr;

static mut POINTS: i32 = 10;

union Win {
    bronze: i32,
    silver: i32,
    gold: i32,
}

enum Win2 {
    bronze(i32),
    silver(i32),
    gold(i32),
}

unsafe fn wraps_unsafe_code() {
    let a = 10;
    let ptr: *const i32 = &a;
    println!("the value of a is {}", *ptr);
}

fn main() {

    // ********* SCENARIO 1- accessing raw pointers  **********

    let a = 10;
    let ptr: *const i32 = &a;

    // following is unsafe and will not compile
    // println!("the value of a is {}", *ptr);

    // its workaround for safe usage
    unsafe {
        println!("SCENARIO 1: the value of a is {}", *ptr);
    }

    // *********  SCENARIO 2- accessing boxed pointers **********

    let b = Box::new(10);
    let ptr_b: *const i32 = &*b;

    // following is unsafe and will not compile
    // println!("SCENARIO 2: the value of b is {}", *ptr_b);

    // its workaround for safe usage
    unsafe {
        println!("SCENARIO 2: the value of b is {}", *ptr_b);
    }

    // ********* SCENARIO 3- make very sure that the pointer is not null before dereferencing it
    
    let ptr_c: *const i32 = ptr::null();

    // following will result in: zsh: segmentation fault. 
    // Responsibility to make sure it isnt null is on the developer, by maybe using if block
    /*
    unsafe {
        println!("the value of ptrC is {}", *ptr_c);
    }
    */
    // its workaround for safe usage
    unsafe {
        if !ptr_c.is_null() {
            println!("SCENARIO 3: the value of ptrC is {}", *ptr_c);
        } else {
            println!("SCENARIO 3: ptrC is null");
        }
    }

    // *********  SCENARIO 4 - accessing unsafe code within functions **********
    // following function call will give unsafe error
    // wraps_unsafe_code();

    // its workaround for safe usage
    println!("SCENARIO 4: before wraps_unsafe_code() functionc all:");
    unsafe {
        wraps_unsafe_code();
    }
    println!("SCENARIO 4: after wraps_unsafe_code() functionc all:");

    // *********  SCENARIO 5 - accessing mutable static variables **********
    // following will give error stating: 
    // use of mutable static is unsafe and requires unsafe block
    // println!("the value of POINTS is {}", POINTS);

    // its workaround for safe usage
    println!("SCENARIO 5: the value of POINTS is {}", unsafe { POINTS });
    // following is supposed to update static contant, but doesnt work even though its in unsafe block
    /*
    unsafe {
        POINTS += 10;
        println!("the value of POINTS incremented is {}", &POINTS);
    }
    */

    // *********  SCENARIO 6 - accessing unions **********
    // following will give unsafe error
    let win = Win { bronze: 10 };
    // println!("the value of win is {}", win.bronze);

    // its workaround for safe usage
    unsafe {
        println!("SCENARIO 6: the value of win is {}", win.bronze);
    }

    // *********  SCENARIO 7 - accessing enums **********
    let win2 = Win2::bronze(10);
    // enums have a way of using pattern matching to access the value,
    // and can be used without declaring unsafe block
    if let Win2::bronze(value) = win2 {
        println!("SCENARIO 7: the value of win is {}", value);
    } else {
        println!("SCENARIO 7: not a bronze");
    }
}
