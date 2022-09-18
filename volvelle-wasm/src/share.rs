// Volvelle Website
// Written in 2022 by
//     Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Share Management
//!

use crate::fe::Fe;
use serde::{Deserialize, Serialize};

/// A share
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Share(Vec<Option<Fe>>);

