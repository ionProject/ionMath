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

use ::vector::Vec4;

use std::convert::From;
use std::ops::{AddAssign, Index, IndexMut, Mul};

/*===============================================================================================*/
/*------MAT4 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic `Mat4` struct.
///
/// It is used for manipulating objects in 3d space.
#[derive (Copy, Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
/*------Mat4 TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> From<T> for Mat4<T> where
    T: Copy + Num + NumCast {

    fn from (value: T) -> Self {

        Mat4::new (value, value, value, value,
                   value, value, value, value,
                   value, value, value, value,
                   value, value, value, value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> From<[Vec4<T>; 4]> for Mat4<T> where
    T: Copy + Num + NumCast {

    fn from (value: [Vec4<T>; 4]) -> Self {

        Mat4 {array: [value[0],
                      value[1],
                      value[2],
                      value[3]]}
    }
}

/*===============================================================================================*/
/*------MAT4 OPERATORS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mul for Mat4<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Self;

    fn mul (self, rhs: Self) -> Self::Output {

        let mut m = Mat4::zero ();

        for row in 0..3 {

            for col in 0..3 {

                for inner in 0..3 {
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

    fn index (&self, index: u8) -> &Self::Output {

        match index {

            0 => &self.array[0],
            1 => &self.array[1],
            2 => &self.array[2],
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
            _ => unreachable! ("Index out of range for Mat4")
        }
    }
}

/*===============================================================================================*/
/*------MAT4 PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mat4<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Mat4` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat4;
    /// let mat = Mat4::new (1,  2,  3,  4,
    ///                      5,  6,  7,  8,
    ///                      9,  10, 11, 12,
    ///                      13, 14, 15, 16);
    /// ```
    pub fn new (m11: T, m12: T, m13: T, m14: T,
                m21: T, m22: T, m23: T, m24: T,
                m31: T, m32: T, m33: T, m34: T,
                m41: T, m42: T, m43: T, m44: T,) -> Self {

        Mat4 {array: [Vec4::new (m11, m12, m13, m14),
                      Vec4::new (m21, m22, m23, m24),
                      Vec4::new (m31, m32, m33, m34),
                      Vec4::new (m41, m42, m43, m44)]}
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a new identity matrix.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat4;
    /// let mat = Mat4::<f32>::identity ();
    /// ```
    pub fn identity () -> Self {

        Mat4::new (T::one  (), T::zero (), T::zero (), T::zero (),
                   T::zero (), T::one  (), T::zero (), T::zero (),
                   T::zero (), T::zero (), T::one  (), T::zero (),
                   T::zero (), T::zero (), T::zero (), T::one  ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a matrix with all zeros.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat4;
    /// let mat = Mat4::<f32>::zero ();
    /// ```
    pub fn zero () -> Self {

        Mat4::from (T::zero ())
    }
}
