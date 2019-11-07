use ruiso::{Featurizable, StructFeature};

#[derive(StructFeature)]
pub struct SimpleTestStruct {
    foo: u32,
    bar: f32,
}

fn main() {
    let st = SimpleTestStruct { foo: 2, bar: 3.0 };
    let mut data: [f32; 2] = [0.0; 2];
    st.fill_slice(&mut data);
    assert!(data[0] == 2.0);
    assert!(data[1] == 3.0);
}
