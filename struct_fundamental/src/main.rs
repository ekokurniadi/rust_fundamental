fn main(){
    let my_rect = Rectangle{
        width:10,
        height:5,
    };

    dbg!("{:#?}",&my_rect);

    println!("area of my rect is {}",get_area(&my_rect));
}

#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn get_area(rec:&Rectangle)->u32{
    rec.width * rec.height
}