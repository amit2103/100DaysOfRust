fn main() {
    let p1 = Point::new(1, 2);
    let mut p2 = Point::new(0,1);
    println!("Distance between P1 and P2 is {} ",p1.distance(&p2));
    let mirror = p1.mirror_y();
    println!("X {} , Y {} ", mirror.x, mirror.y);

    p2.translate(1, 3);
    println!("Translated X for P2 {} and Y is {} ", p2.x, p2.y);
    
}

struct Point {
    x : i32,
    y : i32
}

// struct methods

impl Point {
    fn distance(&self, other: &Point) -> f32 {
        let (dx, dy) = (self.x - other.x, self.y - other.y);
        ((dx.pow(2) + dy.pow(2)) as f32).sqrt()
    }

    fn translate(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    fn mirror_y(self) -> Point {
        Point { x: -self.x, y: self.y }
    }
}

//Associaed Functions - Kind of like Static functions in Java

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}
