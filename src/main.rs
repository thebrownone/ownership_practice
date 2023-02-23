fn main() {
let mut x: Box<i32> = Box::new(1);
let a: i32 = *x;         // *x reads the heap value, so a = 1
*x += 1;                 // *x on the left-side modifies the heap value, 
                         //     so x points to the value 2

let r1: &Box<i32> = &x;  // r1 points to x on the stack
let b: i32 = **r1;       // two dereferences get us to the heap value

let r2: &i32 = &*x;      // r2 points to the heap value directly
let mut c: i32 = *r2;    // so only one dereference is needed to read it
c += 1;                 // *r2 on the left-side modifies the heap value, 
                        //     so x points to the value 2

// use the variables to avoid warnings
println!("{} {} {}", a, b, c);
implicit();
mutable_reference();
permission_checks();
borrow_checker();
mutable_ref();
}

// a dereference operator (*) is used to read the value of a pointer 
// and to modify the value of a mutable pointer

// a reference operator (&) is used to create a pointer to a value
// &*x is a reference to the value pointed to by x and *x is the value 


//Rust implicitly dereferences and references as needed in certain situations 
fn implicit()  {
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);
    
    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);
    
    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn mutable_reference() {
    let mut x: Box<i32> = Box::new(1);
    let r: &mut Box<i32> = &mut x; //r is a mutable reference to x
    **r += 1; // two dereferences to modify the heap value
    assert_eq!(**r, 2);
}

// Read, write and own permissions for variables

fn permission_checks() {
    let x: Box<i32> = Box::new(1);
    let y: &Box<i32> = &x;
    let mut z  = **y;
    z += 1;
    println!("{}", z)
}

fn borrow_checker() {
    let x = String::from("Hello");
    let y: &String = &x;
    // x.push_str(" world"); // error: cannot borrow `x` as mutable because it is also borrowed as immutable
    println!("{} and {}", x, y);
    // If Rust allowed this program to compile, we would violate memory safety. 
    //The operation x.push_str(...) could internally cause x to resize, 
    //reallocating its memory to a different heap location. 
    //This reallocation would invalidate the reference y, 
    //so reading it in the println is undefined behavior.
}

fn mutable_ref() {
    let mut x = String::from("Hello");
    let y: &mut String = &mut x; // y is a mutable reference to x
    y.push_str(" world"); 
    println!("{}", x);
}
// mutable reference allow mutation but prevent aliasing
// in the above example, y is a mutable reference to x which removes read permission from x
// and gives write permission to *y
// but y does not have write permission because it refers to the mutable reference itself

