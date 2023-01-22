
// struct
struct User{
    active:bool,
    user_name:String,
    email:String,
    sing_in_count:u64,
}

fn main() {
    // membuat instance user1
    let mut user1 = User{
        email:String::from("email@gmail.com"),
        user_name:String::from("admin"),
        active:true,
        sing_in_count:1,
    };

    
    let user2 = User{
        email:String::from("user2@gmail.com"),
        user_name:String::from("user2"),
        ..user1 // struct update syntax
    };
    
    // modifikasi isi field email
    user1.email = String::from("email1@gmail.com");
    
    let user3 = build_user(String::from("hallo@gmail.com"), String::from("user"));
    println!("Hello, world! {}",user1.user_name);
    println!("Hello, world! {}",user2.user_name);
    println!("Hello, world! {}",user3.user_name);

}


fn build_user(email:String,username:String)->User{
    User{
        email, // kalau nama dari parameter dan field pada struct sama langsung di masukkan saja
        user_name:username,
        active:true,
        sing_in_count:1
    }
}