#![feature(const_generics)]

pub trait Featurizable<const DIM: usize> {
    const DIM: usize = DIM;
    fn fill_slice(&self, slice: &mut [f32]);
    fn default(slice: &mut [f32]);
}

macro_rules! make_featurizable {
    ($native_ty:ty) => {
        impl Featurizable<1> for $native_ty {
			#[inline]
            fn fill_slice(&self, slice: &mut [f32]) {
		    	slice[0] = *self as f32;
		    }
		    fn default(_slice: &mut [f32]) {}
        }
    };
}

make_featurizable!(f32);
make_featurizable!(f64);
make_featurizable!(u8);
make_featurizable!(u16);
make_featurizable!(u32);
make_featurizable!(u64);
make_featurizable!(i8);
make_featurizable!(i16);
make_featurizable!(i32);
make_featurizable!(i64);
make_featurizable!(usize);

impl Featurizable<1> for bool {
	#[inline]
    fn fill_slice(&self, slice: &mut [f32]) {
    	if *self {
	    	slice[0] = 1.0;
    	}
    }
    fn default(_slice: &mut [f32]) {}
}