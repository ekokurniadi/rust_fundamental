fn main() {
    let mut b = 20; // mutable
    
    const HIGHEST_PRICE:u32 = 10000000; // u can write it with readable ex : 10_000_000

    let x = 10;
    // shadowing variable x
    let x ="ten";

    // scalar types : integer, float, bool, characters
    let i :u32 = 10;
    let f:f32 = 2.1;
    let bool_true  = true;
    let bool_false = false;
    let c = 'Z'; // jika menggunakan single quote maka akan bertipe char
    let s = "Huruf"; // jika menggunakan double quote maka akan bertipe string


    // compound : tuples and arrays
    let tup :(i32,f64,u8) =(100,1.3,1);
    // destructuring
    let (x,y,z) = tup;
    // get data by index
    let first = tup.0;

    let tup2 :() =();


    let a :[i32;4] =[1,2,3,4];
    let b = [3,5];

    my_function(10,'h');
    let my_returned_value_fn = returned_value_fn(10);
    println!("{my_returned_value_fn}");
}

// method with argument
fn my_function(value:i32,label:char){   
    println!("my function with argument {} {}",value,label);
}

fn returned_value_fn(x:i32)->i32{
    x
}