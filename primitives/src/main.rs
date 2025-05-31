fn main() {
    
    //Variables can be typed annoted
    let logical:bool = true;

    let a_float: f64 = 1.0;
    let an_integer = 5i32;


    //Or  a default will be used
    let default_float = 3.0;
    let default_integer = 5i32;

    //or a default will be used
    let default_float = 3.0;
    let default_integer = 7;

    // A type can also be inferred from context
    let mut inferred_type = 12;
    inferred_type = 4294967295i64; // error: `inferred_type` is not of type `i64`

    // A mutable variable's value can be changed
    let mut mutable_integer = 12;
    mutable_integer = 21; // This is fine
    
    // Array signature consist of Type T and length as [T; length]

    let my_array: [i32; 5] = [1, 2, 3, 4, 5];

    //Tuple is a collection of values of different types
    //and  is constructed using parentheses
    let my_tuple: (i32, f64, u8) = (500, 6.4, 1);  

}
