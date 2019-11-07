#![feature(const_generics)]

use std::marker::PhantomData;

pub trait Featurizer<T,const DIM: usize> {
    const DIM: usize = DIM;
    fn fill_slice(raw_data:&T, slice: &mut [f32]);
    fn default(slice: &mut [f32]);
}

macro_rules! make_featurizable {
    ($native_ty:ty,$native_ty:ty) => {
        pub struct featurizer_$native_ty<const DIM: usize> {}
        impl Featurizer<1> for featurizer_$native_ty,const DIM: usize> {
			#[inline]
            fn fill_slice(raw_data:&$native_ty, slice: &mut [f32]) {
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

pub struct featurizer<bool,const DIM: usize> {}
impl Featurizable<1> for bool {
	#[inline]
    fn fill_slice(raw_data:&bool, slice: &mut [f32]) {
    	if *self {
	    	slice[0] = 1.0;
    	}
    }
    fn default(_slice: &mut [f32]) {}
}