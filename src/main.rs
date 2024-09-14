fn main() {
    /*
    Scalar Typess
    */

    // integers
    let x = 5; // default i32

    // floating-point
    let y = 2.0; // default f64
    let z: f32 = 3.0; // modern cpu's are optimized for f64, so f32 is rarely used

    /*
    Numeric Operations
    */

    // addition
    let sum = x + 1;

    // subtraction
    let difference = x - 1;

    // multiplication
    let product = x * 2;

    // division
    let quotient = x / 2;
    let truncated_quotient = x / 2; // results in 2

    // remainder
    let remainder = x % 2;

    /*
    Boolean Type
    */

    let t = true;
    let f: bool = false;

    /*
    Character Type
    */

    let c = 'z';
    let z: char = 'ℤ'; // explicit annotation
    let emoji = '😻';

    /*
    Compound Types
    */

    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring
    let (a, b, c) = tup;

    // Accessing elements
    let five_hundred = tup.0;

    // Array
    let arr = [1, 2, 3, 4, 5];

    // Accessing elements
    let first = arr[0];
    let second = arr[1];

    // Accessing elements with index out of bounds
    // let index = 10;
    // let element = arr[index]; // this will panic

    // Array with type annotation and length
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
}
