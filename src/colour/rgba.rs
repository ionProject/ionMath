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

use ::colour::ColourTrait;
use ::util;
use ::vector::{Vec2, Vec3, Vec4};

use std::convert::From;
use std::ops::{Add,   AddAssign,
               Sub,   SubAssign,
               Mul,   MulAssign,
               Div,   DivAssign,
               Index, IndexMut};

/*===============================================================================================*/
/*------RGBA STRUCT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Stores a RGBA colour value.
#[cfg_attr (feature = "serde_serialize", derive (Deserialize, Serialize))]
#[derive (Copy, Clone, Debug, PartialEq)]
pub struct RGBA {

    // Public
    /// Red channel.
    pub r: f32,
    /// Green channel.
    pub g: f32,
    /// Blue channel.
    pub b: f32,
    /// Alpha channel.
    pub a: f32,
}

/*===============================================================================================*/
/*------CONSTRUCTORS-----------------------------------------------------------------------------*/
/*===============================================================================================*/

impl RGBA {

    /// Returns a new `RGBA` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::RGBA;
    /// let colour = RGBA::new (0.0, 1.0, 0.5, 1.0);
    /// ```
    pub fn new<T> (r: T, g: T, b: T, a: T) -> RGBA where
        T: Copy + Num + NumCast {

        RGBA {r: r.to_f32 ().unwrap (),
              g: g.to_f32 ().unwrap (),
              b: b.to_f32 ().unwrap (),
              a: a.to_f32 ().unwrap ()}
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<T> From<T> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: T) -> RGBA {

        RGBA::new (value,
                   value,
                   value,
                   value)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec2<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec2<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   T::zero (),
                   T::one  ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec3<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec3<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   value.z,
                   T::one ())
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, T> From<&'a Vec4<T>> for RGBA where
    T: Copy + Num + NumCast {

    fn from (value: &Vec4<T>) -> RGBA {

        RGBA::new (value.x,
                   value.y,
                   value.z,
                   value.w)
    }
}

/*===============================================================================================*/
/*------OPERATORS--------------------------------------------------------------------------------*/
/*===============================================================================================*/

impl Add for RGBA {

    type Output = RGBA;

