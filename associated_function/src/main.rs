fn main() {
   let my_rect = Rectangle::new(10, 10);
   let my_rect2 = Rectangle::new(12, 12);
   
    println!("My Rectangle area is {}",my_rect.area());
    println!("My Rectangle width is non zero {}",my_rect.width());
    println!("My Rectangle 2 area is {}",my_rect2.area());
    println!("My Rectangle 2 width is non zero {}",my_rect2.width());
    
    println!("My Rectangle 1 can hold My Rectangle 2 {}",my_rect.can_hold(&my_rect2));

}


struct Rectangle{
    width:u32,
    heigth:u32,
}


impl Rectangle{
    fn new(width:u32,heigth:u32)->Self{
        Self { width, heigth}
    }
    fn area(&self)->u32{
        self.width * self.heigth
    }

    fn width(&self)->bool{
        self.width >0
    }

    fn can_hold(&self,other: &Rectangle)->bool{
        self.width > other.width && self.heigth > other.heigth
    }
}
