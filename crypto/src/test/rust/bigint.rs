/*
 * Copyright (c) 2024-2026 Pavel Vasin
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

use blacknet_crypto::bigint::{UInt256, UInt512};
use core::cmp::Ordering;

#[test]
fn parity() {
    let a = UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    let b = UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000001");
    let c = UInt256::from_hex("8000000000000000000000000000000000000000000000000000000000000000");
    let d = UInt256::from_hex("8000000000000000000000000000000000000000000000000000000000000001");
    assert!(a.is_even());
    assert!(!a.is_odd());
    assert!(!b.is_even());
    assert!(b.is_odd());
    assert!(c.is_even());
    assert!(!c.is_odd());
    assert!(!d.is_even());
    assert!(d.is_odd());
}

#[test]
fn hex() {
    let a = "82040BFACDA49378AA9A7091D231DF1C0000000000000000D30557C9C39DA70C";
    let b = UInt256::from_hex(a);
    let c = UInt256::from([
        0xD30557C9C39DA70C,
        0x0000000000000000,
        0xAA9A7091D231DF1C,
        0x82040BFACDA49378,
    ]);
    let d = format!("{:?}", c);
    assert_eq!(b, c);
    assert_eq!(d, a);
}

#[test]
fn bytes() {
    let a = UInt256::from_hex("82040BFACDA49378AA9A7091D231DF1C0000000000000000D30557C9C39DA70C");
    let b = [
        0x0C, 0xA7, 0x9D, 0xC3, 0xC9, 0x57, 0x05, 0xD3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x1C, 0xDF, 0x31, 0xD2, 0x91, 0x70, 0x9A, 0xAA, 0x78, 0x93, 0xA4, 0xCD, 0xFA, 0x0B,
        0x04, 0x82,
    ];
    assert_eq!(UInt256::from_le_bytes(b), a);
    assert_eq!(a.to_le_bytes::<32>(), b);
}

#[test]
fn cmp() {
    let a = UInt256::from_hex("C022ACCCD2A8701667BE02D3D240A92ADB463CC5A1804DEE6719F97EB1870985");
    let b = UInt256::from_hex("8DC10CA58FAB02B7640643AAEEE96BCA50980538C74BC5299F1E62EBD5C1D5CC");
    assert_eq!(a.cmp(&a), Ordering::Equal);
    assert_eq!(a.cmp(&b), Ordering::Greater);
    assert_eq!(b.cmp(&a), Ordering::Less);
}

#[test]
fn and() {
    let a = UInt256::from_hex("B2DFA0FE5E6FF3E86D499069C13FC781B5BE49BE1C42D6AA2BD8853280195D86");
    let b = UInt256::from_hex("AB8A146F53EC7D333E18EC7F9F15BE617C9F23028210AF1BD0DCFA12BE765069");
    let c = UInt256::from_hex("A28A006E526C71202C08806981158601349E01020000860A00D8801280105000");
    let d = UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    let e = UInt256::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    assert_eq!(a & b, c);
    assert_eq!(a & d, d);
    assert_eq!(a & e, a);
}

#[test]
fn or() {
    let a = UInt256::from_hex("47E41848E0EDA8E114198DF49B6591859D340B2C52657E96B997B45A69CD1489");
    let b = UInt256::from_hex("4650D3F72575A579F78A8ACA4D9C7EBC71BD2405256C3283C022F7C3A382871B");
    let c = UInt256::from_hex("47F4DBFFE5FDADF9F79B8FFEDFFDFFBDFDBD2F2D776D7E97F9B7F7DBEBCF979B");
    assert_eq!(a | a, a);
    assert_eq!(a | b, c);
}

#[test]
fn add() {
    let a = UInt256::from_hex("47E41848E0EDA8E114198DF49B6591859D340B2C52657E96B997B45A69CD1489");
    let b = UInt256::from_hex("4650D3F72575A579F78A8ACA4D9C7EBC71BD2405256C3283C022F7C3A382871B");
    let c = UInt256::from_hex("8E34EC4006634E5B0BA418BEE90210420EF12F3177D1B11A79BAAC1E0D4F9BA4");
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
}

#[test]
fn shl() {
    let a = UInt256::from_hex("C2077969192C8466727494B6D4589D0913670F1ACC7FF5EE284DE8F2E73F623A");
    let b = UInt256::from_hex("840EF2D2325908CCE4E9296DA8B13A1226CE1E3598FFEBDC509BD1E5CE7EC474");
    let c = UInt256::from_hex("103BCB48C964233393A4A5B6A2C4E8489B3878D663FFAF71426F479739FB11D0");
    let d = UInt256::from_hex("81DE5A464B21199C9D252DB516274244D9C3C6B31FFD7B8A137A3CB9CFD88E80");
    let e = UInt256::from_hex("1DE5A464B21199C9D252DB516274244D9C3C6B31FFD7B8A137A3CB9CFD88E800");
    assert_eq!(a << 1, b);
    assert_eq!(b << 2, c);
    assert_eq!(c << 3, d);
    assert_eq!(d << 4, e);
}

#[test]
fn shr() {
    let a = UInt256::from_hex("BE6DEFEC052D76C02BC0AE6539ED1494C1738703E0292310FC809FEBF189F62D");
    let b = UInt256::from_hex("5F36F7F60296BB6015E057329CF68A4A60B9C381F01491887E404FF5F8C4FB16");
    let c = UInt256::from_hex("17CDBDFD80A5AED8057815CCA73DA292982E70E07C0524621F9013FD7E313EC5");
    let d = UInt256::from_hex("02F9B7BFB014B5DB00AF02B994E7B4525305CE1C0F80A48C43F2027FAFC627D8");
    let e = UInt256::from_hex("002F9B7BFB014B5DB00AF02B994E7B4525305CE1C0F80A48C43F2027FAFC627D");
    assert_eq!(a >> 1, b);
    assert_eq!(b >> 2, c);
    assert_eq!(c >> 3, d);
    assert_eq!(d >> 4, e);
}

#[test]
fn neg() {
    let a = UInt256::from_hex("5927CDC6E4408057E3518EDBC4FEE6605A331478F16E1D136752941E66B1A689");
    let b = UInt256::from_hex("A6D832391BBF7FA81CAE71243B01199FA5CCEB870E91E2EC98AD6BE1994E5977");
    let c = UInt256::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    assert_eq!(-a, b);
    assert_eq!(-b, a);
    assert_eq!(-UInt256::ONE, c);
    assert_eq!(-UInt256::ZERO, UInt256::ZERO);
}

#[test]
fn sub() {
    let a = UInt256::from_hex("1C4C6C169AB464300A4AE33B768D66286C41B1B0538B4A6CDCABDB6E8863BD9F");
    let b = UInt256::from_hex("57F74EBB22F6E7244075275B20E2B4AC3D1E693BED584E4D1D7586549AE10C6A");
    let c = UInt256::from_hex("C4551D5B77BD7D0BC9D5BBE055AAB17C2F2348746632FC1FBF365519ED82B135");
    let d = UInt256::from_hex("3BAAE2A4884282F4362A441FAA554E83D0DCB78B99CD03E040C9AAE6127D4ECB");
    assert_eq!(a - b, c);
    assert_eq!(b - a, d);
}

#[test]
fn widening_mul() {
    let a = UInt256::from_hex("00F97CF9ECB6C4F46539B01EE42D4E9A2492C11E609BDE89B73DE031C564E2D7");
    let b = UInt256::from_hex("32192A679771050176C433E125E9389AA7185605C591D8201BA219EB7AAE6037");
    let c = UInt512::from_hex(
        "0030D2EF5819B091A74BE63C749C5ECE71995382110FE3BCFA03A0E4C26578B7452E2E03236EC2422C7EB89A069CA17D8C3F288A32A977CEF3154DC0E1DF5C31",
    );
    assert_eq!(a.widening_mul(b), c);
    assert_eq!(b.widening_mul(a), c);
}

#[test]
fn div() {
    let a = UInt256::from_hex("4C52834CB4D297D60C7B25F17668EFFDA3014AD26F915E87B6972ACB24D4FB9B");
    let b = 0xD6E656EBDCF8B303;
    let c = UInt256::from_hex("00000000000000005AEB502AE6B58C73FC2B5A37009554DAA36C83B133A798E4");
    assert_eq!(a / b, c);
}

#[test]
fn rem() {
    let a = UInt256::from_hex("638EC2DB4E4A587CC7A2C140D8BFBE16DC3E2DDEAE37C08B68F918B2752BD9BC");
    let b = 0xB9F1448812BA25C8;
    let c = 0x9963480625D9687C;
    assert_eq!(a % b, c);
}

#[test]
fn widening_square() {
    let a = UInt256::from_hex("6B3D316D1EEEEB1FF5A4A8F48B87F93B9C0C6CE0DE0AB366CD6901E304C7F7B9");
    let b = UInt512::from_hex(
        "2CEC35F1CF676AC395C6090AAD06AD8AA93B975F4EDB571C88FA5F7E1C416F3303186FE30B4BCA7D00B8357919AA2B42D7771EB3CC1FDB8E8A69A006D95483B1",
    );
    assert_eq!(a.widening_mul(a), b);
    assert_eq!(a.widening_square(), b);
}

#[test]
fn clz() {
    let a = UInt256::from_hex("0000000000000000000000000000000000000000000000000000000000000000");
    let b = UInt256::from_hex("00000000000000000000FFFFFFFF0000FFFFFFFFFFFFFFFFFFFFFFFF00000000");
    let c = UInt256::from_hex("0000FFFFFFFFFFFFFFFFFFFFFFFF0000FFFFFFFFFFFFFFFFFFFFFFFF00000000");
    let d = UInt256::from_hex("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF");
    assert_eq!(a.leading_zeros(), 256);
    assert_eq!(b.leading_zeros(), 80);
    assert_eq!(c.leading_zeros(), 16);
    assert_eq!(d.leading_zeros(), 0);
}