    fn add (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r + rhs.r,
                   self.g + rhs.g,
                   self.b + rhs.b,
                   self.a + rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Add<&'a RGBA> for RGBA {

    type Output = RGBA;

    fn add (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r + rhs.r,
                   self.g + rhs.g,
                   self.b + rhs.b,
                   self.a + rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Add<RGBA> for &'a RGBA {

    type Output = RGBA;

    fn add (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r + rhs.r,
                   self.g + rhs.g,
                   self.b + rhs.b,
                   self.a + rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b> Add<&'a RGBA> for &'b RGBA {

    type Output = RGBA;

    fn add (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r + rhs.r,
                   self.g + rhs.g,
                   self.b + rhs.b,
                   self.a + rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Add<f32> for RGBA {

    type Output = RGBA;

    fn add (self, rhs: f32) -> RGBA {

        RGBA::new (self.r + rhs,
                   self.g + rhs,
                   self.b + rhs,
                   self.a + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Add<f32> for &'a RGBA {

    type Output = RGBA;

    fn add (self, rhs: f32) -> RGBA {

        RGBA::new (self.r + rhs,
                   self.g + rhs,
                   self.b + rhs,
                   self.a + rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign for RGBA {

    fn add_assign (&mut self, rhs: RGBA) {

        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl AddAssign<f32> for RGBA {

    fn add_assign (&mut self, rhs: f32) {

        self.r += rhs;
        self.g += rhs;
        self.b += rhs;
        self.a += rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub for RGBA {

    type Output = RGBA;

    fn sub (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r - rhs.r,
                   self.g - rhs.g,
                   self.b - rhs.b,
                   self.a - rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Sub<&'a RGBA> for RGBA {

    type Output = RGBA;

    fn sub (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r - rhs.r,
                   self.g - rhs.g,
                   self.b - rhs.b,
                   self.a - rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Sub<RGBA> for &'a RGBA {

    type Output = RGBA;

    fn sub (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r - rhs.r,
                   self.g - rhs.g,
                   self.b - rhs.b,
                   self.a - rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b> Sub<&'a RGBA> for &'b RGBA {

    type Output = RGBA;

    fn sub (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r - rhs.r,
                   self.g - rhs.g,
                   self.b - rhs.b,
                   self.a - rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Sub<f32> for RGBA {

    type Output = RGBA;

    fn sub (self, rhs: f32) -> RGBA {

        RGBA::new (self.r - rhs,
                   self.g - rhs,
                   self.b - rhs,
                   self.a - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Sub<f32> for &'a RGBA {

    type Output = RGBA;

    fn sub (self, rhs: f32) -> RGBA {

        RGBA::new (self.r - rhs,
                   self.g - rhs,
                   self.b - rhs,
                   self.a - rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign for RGBA {

    fn sub_assign (&mut self, rhs: RGBA) {

        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
        self.a -= rhs.a;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl SubAssign<f32> for RGBA {

    fn sub_assign (&mut self, rhs: f32) {

        self.r -= rhs;
        self.g -= rhs;
        self.b -= rhs;
        self.a -= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul for RGBA {

    type Output = RGBA;

    fn mul (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r * rhs.r,
                   self.g * rhs.g,
                   self.b * rhs.b,
                   self.a * rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Mul<&'a RGBA> for RGBA {

    type Output = RGBA;

    fn mul (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r * rhs.r,
                   self.g * rhs.g,
                   self.b * rhs.b,
                   self.a * rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Mul<RGBA> for &'a RGBA {

    type Output = RGBA;

    fn mul (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r * rhs.r,
                   self.g * rhs.g,
                   self.b * rhs.b,
                   self.a * rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b> Mul<&'a RGBA> for &'b RGBA {

    type Output = RGBA;

    fn mul (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r * rhs.r,
                   self.g * rhs.g,
                   self.b * rhs.b,
                   self.a * rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Mul<f32> for RGBA {

    type Output = RGBA;

    fn mul (self, rhs: f32) -> RGBA {

        RGBA::new (self.r * rhs,
                   self.g * rhs,
                   self.b * rhs,
                   self.a * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Mul<f32> for &'a RGBA {

    type Output = RGBA;

    fn mul (self, rhs: f32) -> RGBA {

        RGBA::new (self.r * rhs,
                   self.g * rhs,
                   self.b * rhs,
                   self.a * rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign for RGBA {

    fn mul_assign (&mut self, rhs: RGBA) {

        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl MulAssign<f32> for RGBA {

    fn mul_assign (&mut self, rhs: f32) {

        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
        self.a *= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div for RGBA{

    type Output = RGBA;

    fn div (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r / rhs.r,
                   self.g / rhs.g,
                   self.b / rhs.b,
                   self.a / rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Div<&'a RGBA> for RGBA {

    type Output = RGBA;

    fn div (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r / rhs.r,
                   self.g / rhs.g,
                   self.b / rhs.b,
                   self.a / rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Div<RGBA> for &'a RGBA {

    type Output = RGBA;

    fn div (self, rhs: RGBA) -> RGBA {

        RGBA::new (self.r / rhs.r,
                   self.g / rhs.g,
                   self.b / rhs.b,
                   self.a / rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a, 'b> Div<&'a RGBA> for &'b RGBA {

    type Output = RGBA;

    fn div (self, rhs: &RGBA) -> RGBA {

        RGBA::new (self.r / rhs.r,
                   self.g / rhs.g,
                   self.b / rhs.b,
                   self.a / rhs.a)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Div<f32> for RGBA {

    type Output = RGBA;

    fn div (self, rhs: f32) -> RGBA {

        RGBA::new (self.r / rhs,
                   self.g / rhs,
                   self.b / rhs,
                   self.a / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl<'a> Div<f32> for &'a RGBA {

    type Output = RGBA;

    fn div (self, rhs: f32) -> RGBA {

        RGBA::new (self.r / rhs,
                   self.g / rhs,
                   self.b / rhs,
                   self.a / rhs)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign for RGBA {

    fn div_assign (&mut self, rhs: RGBA) {

        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
        self.a /= rhs.a;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl DivAssign<f32> for RGBA {

    fn div_assign (&mut self, rhs: f32) {

        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
        self.a /= rhs;
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl Index<u8> for RGBA {

    type Output = f32;

    fn index (&self, index: u8) -> &f32 {

        match index {

            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            3 => &self.a,
            _ => unreachable! ("Index out of range for RGBA")
        }
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl IndexMut<u8> for RGBA {

    fn index_mut (&mut self, index: u8) -> &mut f32 {

        match index {

            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            3 => &mut self.a,
            _ => unreachable! ("Index out of range for RGBA")
        }
    }
}

/*===============================================================================================*/
/*------TRAIT IMPLEMENTATIONS--------------------------------------------------------------------*/
/*===============================================================================================*/

impl ColourTrait for RGBA {

    /// Returns the colour black.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::black ();
    /// ```
    fn black () -> RGBA {
        RGBA::new (0, 0, 0, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour light grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::light_grey ();
    /// ```
    fn light_grey () -> RGBA {
        RGBA::new (0.75, 0.75, 0.75, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::grey ();
    /// ```
    fn grey () -> RGBA {
        RGBA::new (0.5, 0.5, 0.5, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour dark grey.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::dark_grey ();
    /// ```
    fn dark_grey () -> RGBA {
        RGBA::new (0.25, 0.25, 0.25, 1.0)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour white.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::white ();
    /// ```
    fn white () -> RGBA {
        RGBA::from (1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour red.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::red ();
    /// ```
    fn red () -> RGBA {
        RGBA::new (1, 0, 0, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour green.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::green ();
    /// ```
    fn green () -> RGBA {
        RGBA::new (0, 1, 0, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour blue.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::blue ();
    /// ```
    fn blue () -> RGBA {
        RGBA::new (0, 0, 1, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour yellow.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::yellow ();
    /// ```
    fn yellow () -> RGBA {
        RGBA::new (1, 1, 0, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour cyan.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::cyan ();
    /// ```
    fn cyan () -> RGBA {
        RGBA::new (0, 1, 1, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns the colour magenta.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::magenta ();
    /// ```
    fn magenta () -> RGBA {
        RGBA::new (1, 0, 1, 1)
    }

/*-----------------------------------------------------------------------------------------------*/

    /// Returns a fully transparent colour.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::{ColourTrait, RGBA};
    /// let colour = RGBA::transparent ();
    /// ```
    fn transparent () -> RGBA {
        RGBA::from (0)
    }

/*-----------------------------------------------------------------------------------------------*/

    fn lerp (start: &RGBA, end: &RGBA, percentage: f32) -> RGBA {

        RGBA::new (util::lerp (start.r, end.r, percentage),
                   util::lerp (start.g, end.g, percentage),
                   util::lerp (start.b, end.b, percentage),
                   util::lerp (start.a, end.a, percentage))
    }

/*-----------------------------------------------------------------------------------------------*/

    fn clamp (&self, min: &RGBA, max: &RGBA) -> RGBA {

        RGBA::new (util::clamp (self.r, min.r, max.r),
                   util::clamp (self.g, min.g, max.g),
                   util::clamp (self.b, min.b, max.b),
                   util::clamp (self.a, min.a, max.a))
    }
}
