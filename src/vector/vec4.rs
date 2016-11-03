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
use ::vector::{Vec2, Vec3, VecTrait, VecTraitF};

use std::convert::From;
use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};

/*===============================================================================================*/
/*------VEC4 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec4 struct.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec4<T> where
    T: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: T,
    /// The vector y-coordinate.
    pub y: T,
    /// The vector z-coordinate.
    pub z: T,
    /// The vector w-coordinate.
    pub w: T,
}

// Predefined Vec4 types
/// `Vec4<f32>`
pub type Vec4f = Vec4<f32>;
/// `Vec4<i32>`
pub type Vec4i = Vec4<i32>;
/// `Vec4<u32>`
pub type Vec4u = Vec4<u32>;

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec4<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Vec4<T>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::new (3, 7, 10, 9);
    /// ```
    pub fn new<C> (x: C, y: C, z: C, w: C) -> Vec4<T> where
        C: Copy + Num + NumCast {

        Vec4 {x: T::from (x).unwrap (),
              y: T::from (y).unwrap (),
              z: T::from (z).unwrap (),
              w: T::from (w).unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T, U> From<U> for Vec4<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: U) -> Vec4<T> {

        Vec4::new (value,
                   value,
                   value,
                   value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec2<U>> for Vec4<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec2<U>) -> Vec4<T> {

        Vec4::new (value.x,
                   value.y,
                   U::zero (),
                   U::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec3<U>> for Vec4<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec3<U>) -> Vec4<T> {

        Vec4::new (value.x,
                   value.y,
                   value.z,
                   U::zero ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec4<U>> for Vec4<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec4<U>) -> Vec4<T> {

        Vec4::new (value.x,
                   value.y,
                   value.z,
                   value.w)
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Add for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z,
                   self.w + rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<&'a Vec4<T>> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z,
                   self.w + rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<Vec4<T>> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z,
                   self.w + rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Add<&'a Vec4<T>> for &'b Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x + rhs.x,
                   self.y + rhs.y,
                   self.z + rhs.z,
                   self.w + rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Add<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs,
                   self.w + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<T> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn add (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x + rhs,
                   self.y + rhs,
                   self.z + rhs,
                   self.w + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign for Vec4<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec4<T>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: T) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
        self.w = self.w + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z,
                   self.w - rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<&'a Vec4<T>> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z,
                   self.w - rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<Vec4<T>> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z,
                   self.w - rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Sub<&'a Vec4<T>> for &'b Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x - rhs.x,
                   self.y - rhs.y,
                   self.z - rhs.z,
                   self.w - rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs,
                   self.w - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<T> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn sub (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x - rhs,
                   self.y - rhs,
                   self.z - rhs,
                   self.w - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign for Vec4<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec4<T>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
        self.z = self.z - rhs.z;
        self.w = self.w - rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: T) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
        self.z = self.z - rhs;
        self.w = self.w - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z,
                   self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<&'a Vec4<T>> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z,
                   self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<Vec4<T>> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z,
                   self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Mul<&'a Vec4<T>> for &'b Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x * rhs.x,
                   self.y * rhs.y,
                   self.z * rhs.z,
                   self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs,
                   self.w * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<T> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn mul (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x * rhs,
                   self.y * rhs,
                   self.z * rhs,
                   self.w * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign for Vec4<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec4<T>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
        self.w = self.w * rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: T) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
        self.w = self.w * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z,
                   self.w / rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<&'a Vec4<T>> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z,
                   self.w / rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<Vec4<T>> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z,
                   self.w / rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Div<&'a Vec4<T>> for &'b Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (self.x / rhs.x,
                   self.y / rhs.y,
                   self.z / rhs.z,
                   self.w / rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs,
                   self.w / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<T> for &'a Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = Vec4<T>;

    fn div (self, rhs: T) -> Vec4<T> {

        Vec4::new (self.x / rhs,
                   self.y / rhs,
                   self.z / rhs,
                   self.w / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign for Vec4<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec4<T>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
        self.z = self.z / rhs.z;
        self.w = self.w / rhs.w;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign<T> for Vec4<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: T) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
        self.w = self.w / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Index<u8> for Vec4<T> where
    T: Copy + Num + NumCast {

    type Output = T;

    fn index (&self, index: u8) -> &T {

        match index {

            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> IndexMut<u8> for Vec4<T> where
    T: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut T {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => unreachable! ("Index out of range for Vec4")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> VecTrait for Vec4<T> where
    T: Copy + Default + Num + NumCast + PartialOrd {

    type ValType = T;

    fn lerp (start: &Vec4<T>, end: &Vec4<T>, percentage: f32) -> Vec4<T> {

        Vec4::new (util::lerp (start.x, end.x, percentage),
                   util::lerp (start.y, end.y, percentage),
                   util::lerp (start.z, end.z, percentage),
                   util::lerp (start.w, end.w, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn max (lhs: &Vec4<T>, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (util::max (lhs.x, rhs.x),
                   util::max (lhs.y, rhs.y),
                   util::max (lhs.z, rhs.z),
                   util::max (lhs.w, rhs.w))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: &Vec4<T>, rhs: &Vec4<T>) -> Vec4<T> {

        Vec4::new (util::min (lhs.x, rhs.x),
                   util::min (lhs.y, rhs.y),
                   util::min (lhs.z, rhs.z),
                   util::min (lhs.w, rhs.w))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn clamp (&self, min: &Vec4<T>, max: &Vec4<T>) -> Vec4<T> {

        Vec4::new (util::clamp (self.x, min.x, max.x),
                   util::clamp (self.y, min.y, max.y),
                   util::clamp (self.z, min.z, max.z),
                   util::clamp (self.w, min.w, max.w))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn dot (&self, rhs: &Vec4<T>) -> T {

        (self.x * rhs.x) +
        (self.y * rhs.y) +
        (self.z * rhs.z) +
        (self.w * rhs.w)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> VecTraitF for Vec4<T> where
    T: Default + Float {

    type ValTypeF = T;

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec4, VecTraitF};
    /// let vec01 = Vec4::<f32>::new (1.0, 3.0, 0.0, 4.3);
    /// let vec02 = Vec4::<f32>::new (4.0, 9.0, 0.0, 1.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    fn distance (&self, rhs: &Vec4<T>) -> T {
        (self - rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec4, VecTraitF};
    /// let vec = Vec4::<f32>::new (1.0, 3.0, 0.0, 6.0);
    /// let vec_length = vec.length ();
    /// ```
    fn length (&self) -> T {

        (self.x * self.x +
         self.y * self.y +
         self.z * self.z +
         self.w * self.w).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec4, VecTraitF};
    /// let vec = Vec4::<f32>::new (3.0, 9.0, 0.0, 4.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    fn normalize (&self) -> Vec4<T> {

        let length = self.length ();

        if length != T::zero () {

            return Vec4::new (self.x / length,
                              self.y / length,
                              self.z / length,
                              self.w / length);
        }

        Vec4::zero ()
    }
}

/*===============================================================================================*/
/*------PUBLIC STATIC METHODS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec4<T> where
    T: Copy + Num + NumCast {

    /// Returns a `Vec4<V>` with a value of (0, 1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::up ();
    /// ```
    pub fn up () -> Vec4<T> {
        Vec4::new (0, 1, 0, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, -1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::down ();
    /// ```
    pub fn down () -> Vec4<T> {
        Vec4::new (0, -1, 0, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, -1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::left ();
    /// ```
    pub fn left () -> Vec4<T> {
        Vec4::new (-1, 0, 0, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 1, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::right ();
    /// ```
    pub fn right () -> Vec4<T> {
        Vec4::new (1, 0, 0, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 0, 1, 0)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::forward ();
    /// ```
    pub fn forward () -> Vec4<T> {
        Vec4::new (0, 0, 1, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<V>` with a value of (0, 0, -1, 0)
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::back ();
    /// ```
    pub fn back () -> Vec4<T> {
        Vec4::new (0, 0, -1, 0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec4<T>` with a value of (0, 0, 0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec4;
    /// let vec = Vec4::<f32>::zero ();
    /// ```
    pub fn zero () -> Vec4<T> {
        Vec4::from (0)
    }
}
