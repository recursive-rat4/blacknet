/*
 * Copyright (c) 2026 Pavel Vasin
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use blacknet_compat::Mode;
use blacknet_compat::config::Config;

#[test]
fn regtest() {
    let mode = Mode::regtest();
    let conf = mode.blacknet_conf();
    match Config::parse(conf) {
        Ok(_) => {}
        Err(err) => assert!(false, "{err}"),
    }
}

#[test]
fn mainnet() {
    let mode = Mode::mainnet();
    let conf = mode.blacknet_conf();
    match Config::parse(conf) {
        Ok(_) => {}
        Err(err) => assert!(false, "{err}"),
    }
}
