fn main() {
    println!("Hello, world!");

    // NOTE : Default dari reference adalah immutable / tidak bisa di rubah

    let s1 = String::from("hello world");
    let len = calculate_length(&s1); // reference from s1 (reference borrowing)
    println!("the length of {} is {}",s1,len);

    // mengubah isi reference
    let mut s = String::from("hello");
    change_value_s(&mut s); // create mutable reference


    // mix reference mutable dan immutable
    let mut t = String::from("hello again");
    let t1 = &t;
    let t2=&t;
    // let t3 = &mut t; akan terjadi error karena tidak dibolehkan merubah reference saat masih digunakan
    println!("{},{}",t1,t2);

    let t3 = &mut t; // boleh digunakan karena mutable reference sudah selesai dijalankan
    println!("{}",t3);

    // dangle pointer
    // let reference_to_dangle = dangle();

    // slice
    let mut sample = String::from("hello world");
    let first_word = returned_first_word(&sample);
    sample.clear();

    // first_word masih memiliki nilai
    println!("{}",first_word);


    let mut sample2  = String::from("hello world");
    // let hello  = &sample[..=5];
    // let world = &sample[6..];


    let firts_word2 = returned_first_word2(&sample2);

    println!("{}",firts_word2)

}

// hello world
fn returned_first_word(s: &String)->usize{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{ // b' ' merupakan byte literal character spasi
            return i;
        }
    }
    s.len()
}
// hello world
fn returned_first_word2(s: &String)->&str{
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{ // b' ' merupakan byte literal character spasi
            return &s[..i];
        }
    }
    &s[..]
}

// fn dangle()->&String{
//     let s = String::from("hello");
//     &s
// } // akan terjadi error karena variable s sudah di drop tetapi kita ingin mereturn reference dari variable s

fn change_value_s(some_string:&mut String){
    some_string.push_str(" world")
}

fn calculate_length(s:&String) ->usize{
    s.len()
}
