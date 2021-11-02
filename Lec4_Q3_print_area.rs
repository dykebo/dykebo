pub trait Area{
    fn calculate_area(&self) -> f32;
}


#[derive(Debug)]
pub struct Square{
    pub length: f32,
}
impl Area for Square {
    fn calculate_area(&self) -> f32{
        return self.length * self.length;
    }    
}

pub struct Triangle{
    pub base: f32,
    pub height: f32,
}
impl Area for Triangle {
    fn calculate_area(&self) -> f32{
        return (self.base * self.height) / 2.0;
    }    
}

pub struct Circle{
    pub radius: f32,
    pub pi: f32,
}
impl Area for Circle {
    fn calculate_area(&self) -> f32{
        return self.pi * self.radius * self.radius;
    }    
}



fn main() {
    
    let a_square = Square {
        length: 50.0,
    };
    let a_triangle = Triangle {
        base: 10.0,
        height: 3.0
    };
    let a_circle = Circle {
        radius: 1.0,
        pi: 3.1415926,
    };
    
    let ans=get_area(&a_square);
    println!("The area of the square is {}", ans);

    let ans=get_area(&a_triangle);
    println!("The area of the triangle is {}", ans);

    let ans=get_area(&a_circle);
    println!("The area of the circle is {}", ans);

}

pub fn get_area<T: Area>(geometry: &T) -> f32 {
   return geometry.calculate_area();
}