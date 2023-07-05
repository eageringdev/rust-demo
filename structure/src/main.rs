enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("{}", Rectangle::square(5).area());
}
