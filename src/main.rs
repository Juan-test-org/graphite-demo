fn main() {
    println!("Hello, world!");
}

pub trait Convert {
    type Output;
    fn convert(self) -> Self::Output;
    fn convert_back(output: Self::Output) -> Self;
}

impl Convert for i32 {
    type Output = f64;

    fn convert(self) -> Self::Output {
        self as f64
    }
}