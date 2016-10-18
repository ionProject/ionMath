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
use self::num_traits::Num;

/*===============================================================================================*/
/*------MINMAX TRAIT-----------------------------------------------------------------------------*/
/*===============================================================================================*/

/// `MinMax` trait.
pub trait MinMax {

    /// Returns the largest of two values.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::util::MinMax;
    /// let num = i32::max (43, 7);
    /// ```
    fn max (lhs: Self, rhs: Self) -> Self;

    /// Returns the smallest of two numbers.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::util::MinMax;
    /// let num = i32::min (43, 7);
    /// ```
    fn min (lhs: Self, rhs: Self) -> Self;
}

/*===============================================================================================*/
/*------MINMAX TRAIT IMPLEMENTATIONS-------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> MinMax for T where
    T: Num + PartialOrd {

    fn max (lhs: Self, rhs: Self) -> Self {

        if lhs > rhs {lhs} else {rhs}
    }

/*-----------------------------------------------------------------------------------------------*/

    fn min (lhs: Self, rhs: Self) -> Self {

        if lhs < rhs {lhs} else {rhs}
    }
}
