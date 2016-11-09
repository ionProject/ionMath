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
use ::vector::Vec3;

use std::convert::From;
use std::ops::{AddAssign, Index, IndexMut, Mul};

/*===============================================================================================*/
/*------MAT3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic `Mat3` struct.
///
/// It is used for manipulating objects in 3d space.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Mat3<T> where
    T: Copy + Num + NumCast {

    // Private
    array: [Vec3<T>; 3]
}

// Predefined Mat3 types
/// `Mat3<f32>`
pub type Mat3f = Mat3<f32>;
/// `Mat3<i32>`
pub type Mat3i = Mat3<i32>;
/// `Mat3<u32>`
pub type Mat3u = Mat3<u32>;

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mat3<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Mat3` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::Mat3;
    /// let mat = Mat3::<f32>::new (1, 2, 3,
    ///                             4, 5, 6,
    ///                             7, 8, 9);
    /// ```
    #[cfg_attr (feature = "cargo-clippy", allow (too_many_arguments))]
    pub fn new<C>(m11: C, m12: C, m13: C,
                  m21: C, m22: C, m23: C,
                  m31: C, m32: C, m33: C) -> Mat3<T> where
        C: Copy + Num + NumCast {

        Mat3 {

            array: [Vec3::new (m11, m12, m13),
                    Vec3::new (m21, m22, m23),
                    Vec3::new (m31, m32, m33)]
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T, C> From<C> for Mat3<T> where
    T: Copy + Num + NumCast,
    C: Copy + Num + NumCast {

    fn from (value: C) -> Mat3<T> {

        Mat3::new (value, value, value,
                   value, value, value,
                   value, value, value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, C> From<&'a Vec3<C>> for Mat3<T> where
    T: Copy + Num + NumCast,
    C: Copy + Num + NumCast {

    fn from (value: &Vec3<C>) -> Mat3<T> {

        Mat3::new (value.x, value.y, value.z,
                   value.x, value.y, value.z,
                   value.x, value.y, value.z)
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Mul for Mat3<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat3<T>;

    #[cfg_attr (feature = "cargo-clippy", allow (needless_range_loop))]
    fn mul (self, rhs: Mat3<T>) -> Mat3<T> {

        let mut m = Mat3::from (0);

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

impl<'a, T> Mul<&'a Mat3<T>> for Mat3<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat3<T>;

    #[cfg_attr (feature = "cargo-clippy", allow (needless_range_loop))]
    fn mul (self, rhs: &Mat3<T>) -> Mat3<T> {

        let mut m = Mat3::from (0);

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

impl<'a, T> Mul<Mat3<T>> for &'a Mat3<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat3<T>;

    #[cfg_attr (feature = "cargo-clippy", allow (needless_range_loop))]
    fn mul (self, rhs: Mat3<T>) -> Mat3<T> {

        let mut m = Mat3::from (0);

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

impl<'a, 'b, T> Mul<&'a Mat3<T>> for &'a Mat3<T> where
    T: AddAssign + Copy + Num + NumCast {

    type Output = Mat3<T>;

    #[cfg_attr (feature = "cargo-clippy", allow (needless_range_loop))]
    fn mul (self, rhs: &Mat3<T>) -> Mat3<T> {

        let mut m = Mat3::from (0);

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

impl<T> Index<u8> for Mat3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn index (&self, index: u8) -> &Vec3<T> {

        match index {

            0 => &self.array[0],
            1 => &self.array[1],
            2 => &self.array[2],
            _ => unreachable! ("Index out of range for Mat3")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> IndexMut<u8> for Mat3<T> where
    T: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut Vec3<T> {

        match index {

            0 => &mut self.array[0],
            1 => &mut self.array[1],
            2 => &mut self.array[2],
            _ => unreachable! ("Index out of range for Mat3")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> MatTrait for Mat3<T> where
    T: Copy + Default + Num + NumCast {

    /// Returns a new identity matrix.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::matrix::{Mat3, MatTrait};
    /// let mat = Mat3::<f32>::identity ();
    /// ```
    fn identity () -> Mat3<T> {

        Mat3::new (1, 0, 0,
                   0, 1, 0,
                   0, 0, 1)
    }
}
