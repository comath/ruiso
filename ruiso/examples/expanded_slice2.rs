use ruiso::*;

#[derive(Debug)]
pub struct SimpleTestStruct {
    foo: u32,
    bar: f32,
}

struct SimpleTestStructFeaturizer {}

impl Featurizer<SimpleTestStruct> for SimpleTestStructFeaturizer {
    fn dim() -> usize {
        2
    }
    fn fill_slice(data: &SimpleTestStruct, slice: &mut [f32]) {
        data.foo.fill_slice(&mut slice[0usize..1usize]);
        data.bar.fill_slice(&mut slice[1usize..2usize]);
    }
    fn default(_slice: &mut [f32]) {}
}

fn main() {
    let st = SimpleTestStruct { foo: 2, bar: 3.0 };
    let mut data: [f32; 2] = [0.0; 2];
    SimpleTestStructFeaturizer::fill_slice(&st, &mut data);
    println!("st {:?}, data {:?}", st, data);
    assert!(data[0] == 2.0);
    assert!(data[1] == 3.0);
}
