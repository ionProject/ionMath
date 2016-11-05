/*===============================================================================================*/
// Copyright 2016 Kyle Finlay
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
/*===============================================================================================*/

// Crate imports
extern crate num_traits;

// Module imports
use self::num_traits::{Num, NumCast};

use ::matrix::MatTrait;
use ::vector::Vec4;

use std::convert::From;
use std::ops::{AddAssign, Index, IndexMut, Mul};

/*===============================================================================================*/
/*------MAT3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic `Mat4` struct.
///
/// It is used for manipulating objects in 3d space.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Mat4<T> where
    T: Copy + Num + NumCast {

    // Private
    array: [Vec4<T>; 4]
}

// Predefined Mat4 types
/// `Mat4<f32>`
pub type Mat4f = Mat4<f32>;
/// `Mat4<i32>`
pub type Mat4i = Mat4<i32>;
/// `Mat4<u32>`
pub type Mat4u = Mat4<u32>;

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mat4<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Mat4` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat4;
    /// let mat = Mat4::<f32>::new (1,  2,  3,  4,
    ///                             5,  6,  7,  8,
    ///                             9,  10, 11, 12,
    ///                             13, 14, 15, 16);
    /// ```
    pub fn new<C> (m11: C, m12: C, m13: C, m14: C,
                   m21: C, m22: C, m23: C, m24: C,
                   m31: C, m32: C, m33: C, m34: C,
                   m41: C, m42: C, m43: C, m44: C,) -> Mat4<T> where
        C: Copy + Num + NumCast {

        Mat4 {

            array: [Vec4::new (m11, m12, m13, m14),
                    Vec4::new (m21, m22, m23, m24),
                    Vec4::new (m31, m32, m33, m34),
                    Vec4::new (m41, m42, m43, m44)]}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T, C> From<C> for Mat4<T> where
    T: Copy + Num + NumCast,
    C: Copy + Num + NumCast {

    fn from (value: C) -> Mat4<T> {

        Mat4::new (value, value, value, value,
                   value, value, value, value,
                   value, value, value, value,
                   value, value, value, value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, C> From<&'a Vec4<C>> for Mat4<T> where
    T: Copy + Num + NumCast,
    C: Copy + Num + NumCast {

    fn from (value: &Vec4<C>) -> Mat4<T> {

        Mat4::new (value.x, value.y, value.z, value.w,
                   value.x, value.y, value.z, value.w,
                   value.x, value.y, value.z, value.w,
                   value.x, value.y, value.z, value.w)
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mul for Mat4<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat4<T>;

    fn mul (self, rhs: Mat4<T>) -> Mat4<T> {

        let mut m = Mat4::from (0);

        for row in 0..4 {

            for col in 0..4 {

                for inner in 0..4 {
                    m[row][col] += self[row][inner] * rhs[inner][col]
                }
            }
        }

        m
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<&'a Mat4<T>> for Mat4<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat4<T>;

    fn mul (self, rhs: &Mat4<T>) -> Mat4<T> {

        let mut m = Mat4::from (0);

        for row in 0..4 {

            for col in 0..4 {

                for inner in 0..4 {
                    m[row][col] += self[row][inner] * rhs[inner][col]
                }
            }
        }

        m
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<Mat4<T>> for &'a Mat4<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat4<T>;

    fn mul (self, rhs: Mat4<T>) -> Mat4<T> {

        let mut m = Mat4::from (0);

        for row in 0..4 {

            for col in 0..4 {

                for inner in 0..4 {
                    m[row][col] += self[row][inner] * rhs[inner][col]
                }
            }
        }

        m
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Mul<&'a Mat4<T>> for &'a Mat4<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat4<T>;

    fn mul (self, rhs: &Mat4<T>) -> Mat4<T> {

        let mut m = Mat4::from (0);

        for row in 0..4 {

            for col in 0..4 {

                for inner in 0..4 {
                    m[row][col] += self[row][inner] * rhs[inner][col]
                }
            }
        }

        m
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Index<u8> for Mat4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn index (&self, index: u8) -> &Vec4<T> {

        match index {

            0 => &self.array[0],
            1 => &self.array[1],
            2 => &self.array[2],
            3 => &self.array[3],
            _ => unreachable! ("Index out of range for Mat4")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> IndexMut<u8> for Mat4<T> where
    T: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut Vec4<T> {

        match index {

            0 => &mut self.array[0],
            1 => &mut self.array[1],
            2 => &mut self.array[2],
            3 => &mut self.array[3],
            _ => unreachable! ("Index out of range for Mat4")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> MatTrait for Mat4<T> where
    T: Copy + Default + Num + NumCast {

    /// Returns a new identity matrix.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::{Mat4, MatTrait};
    /// let mat = Mat4::<f32>::identity ();
    /// ```
    fn identity () -> Mat4<T> {

        Mat4::new (1, 0, 0, 0,
                   0, 1, 0, 0,
                   0, 0, 1, 0,
                   0, 0, 0, 1)
    }
}
