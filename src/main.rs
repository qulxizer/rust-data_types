fn main() {
    data_types()
}

fn data_types() {
    //Integers
    /*  
    An integer stores number in this case we are using i32 
    it should hold up numbers till -2147483648 2147483647
    if you are confident the number will always be positive
    use u32 it will hold up till 4,294,967,295
    */ 
    let integer:i32 = 10;
    println!("{integer}");

    //Floats
    /* 
    Floating-Point stores numbers with a decimal points
    like 3.141592
    
     */
    const PI:f64 = 3.141592;
    println!("{PI}");

    //Numeric Operations

    //addition
    let _sum = 5 + 10;

    //subtraction
    let _difference = 95.5 - 4.3;

    //multiplication
    let _product = 4 * 30;

    //division
    let _quotient = 56.7 / 32.2;
    let _truncated = -5 / 3; // Results in -1

    //remainder
    let _remainder = 43 % 5;

    //Booleans

    let _bool:bool = true; 
    let _bool:bool = false; 

    //Char

    let _c = 'z';
    let _z: char = 'ℤ'; // with explicit type annotation
    let _heart_eyed_cat = '😻';

    //Tuples

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;

    //Arrays
    let _a = [1, 2, 3, 4, 5];

}