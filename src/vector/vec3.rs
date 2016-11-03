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
use self::num_traits::{Float, Num, NumCast};

use ::util;
use ::vector::{Vec2, Vec4, VecTrait, VecTraitF};

use std::convert::From;
use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};

/*===============================================================================================*/
/*------VEC3 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec3 struct.
///
/// It is used for 3D transformations and graphics.
/// It can accept any number as a value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3<T> where
    T: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: T,
    /// The vector y-coordinate.
    pub y: T,
    /// The vector z-coordinate.
    pub z: T,
}

// Predefined Vec3 types
/// `Vec3<f32>`
pub type Vec3f = Vec3<f32>;
/// `Vec3<i32>`
pub type Vec3i = Vec3<i32>;
/// `Vec3<u32>`
pub type Vec3u = Vec3<u32>;

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec3<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Vec3<T>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::new (3, 7, 10);
    /// ```
    pub fn new<C> (x: C, y: C, z: C) -> Vec3<T> where
        C: Copy + Num + NumCast {

        Vec3 {x: T::from (x).unwrap (),
              y: T::from (y).unwrap (),
              z: T::from (z).unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T, U> From<U> for Vec3<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: U) -> Vec3<T> {

        Vec3::new (T::from (value).unwrap (),
                   T::from (value).unwrap (),
                   T::from (value).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec2<U>> for Vec3<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec2<U>) -> Vec3<T> {

        Vec3::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap (),
                   T::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec3<U>> for Vec3<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec3<U>) -> Vec3<T> {

        Vec3::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap (),
                   T::from (value.z).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec4<U>> for Vec3<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec4<U>) -> Vec3<T> {

        Vec3::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap (),
                   T::from (value.z).unwrap ())
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Add for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<&'a Vec3<T>> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<Vec3<T>> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Add<&'a Vec3<T>> for &'b Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Add<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<T> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn add (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign for Vec3<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec3<T>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: T) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<&'a Vec3<T>> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<Vec3<T>> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Sub<&'a Vec3<T>> for &'b Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<T> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn sub (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign for Vec3<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec3<T>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: T) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<&'a Vec3<T>> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<Vec3<T>> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Mul<&'a Vec3<T>> for &'b Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<T> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn mul (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign for Vec3<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec3<T>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: T) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<&'a Vec3<T>> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<Vec3<T>> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Div<&'a Vec3<T>> for &'b Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<T> for &'a Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = Vec3<T>;

    fn div (self, rhs: T) -> Vec3<T> {

        Vec3::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign for Vec3<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec3<T>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign<T> for Vec3<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: T) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Index<u8> for Vec3<T> where
    T: Copy + Num + NumCast {

    type Output = T;

    fn index (&self, index: u8) -> &T {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> IndexMut<u8> for Vec3<T> where
    T: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut T {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => unreachable! ("Index out of range for Vec3")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> VecTrait for Vec3<T> where
    T: Copy + Default + Num + NumCast + PartialOrd {

    type ValType = T;

    fn lerp (start: &Vec3<T>, end: &Vec3<T>, percentage: f32) -> Vec3<T> {

        Vec3::new (util::lerp (start.x, end.x, percentage),
                   util::lerp (start.y, end.y, percentage),
                   util::lerp (start.z, end.z, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn max (lhs: &Vec3<T>, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (util::max (lhs.x, rhs.x),
                   util::max (lhs.y, rhs.y),
                   util::max (lhs.z, rhs.z))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: &Vec3<T>, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new (util::min (lhs.x, rhs.x),
                   util::min (lhs.y, rhs.y),
                   util::min (lhs.z, rhs.z))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn clamp (&self, min: &Vec3<T>, max: &Vec3<T>) -> Vec3<T> {

        Vec3::new (util::clamp (self.x, min.x, max.x),
                   util::clamp (self.y, min.y, max.y),
                   util::clamp (self.z, min.z, max.z))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn dot (&self, rhs: &Vec3<T>) -> T {

        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> VecTraitF for Vec3<T> where
    T: Default + Float {

    type ValTypeF = T;

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec01 = Vec3::<f32>::new (1.0, 3.0, 0.0);
    /// let vec02 = Vec3::<f32>::new (4.0, 9.0, 0.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    fn distance (&self, rhs: &Vec3<T>) -> T {
        (self - rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec = Vec3::<f32>::new (1.0, 3.0, 0.0);
    /// let vec_length = vec.length ();
    /// ```
    fn length (&self) -> T {

        (self.x * self.x +
         self.y * self.y +
         self.z * self.z).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec3, VecTraitF};
    /// let vec = Vec3::<f32>::new (3.0, 9.0, 0.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    fn normalize (&self) -> Vec3<T> {

        let length = self.length ();

        if length != T::zero () {

            return Vec3::new (self.x / length,
                              self.y / length,
                              self.z / length);
        }

        Vec3::zero ()
    }
}

/*===============================================================================================*/
/*------PUBLIC METHODS---------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec3<T> where
    T: Copy + Num + NumCast {

    /// Returns the cross product of two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec01 = Vec3::<f32>::new (1, 3, 6);
    /// let vec02 = Vec3::<f32>::new (4, 9, 2);
    ///
    /// let cross_product = vec01.cross (&vec02);
    /// ```
    pub fn cross (&self, rhs: &Vec3<T>) -> Vec3<T> {

        Vec3::new ((self.y * rhs.z) - (self.z * rhs.y),
                   (self.z * rhs.x) - (self.x * rhs.z),
                   (self.x * rhs.y) - (self.y * rhs.x))
    }

/*===============================================================================================*/
/*------PUBLIC STATIC METHODS--------------------------------------------------------------------*/
/*===============================================================================================*/

    /// Returns a `Vec3<V>` with a value of (0, 1, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::up ();
    /// ```
    pub fn up () -> Vec3<T> {

        Vec3::new (T::zero (),
                   T::one  (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (0, -1, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::down ();
    /// ```
    pub fn down () -> Vec3<T> {

        Vec3::new (T::zero (),
                   T::from (-1).unwrap (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (-1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::left ();
    /// ```
    pub fn left () -> Vec3<T> {

        Vec3::new (T::from (-1).unwrap (),
                   T::zero (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::right ();
    /// ```
    pub fn right () -> Vec3<T> {

        Vec3::new (T::one  (),
                   T::zero (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (0, 0, 1)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::forward ();
    /// ```
    pub fn forward () -> Vec3<T> {

        Vec3::new (T::zero (),
                   T::zero (),
                   T::one  ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (0, 0, -1)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::back ();
    /// ```
    pub fn back () -> Vec3<T> {

        Vec3::new (T::zero (),
                   T::zero (),
                   T::from (-1).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec3<T>` with a value of (0, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec3;
    /// let vec = Vec3::<f32>::zero ();
    /// ```
    pub fn zero () -> Vec3<T> {
        Vec3::from (T::zero ())
    }
}
