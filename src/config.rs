/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */
use serde::Deserialize;
use std::collections::HashMap;

use crate::account::ConnectionInfo;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Config {
    pub accounts: HashMap<String, ConnectionInfo>,
    pub bell: Option<bool>,
}
