#![feature(const_generics)]

use ruiso::{Featurizable};

#[derive(Debug)]
pub struct SimpleTestStruct {
    foo: u32,
    bar: f32,
}
impl Featurizable<2usize> for SimpleTestStruct {
    fn fill_slice(&self, slice: &mut [f32]) {
        self.foo.fill_slice(&mut slice[0usize..1usize]);
        self.bar.fill_slice(&mut slice[1usize..2usize]);
    }
    fn default(_slice: &mut [f32]) {}
}
fn main() {
    let st = SimpleTestStruct { foo: 2, bar: 3.0 };
    let mut data: [f32; 2] = [0.0; 2];
    st.fill_slice(&mut data);
    println!("st {:?}, data {:?}", st,data);
    assert!(data[0] == 2.0);
    assert!(data[1] == 3.0);
}