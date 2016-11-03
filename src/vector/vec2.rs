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
use ::vector::{Vec3, Vec4, VecTrait, VecTraitF};

use std::convert::From;
use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};

/*===============================================================================================*/
/*------VEC2 STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// The generic Vec2 struct.
///
/// It is used mainly for 2D related mathematics such as texture coordinates,
/// UV coordinates, etc. <br>
/// It can accept any number as a value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2<T> where
    T: Copy + Num + NumCast {

    // Public
    /// The vector x-coordinate.
    pub x: T,
    /// The vector y-coordinate.
    pub y: T,
}

// Predefined Vec2 types
/// `Vec2<f32>`
pub type Vec2f = Vec2<f32>;
/// `Vec2<i32>`
pub type Vec2i = Vec2<i32>;
/// `Vec2<u32>`
pub type Vec2u = Vec2<u32>;

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec2<T> where
    T: Copy + Num + NumCast {

    /// Returns a new `Vec2<T>` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::new (3, 7);
    /// ```
    pub fn new<C> (x: C, y: C) -> Vec2<T> where
        C: Copy + Num + NumCast {

        Vec2 {x: T::from (x).unwrap (),
              y: T::from (y).unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T, U> From<U> for Vec2<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: U) -> Vec2<T> {

        Vec2::new (T::from (value).unwrap (),
                   T::from (value).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec2<U>> for Vec2<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec2<U>) -> Vec2<T> {

        Vec2::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec3<U>> for Vec2<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec3<U>) -> Vec2<T> {

        Vec2::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T, U> From<&'a Vec4<U>> for Vec2<T> where
    T: Copy + Num + NumCast,
    U: Copy + Num + NumCast {

    fn from (value: &Vec4<U>) -> Vec2<T> {

        Vec2::new (T::from (value.x).unwrap (),
                   T::from (value.y).unwrap ())
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Add for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<&'a Vec2<T>> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<Vec2<T>> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: Vec2<T>) -> Vec2<T> {

       Vec2::new (self.x + rhs.x,
                  self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Add<&'a Vec2<T>> for &'b Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x + rhs.x,
                   self.y + rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Add<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x + rhs,
                   self.y + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Add<T> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn add (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x + rhs,
                   self.y + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign for Vec2<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: Vec2<T>) {

        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> AddAssign<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    fn add_assign (&mut self, rhs: T) {

        self.x = self.x + rhs;
        self.y = self.y + rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<&'a Vec2<T>> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<Vec2<T>> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Sub<&'a Vec2<T>> for &'b Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x - rhs.x,
                   self.y - rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Sub<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Sub<T> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn sub (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x - rhs,
                   self.y - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign for Vec2<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: Vec2<T>) {

        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> SubAssign<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    fn sub_assign (&mut self, rhs: T) {

        self.x = self.x - rhs;
        self.y = self.y - rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<&'a Vec2<T>> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<Vec2<T>> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Mul<&'a Vec2<T>> for &'b Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x * rhs.x,
                   self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Mul<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x * rhs,
                   self.y * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Mul<T> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn mul (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x * rhs,
                   self.y * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign for Vec2<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: Vec2<T>) {

        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> MulAssign<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    fn mul_assign (&mut self, rhs: T) {

        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<&'a Vec2<T>> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<Vec2<T>> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b, T> Div<&'a Vec2<T>> for &'b Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (self.x / rhs.x,
                   self.y / rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Div<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x / rhs,
                   self.y / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> Div<T> for &'a Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = Vec2<T>;

    fn div (self, rhs: T) -> Vec2<T> {

        Vec2::new (self.x / rhs,
                   self.y / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign for Vec2<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: Vec2<T>) {

        self.x = self.x / rhs.x;
        self.y = self.y / rhs.y;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> DivAssign<T> for Vec2<T> where
    T: Copy + Num + NumCast {

    fn div_assign (&mut self, rhs: T) {

        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> Index<u8> for Vec2<T> where
    T: Copy + Num + NumCast {

    type Output = T;

    fn index (&self, index: u8) -> &T {

        match index {

            0 => &self.x,
            1 => &self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> IndexMut<u8> for Vec2<T> where
    T: Copy + Num + NumCast {

    fn index_mut (&mut self, index: u8) -> &mut T {

        match index {

            0 => &mut self.x,
            1 => &mut self.y,
            _ => unreachable! ("Index out of range for Vec2")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> VecTrait for Vec2<T> where
    T: Copy + Default + Num + NumCast + PartialOrd {

    type ValType = T;

    fn lerp (start: &Vec2<T>, end: &Vec2<T>, percentage: f32) -> Vec2<T> {

        Vec2::new (util::lerp (start.x, end.x, percentage),
                   util::lerp (start.y, end.y, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn max (lhs: &Vec2<T>, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (util::max (lhs.x, rhs.x),
                   util::max (lhs.y, rhs.y))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: &Vec2<T>, rhs: &Vec2<T>) -> Vec2<T> {

        Vec2::new (util::min (lhs.x, rhs.x),
                   util::min (lhs.y, rhs.y))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn clamp (&self, min: &Vec2<T>, max: &Vec2<T>) -> Vec2<T> {

        Vec2::new (util::clamp (self.x, min.x, max.x),
                   util::clamp (self.y, min.y, max.y))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn dot (&self, rhs: &Vec2<T>) -> T {

        (self.x * rhs.x) +
        (self.y * rhs.y)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> VecTraitF for Vec2<T> where
    T: Default + Float {

    type ValTypeF = T;

    /// Returns the distance between two vectors.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec01 = Vec2::<f32>::new (1.0, 3.0);
    /// let vec02 = Vec2::<f32>::new (4.0, 9.0);
    ///
    /// let distance = vec01.distance (&vec02);
    /// ```
    fn distance (&self, rhs: &Vec2<T>) -> T {
        (self - rhs).length ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the length of a vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec = Vec2::<f32>::new (1.0, 3.0);
    /// let vec_length = vec.length ();
    /// ```
    fn length (&self) -> T {

        (self.x * self.x +
         self.y * self.y).sqrt ()
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a normalized vector.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::{Vec2, VecTraitF};
    /// let vec = Vec2::<f32>::new (3.0, 9.0);
    /// let vec_normalized = vec.normalize ();
    /// ```
    fn normalize (&self) -> Vec2<T> {

        let length = self.length ();

        if length != T::zero () {

            return Vec2::new (self.x / length,
                              self.y / length);
        }

        Vec2::zero ()
    }
}

/*===============================================================================================*/
/*------PUBLIC STATIC METHODS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Vec2<T> where
    T: Copy + Num + NumCast {

    /// Returns a `Vec2<T>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::up ();
    /// ```
    pub fn up () -> Vec2<T> {

        Vec2::new (T::zero (),
                   T::one  ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<T>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::down ();
    /// ```
    pub fn down () -> Vec2<T> {

        Vec2::new (T::zero (),
                   T::from (-1).unwrap ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<T>` with a value of (0, -1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::left ();
    /// ```
    pub fn left () -> Vec2<T> {

        Vec2::new (T::from (-1).unwrap (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<T>` with a value of (0, 1).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::right ();
    /// ```
    pub fn right () -> Vec2<T> {

        Vec2::new (T::one  (),
                   T::zero ())
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a `Vec2<T>` with a value of (0, 0).
    ///
    /// # Examples
    /// ```
    /// # use ion_math::vector::Vec2;
    /// let vec = Vec2::<f32>::zero ();
    /// ```
    pub fn zero () -> Vec2<T> {
        Vec2::from (T::zero ())
    }
}
