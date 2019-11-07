use ruiso::*;

#[derive(StructFeature)]
pub struct SimpleTestStruct {
    foo: u32,
    bar: f32,
}

#[derive(StructFeature)]
pub struct TestStruct {
    foo: u32,
    #[struct_feature(dim = 13)]
    kal: String,
    bar: Option<f32>,
}

#[derive(StructFeature)]
pub struct DefaultTestStruct {
    foo: u32,
    kal: String,
    #[struct_feature(default = 5.0)]
    bar: Option<f32>,
}

pub struct WordHasher20 {}
impl Featurizer<String> for WordHasher20 {
    #[inline]
    fn dim() -> usize {
        20
    }
    #[inline]
    fn fill_slice(data: &String, slice: &mut [f32]) {
        for s in data.split(" ") {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            s.hash(&mut hasher);
            let result = (hasher.finish() as usize) % Self::dim();
            slice[result] += 1.0;
        }
    }
    fn default(_slice: &mut [f32]) {}
}

#[derive(StructFeature)]
pub struct FeaturizerTestStruct {
    foo: u32,
    #[struct_feature(featurizer = "WordHasher20")]
    kal: String,
    #[struct_feature(default = 5.0)]
    bar: Option<f32>,
}

#[derive(StructFeature)]
pub struct OffTestStruct {
    foo: u32,
    #[struct_feature(off)]
    kal: String,
    #[struct_feature(default = 5.0)]
    bar: Option<f32>,
}

#[derive(EnumFeature)]
pub enum Animals {
    Cat,
    Dog,
    Squirrel,
    Eldritch,
}

#[derive(StructFeature)]
pub struct EnumTestStruct {
    foo: u32,
    #[struct_feature(off)]
    kal: String,
    #[struct_feature(default = 5.0)]
    bar: Option<f32>,
    #[struct_feature(featurizer = "AnimalsFeaturizer4")]
    ell: Animals,
}

#[derive(StructFeature)]
pub struct OptionStringTestStruct {
    #[struct_feature(dim = 23)]
    zin: Option<String>,
}

#[derive(StructFeature)]
pub struct VecStringTestStruct {
    #[struct_feature(dim = 23)]
    zin: Option<Vec<String>>,
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_dimension_correct() {
        let dim = SimpleTestStruct::dim();
        assert!(dim == 2);
    }

    #[test]
    fn simple_fill_correct() {
        let st = SimpleTestStruct { foo: 2, bar: 3.0 };
        let mut data: [f32; 2] = [0.0; 2];
        st.fill_slice(&mut data);
        assert!(data[0] == 2.0);
        assert!(data[1] == 3.0);
    }

    #[test]
    fn fill_s0me_correct() {
        let st = TestStruct {
            foo: 2,
            bar: Some(3.0),
            kal: "hah".to_string(),
        };
        let mut data: [f32; 13 + 2] = [0.0; 13 + 2];
        st.fill_slice(&mut data);
        println!("{:?}", &data);
        assert!(data[0] == 2.0);
        assert!(data[13 + 1] == 3.0);
    }

    #[test]
    fn fill_none_correct() {
        let st = TestStruct {
            foo: 2,
            bar: None,
            kal: "hah".to_string(),
        };
        let mut data: [f32; 15 + 2] = [0.0; 15 + 2];
        st.fill_slice(&mut data);
        assert!(data[0] == 2.0);
        assert!(data[15 + 1] == 0.0);
    }

    #[test]
    fn fill_none_correct_default() {
        let st = DefaultTestStruct {
            foo: 2,
            bar: None,
            kal: "hah".to_string(),
        };
        let mut data: [f32; 39] = [0.0; 39];
        st.fill_slice(&mut data);
        assert!(data[0] == 2.0);
        assert!(data[38] == 5.0);
    }

    #[test]
    fn fill_correct_off() {
        let st = OffTestStruct {
            foo: 2,
            bar: None,
            kal: "hah".to_string(),
        };
        let mut data: [f32; 2] = [0.0; 2];
        st.fill_slice(&mut data);
        assert!(data[0] == 2.0);
        assert!(data[1] == 5.0);
    }

    #[test]
    fn fill_correct_enum() {
        let st = EnumTestStruct {
            foo: 2,
            bar: None,
            kal: "hah".to_string(),
            ell: Animals::Dog,
        };
        let mut data: [f32; 6] = [0.0; 6];
        st.fill_slice(&mut data);
        assert!(data[0] == 2.0);
        assert!(data[1] == 5.0);
        assert!(data[3] == 1.0);
    }

    #[test]
    fn fill_correct_vec() {
        let st = VecStringTestStruct {
            zin: Some(vec!["cook".to_string(), "cat".to_string()]),
        };
        let mut data: [f32; 23] = [0.0; 23];
        st.fill_slice(&mut data);
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        "cook".hash(&mut hasher);
        let result = (hasher.finish() as usize) % 23;
        assert!(data[result] == 1.0);
    }
}
