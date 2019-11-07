use ruiso::{EnumFeature, Featurizable, Featurizer};

#[derive(EnumFeature)]
pub enum ExampleEnum {
    Foo,
    Bar,
    Kal,
    Ell,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dimension_correct() {
        let dim = ExampleEnumFeaturizer4::dim();
        assert!(dim == 4);
    }

    #[test]
    fn fill_featurize_correct() {
        let st = ExampleEnum::Foo;
        let mut data: [f32; 4] = [0.0; 4];
        ExampleEnumFeaturizer4::fill_slice(&st, &mut data);
        assert!(data[0] == 1.0);
        assert!(data[1] == 0.0);
        assert!(data[2] == 0.0);
        assert!(data[3] == 0.0);
    }

    #[test]
    fn fill_featurizable_correct() {
        let st = ExampleEnum::Foo;
        let mut data: [f32; 4] = [0.0; 4];
        st.fill_slice(&mut data);
        assert!(data[0] == 1.0);
        assert!(data[1] == 0.0);
        assert!(data[2] == 0.0);
        assert!(data[3] == 0.0);
    }
}
