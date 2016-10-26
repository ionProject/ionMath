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
/*------CLAMP TRAIT------------------------------------------------------------------------------*/
/*===============================================================================================*/

/// Clamp trait.
pub trait Clamp {

    /// Clamps a value between two numbers.
    fn clamp (&self, min: &Self, max: &Self) -> Self;
}

/*===============================================================================================*/
/*------CLAMP TRAIT IMPLEMENTATIONS--------------------------------------------------------------*/
/*===============================================================================================*/

impl<T> Clamp for T where
    T: Copy + Num + PartialOrd {

    fn clamp (&self, min: &T, max: &T) -> T {

        debug_assert! (min < max, "Min cannot be greater than max.");
        if self < min {*min} else if self > max {*max} else {*self}
    }
}
