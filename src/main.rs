fn main() {
    println!("Hello, world!");
}

pub trait Convert {
    type Output;
    fn convert(self) -> Self::Output;
}