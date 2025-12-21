/*
 * Copyright (c) 2025 Pavel Vasin
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

use blacknet_network::i2psam::Answer;

#[test]
fn values() {
    let newlined = Answer::new("HELLO REPLY RESULT=OK VERSION=3.3\n".to_owned());
    assert_eq!(newlined.get("VERSION").unwrap(), "3.3");

    let quoted = Answer::new(
        "HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n".to_owned(),
    );
    assert_eq!(
        quoted.get("MESSAGE").unwrap(),
        "Must start with HELLO VERSION"
    );
}

#[test]
fn oks() {
    let yay = Answer::new("HELLO REPLY RESULT=OK VERSION=3.3\n".to_owned());
    assert_eq!(yay.ok().unwrap(), ());

    let nay = Answer::new(
        "HELLO REPLY RESULT=I2P_ERROR MESSAGE=\"Must start with HELLO VERSION\"\n".to_owned(),
    );
    assert!(nay.ok().is_err());
}

#[test]
fn destinations() {
    assert_eq!(
        Answer::hash("EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttdxKZ7mzXi8roW~OiBVCZFFApYICToUMlefZ-ZMxtm213EpnubNeLyuhb86IFUJkUUClggJOhQyV59n5kzG2bbXcSme5s14vK6FvzogVQmRRQKWCAk6FDJXn2fmTMbZttd3bv4RZ3HHk0U1v2T5r8N6TFmPNsTli1XzmB20yGQHW4BQAEAAcAAA==").unwrap(),
        [ 81, 169, 239, 153, 149, 11, 34, 49, 163, 77, 41, 180, 244, 162, 252, 194, 49, 92, 204, 43, 2, 56, 105, 63, 140, 102, 235, 132, 22, 244, 63, 19 ]
    );
}
