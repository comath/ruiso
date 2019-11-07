//! # Featurization crate
//! 
//! Provides 2 macros for generating features from struct and enums, 
//! and a pair of traits for these to implement. Don't use these directly unless you want something custom.
//! 


//#[cfg(feature = "ruiso_derive")]
#![deny(warnings, missing_docs)]

#[allow(unused_imports)]
#[macro_use]
//#[cfg(feature = "ruiso_derive")]
extern crate ruiso_derive;
pub use ruiso_derive::*;
pub use std::hash::{Hash, Hasher};

/// # Featurizer
/// Implement this for a custom featurizer.
/// This must end in a the dimension, Bla34, and it needs to be a zero sized type as it's called but not created.
/// Prefereably this would use constant type constraints rather than this clunky naming scheme.
pub trait Featurizer<T> {
	/// This has to be the dimension of the type. This must be the integer at the end of the zero sized type.
    fn dim() -> usize;
    /// Fills the provided slice with the vectorized features
    fn fill_slice(data: &T, slice: &mut [f32]);
    /// If the struct that this is in has an option, and is None, this is called
    fn default(slice: &mut [f32]);
    /// Creates the vector, fills it and hands it back to you
    fn featurize(data: &T) -> Vec<f32> {
        let mut features: Vec<f32> = vec![0.0; Self::dim()];
        Self::fill_slice(data, &mut features);
        features
    }
}

/// # Featurizable
/// Implement this for a featurizer of your type.
pub trait Featurizable {
	/// This has to be the dimension of the type.
    fn dim() -> usize;
    /// Fills the provided slice with the vectorized features
    fn fill_slice(&self, slice: &mut [f32]);
    /// If the struct that this is in has an option, and is None, this is called
    fn default(slice: &mut [f32]);
    /// Creates the vector, fills it and hands it back to you
    fn featurize(&self) -> Vec<f32> {
        let mut features: Vec<f32> = vec![0.0; Self::dim()];
        self.fill_slice(&mut features);
        features
    }
}

macro_rules! make_featurizable {
    ($name:ident,$native_ty:ty) => {
    	/// Featurizer for $native_ty
        #[derive(Debug)]
        pub struct $name {}
        impl Featurizer<$native_ty> for $name {
            #[inline]
            fn dim() -> usize {
                1
            }
            #[inline]
            fn fill_slice(data: &$native_ty, slice: &mut [f32]) {
                slice[0] = *data as f32;
            }
            fn default(_slice: &mut [f32]) {}
        }
    	/// Featurizer for $native_ty
        impl Featurizable for $native_ty {
            #[inline]
            fn dim() -> usize {
                1
            }
            #[inline]
            fn fill_slice(&self, slice: &mut [f32]) {
                slice[0] = *self as f32;
            }
            fn default(_slice: &mut [f32]) {}
        }
    };
}

make_featurizable!(Defaultf32, f32);
make_featurizable!(Defaultf64, f64);
make_featurizable!(Defaultu8, u8);
make_featurizable!(Defaultu16, u16);
make_featurizable!(Defaultu32, u32);
make_featurizable!(Defaultu64, u64);
make_featurizable!(Defaulti8, i8);
make_featurizable!(Defaulti16, i16);
make_featurizable!(Defaulti32, i32);
make_featurizable!(Defaulti64, i64);
make_featurizable!(Defaultusize, usize);

/// Builds a hashing trick featurizer of the desired dimension and name.
pub struct Defaultbool1 {}
impl Featurizer<bool> for Defaultbool1 {
    #[inline]
    fn dim() -> usize {
        1
    }
    #[inline]
    fn fill_slice(data: &bool, slice: &mut [f32]) {
        if *data {
            slice[0] = 1.0;
        }
    }
    fn default(_slice: &mut [f32]) {}
}

/// Builds a hashing trick featurizer of the desired dimension and name.
#[macro_export]
macro_rules! make_string_feature {
    ($name:ident,$dim:expr) => {
        #[derive(Debug)]
        pub struct $name {}
        impl Featurizer<String> for $name {
            #[inline]
            fn dim() -> usize {
                $dim
            }
            #[inline]
            fn fill_slice(data: &String, slice: &mut [f32]) {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                data.hash(&mut hasher);
                let result = (hasher.finish() as usize) % $dim;
                slice[result] += 1.0;
            }
            fn default(_slice: &mut [f32]) {}
        }
    };
}

/// Builds a hashing trick featurizer of the desired dimension and name.
#[macro_export]
macro_rules! make_vec_string_feature {
    ($name:ident,$dim:expr) => {
        #[derive(Debug)]
        pub struct $name {}
        impl Featurizer<Vec<String>> for $name {
            #[inline]
            fn dim() -> usize {
                $dim
            }
            #[inline]
            fn fill_slice(data: &Vec<String>, slice: &mut [f32]) {
                for s in data.iter() {
                    let mut hasher = std::collections::hash_map::DefaultHasher::new();
                    s.hash(&mut hasher);
                    let result = (hasher.finish() as usize) % $dim;
                    slice[result] += 1.0;
                }
            }
            fn default(_slice: &mut [f32]) {}
        }
    };
}
