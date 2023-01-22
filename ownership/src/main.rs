fn main() {
    // Membuat scope
    {
        let s ="hello";
        println!("{}",s);
    }

    let x = 5;
    let y =x; // copy trait [scalar type mempunyai annotation copy]
    println!("x: {} y: {}",x,y);
    
    let s1 = String::from("hallo");
    // let s2 = s1; // moved s1 ke variabel s2
    let s2 = s1.clone();
    println!("s1: {} s2: {}",s1,s2);


    // ownership dan function
    let s3 = String::from("ownership string");

    takes_ownership(s3); // move value s3 to takes_ownership function
    // println!("{}",s3);            // s3 no longer available karena sudah di move ke function

    let copy_int = 5;
    makes_copy(copy_int);

// ambil ownership
    let var1 = gives_ownership();
    println!("{}",var1);
    let var2 = String::from("hello");
    let var3 = take_and_give_back(var2);
    println!("{}",var3);

    // return tuple
    let var4 = String::from("jumlah huruf");
    let (string_huruf,length) = calculate_length(var4);
    println!("{} {}",string_huruf,length);

}


fn calculate_length(s:String)->(String,usize){
    let length = s.len();
    (s,length)
}


fn gives_ownership()->String{
    let some_str = String::from("world");
    some_str
}

fn take_and_give_back(some_str:String)->String{
    some_str
}


fn takes_ownership(some_string:String){
    println!("{}",some_string);
} // automatic call drop


fn makes_copy(some_integer :i32){
    println!("{}",some_integer);
}