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

/*===============================================================================================*/
//! ionMath is a simple and fast mathematics library.
//!
//! It is a core part of the ion Project, and provides many functions that are useful for <br>
//! computer graphics and video game development.
//!
//! This includes:                 <br>
//! - Vectors                      <br>
//! - Matrices                     <br>
//! - Quaternions                  <br>
//! - Angles (degrees and radians) <br>
//! - Colours and colour conversion
/*===============================================================================================*/

// Crate attributes
#![deny     (missing_debug_implementations)]
#![deny     (missing_docs)]
#![cfg_attr (feature = "serde_serialize", feature (proc_macro))]

// Crate imports
#[cfg (feature = "serde_serialize")]
#[macro_use]
extern crate serde_derive;

// Modules
pub mod angle;
pub mod matrix;
pub mod util;
pub mod vector;
