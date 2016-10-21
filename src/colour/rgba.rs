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

// Module imports
use std::convert::From;

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
/*------RGBA TRAIT IMPLEMENTATIONS---------------------------------------------------------------*/
/*===============================================================================================*/

impl Default for RGBA {

    fn default () -> Self {
        RGBA::from (1.0)
    }
}

/*-----------------------------------------------------------------------------------------------*/

impl From<f32> for RGBA {

    fn from (value: f32) -> Self {
        RGBA::new (value, value, value, value)
    }
}

/*===============================================================================================*/
/*------RGBA PUBLIC METHODS----------------------------------------------------------------------*/
/*===============================================================================================*/

impl RGBA {

    /// Returns a new `RGBA` instance.
    ///
    /// # Examples
    /// ```
    /// # use ion_math::colour::RGBA;
    /// let colour = RGBA::new (0.0, 1.0, 0.5, 1.0);
    /// ```
    pub fn new (r: f32, g: f32, b: f32, a: f32) -> Self {
        RGBA {r: r, g: g, b: b, a: a}
    }
}
