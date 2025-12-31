/*
 * Copyright (c) 2024-2025 Pavel Vasin
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

use blacknet_crypto::algebra::AdditiveMonoid;
use blacknet_crypto::bigint::UInt256;

type F = blacknet_crypto::field25519::Field25519;

#[test]
fn group_add_affine() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupAffine;
    let ax = F::from_hex("1E3DBD8EF7121F586A32C8789BE6C1BD516EA0B7B5E00D356527F3B9137C7F13");
    let ay = F::from_hex("10833664A42569382BCDF87CCF2D0423A5CDBF39666777496B1A17D1265138E7");
    let bx = F::from_hex("172C422E616DC9017CB392143DCDB133E1071D8E87806CCD9B222D82665AAC69");
    let by = F::from_hex("39C554667DD211EB35B90AD285D01819B184E5C2ED777BF10C4E7039E853D5F5");
    let cx = F::from_hex("1CF5593AE7B4FD9F9AADB739818D5A3A027C5481D295567AC053E3EE711CF3D6");
    let cy = F::from_hex("4734A155F67FF10F2371E91A5EF1C89E0A246AF9FE5F4C76B3EB72325335E3AF");
    let dx = F::from_hex("3D3B0EA90D13082AA6862F0DAC1E211C286614F222BAFE7210862D448EF0E467");
    let dy = F::from_hex("32AB5CACFE5225A4BF684AF87237E34A5A8F8FB4608AD5994F2A327A06513A42");
    let ex = F::from_hex("75E985B20C0DF581F0941168FCE3006628E6F5086125A916031E11F4C1AC1533");
    let ey = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let fx = F::from_hex("0A167A4DF3F20A7E0F6BEE97031CFF99D7190AF79EDA56E9FCE1EE0B3E53EABA");
    let fy = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    let e = G::new(ex, ey);
    let f = G::new(fx, fy);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(d + d, e);
    assert_eq!(e + f, G::IDENTITY);
    assert_eq!(G::IDENTITY + c, c);
    assert_eq!(c + G::IDENTITY, c);
    assert_eq!(G::IDENTITY + G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_neg_affine() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupAffine;
    let ax = F::from_hex("2C998F5CD6F89A5323244238DCB0E122F3C48B690D17895D64C622FE7B134873");
    let ay = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let bx = F::from_hex("536670A3290765ACDCDBBDC7234F1EDD0C3B7496F2E876A29B39DD0184ECB77A");
    let by = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    assert_eq!(-a, b);
    assert_eq!(-G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_sub_affine() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupAffine;
    let ax = F::from_hex("248949BF1E33E577C48DF9037C0FEDCE42EA070F91125CD796F49349A994794D");
    let ay = F::from_hex("1736DB3E2DA93A423B2DC4E1A010CF1BAB40AF86FAE99E7ABAB19262E335E3FC");
    let bx = F::from_hex("342CFACF5781EFBB03D6326015C9078AAC0FBC7E5F17D6AD71C9BD8D5BB0E41D");
    let by = F::from_hex("3E124FEECFA34E7E6BE1773135B1F5BE7D66E4D22B33243361AF0179EC747917");
    let cx = F::from_hex("329A08756FA031A978BDB2DC8CE0191B67A31A11061AB327DD6BB07FCA2B1637");
    let cy = F::from_hex("1A67928E14B11CB3C91CD549C14AF67810F38F29163459EDC4FA31C85C8F2D10");
    let dx = F::from_hex("124713CD5616381192FDB5BB7868ABA8A48952687874B05F8CA79FFECA50FCB9");
    let dy = F::from_hex("1A0CF87C5FE58923F6C85DA6A9B0D0B812C34CE97CC8374F518D9B4E5B54904D");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    assert_eq!(a - b, c);
    assert_eq!(-b + a, c);
    assert_eq!(d - d, G::IDENTITY);
    assert_eq!(G::IDENTITY - c, -c);
    assert_eq!(c - G::IDENTITY, c);
    assert_eq!(G::IDENTITY - G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_mul_affine() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupAffine;
    let ax = F::from_hex("3AED134ED42AD34F18DB7529FB0ED4470DBB0A157D676ECA74F7789208B87678");
    let ay = F::from_hex("278B8C16CEFF8BD165602933BC9CA4E4365C38F3CE8D17519172D76B8B046621");
    let cx = F::from_hex("24E994BFDF037A840793AC3321DBC483E081150B4C888FE8E6B77CBCCA117411");
    let cy = F::from_hex("64967B5AA7773EB87585C0D81E755B917A6AF58EA2F20E6A12B47D98FB9929F4");
    let b = UInt256::from_hex("0E18DDB951F8A3A10C33028E6CD15A9B4480C3C825F515B6DA24B75E7C813623");
    let d = UInt256::from_hex("251D364ED569CBF14184665CE3FA321E9678002959E04609D1A0ECC692CEE9E1");
    let a = G::new(ax, ay);
    let c = G::new(cx, cy);
    assert_eq!(a * b.bits::<255>(), c);
    assert_eq!(a * UInt256::ONE.bits::<255>(), a);
    assert_eq!(a * UInt256::ZERO.bits::<255>(), G::IDENTITY);
    assert_eq!(G::IDENTITY * d.bits::<255>(), G::IDENTITY);
}

#[test]
fn group_add_extended() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupExtended;
    let ax = F::from_hex("1E3DBD8EF7121F586A32C8789BE6C1BD516EA0B7B5E00D356527F3B9137C7F13");
    let ay = F::from_hex("10833664A42569382BCDF87CCF2D0423A5CDBF39666777496B1A17D1265138E7");
    let bx = F::from_hex("172C422E616DC9017CB392143DCDB133E1071D8E87806CCD9B222D82665AAC69");
    let by = F::from_hex("39C554667DD211EB35B90AD285D01819B184E5C2ED777BF10C4E7039E853D5F5");
    let cx = F::from_hex("1CF5593AE7B4FD9F9AADB739818D5A3A027C5481D295567AC053E3EE711CF3D6");
    let cy = F::from_hex("4734A155F67FF10F2371E91A5EF1C89E0A246AF9FE5F4C76B3EB72325335E3AF");
    let dx = F::from_hex("3D3B0EA90D13082AA6862F0DAC1E211C286614F222BAFE7210862D448EF0E467");
    let dy = F::from_hex("32AB5CACFE5225A4BF684AF87237E34A5A8F8FB4608AD5994F2A327A06513A42");
    let ex = F::from_hex("75E985B20C0DF581F0941168FCE3006628E6F5086125A916031E11F4C1AC1533");
    let ey = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let fx = F::from_hex("0A167A4DF3F20A7E0F6BEE97031CFF99D7190AF79EDA56E9FCE1EE0B3E53EABA");
    let fy = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    let e = G::new(ex, ey);
    let f = G::new(fx, fy);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(d + d, e);
    assert_eq!(e + f, G::IDENTITY);
    assert_eq!(G::IDENTITY + c, c);
    assert_eq!(c + G::IDENTITY, c);
    assert_eq!(G::IDENTITY + G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_neg_extended() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupExtended;
    let ax = F::from_hex("2C998F5CD6F89A5323244238DCB0E122F3C48B690D17895D64C622FE7B134873");
    let ay = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let bx = F::from_hex("536670A3290765ACDCDBBDC7234F1EDD0C3B7496F2E876A29B39DD0184ECB77A");
    let by = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    assert_eq!(-a, b);
    assert_eq!(-G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_sub_extended() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupExtended;
    let ax = F::from_hex("248949BF1E33E577C48DF9037C0FEDCE42EA070F91125CD796F49349A994794D");
    let ay = F::from_hex("1736DB3E2DA93A423B2DC4E1A010CF1BAB40AF86FAE99E7ABAB19262E335E3FC");
    let bx = F::from_hex("342CFACF5781EFBB03D6326015C9078AAC0FBC7E5F17D6AD71C9BD8D5BB0E41D");
    let by = F::from_hex("3E124FEECFA34E7E6BE1773135B1F5BE7D66E4D22B33243361AF0179EC747917");
    let cx = F::from_hex("329A08756FA031A978BDB2DC8CE0191B67A31A11061AB327DD6BB07FCA2B1637");
    let cy = F::from_hex("1A67928E14B11CB3C91CD549C14AF67810F38F29163459EDC4FA31C85C8F2D10");
    let dx = F::from_hex("124713CD5616381192FDB5BB7868ABA8A48952687874B05F8CA79FFECA50FCB9");
    let dy = F::from_hex("1A0CF87C5FE58923F6C85DA6A9B0D0B812C34CE97CC8374F518D9B4E5B54904D");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    assert_eq!(a - b, c);
    assert_eq!(-b + a, c);
    assert_eq!(d - d, G::IDENTITY);
    assert_eq!(G::IDENTITY - c, -c);
    assert_eq!(c - G::IDENTITY, c);
    assert_eq!(G::IDENTITY - G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_mul_extended() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupExtended;
    let ax = F::from_hex("3AED134ED42AD34F18DB7529FB0ED4470DBB0A157D676ECA74F7789208B87678");
    let ay = F::from_hex("278B8C16CEFF8BD165602933BC9CA4E4365C38F3CE8D17519172D76B8B046621");
    let cx = F::from_hex("24E994BFDF037A840793AC3321DBC483E081150B4C888FE8E6B77CBCCA117411");
    let cy = F::from_hex("64967B5AA7773EB87585C0D81E755B917A6AF58EA2F20E6A12B47D98FB9929F4");
    let b = UInt256::from_hex("0E18DDB951F8A3A10C33028E6CD15A9B4480C3C825F515B6DA24B75E7C813623");
    let d = UInt256::from_hex("251D364ED569CBF14184665CE3FA321E9678002959E04609D1A0ECC692CEE9E1");
    let a = G::new(ax, ay);
    let c = G::new(cx, cy);
    assert_eq!(a * b.bits::<255>(), c);
    assert_eq!(a * UInt256::ONE.bits::<255>(), a);
    assert_eq!(a * UInt256::ZERO.bits::<255>(), G::IDENTITY);
    assert_eq!(G::IDENTITY * d.bits::<255>(), G::IDENTITY);
}

#[test]
fn group_add_projective() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupProjective;
    let ax = F::from_hex("1E3DBD8EF7121F586A32C8789BE6C1BD516EA0B7B5E00D356527F3B9137C7F13");
    let ay = F::from_hex("10833664A42569382BCDF87CCF2D0423A5CDBF39666777496B1A17D1265138E7");
    let bx = F::from_hex("172C422E616DC9017CB392143DCDB133E1071D8E87806CCD9B222D82665AAC69");
    let by = F::from_hex("39C554667DD211EB35B90AD285D01819B184E5C2ED777BF10C4E7039E853D5F5");
    let cx = F::from_hex("1CF5593AE7B4FD9F9AADB739818D5A3A027C5481D295567AC053E3EE711CF3D6");
    let cy = F::from_hex("4734A155F67FF10F2371E91A5EF1C89E0A246AF9FE5F4C76B3EB72325335E3AF");
    let dx = F::from_hex("3D3B0EA90D13082AA6862F0DAC1E211C286614F222BAFE7210862D448EF0E467");
    let dy = F::from_hex("32AB5CACFE5225A4BF684AF87237E34A5A8F8FB4608AD5994F2A327A06513A42");
    let ex = F::from_hex("75E985B20C0DF581F0941168FCE3006628E6F5086125A916031E11F4C1AC1533");
    let ey = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let fx = F::from_hex("0A167A4DF3F20A7E0F6BEE97031CFF99D7190AF79EDA56E9FCE1EE0B3E53EABA");
    let fy = F::from_hex("62E44D27633DD1CB2386748EDE1BA66240A7F428F98B2BE674381AAE4699E851");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    let e = G::new(ex, ey);
    let f = G::new(fx, fy);
    assert_eq!(a + b, c);
    assert_eq!(b + a, c);
    assert_eq!(d + d, e);
    assert_eq!(e + f, G::IDENTITY);
    assert_eq!(G::IDENTITY + c, c);
    assert_eq!(c + G::IDENTITY, c);
    assert_eq!(G::IDENTITY + G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_neg_projective() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupProjective;
    let ax = F::from_hex("2C998F5CD6F89A5323244238DCB0E122F3C48B690D17895D64C622FE7B134873");
    let ay = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let bx = F::from_hex("536670A3290765ACDCDBBDC7234F1EDD0C3B7496F2E876A29B39DD0184ECB77A");
    let by = F::from_hex("3B94477B1521DE0931F76F6CEE88E34BC4E4B581F88D8EAE8616625EC8046C4F");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    assert_eq!(-a, b);
    assert_eq!(-G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_sub_projective() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupProjective;
    let ax = F::from_hex("248949BF1E33E577C48DF9037C0FEDCE42EA070F91125CD796F49349A994794D");
    let ay = F::from_hex("1736DB3E2DA93A423B2DC4E1A010CF1BAB40AF86FAE99E7ABAB19262E335E3FC");
    let bx = F::from_hex("342CFACF5781EFBB03D6326015C9078AAC0FBC7E5F17D6AD71C9BD8D5BB0E41D");
    let by = F::from_hex("3E124FEECFA34E7E6BE1773135B1F5BE7D66E4D22B33243361AF0179EC747917");
    let cx = F::from_hex("329A08756FA031A978BDB2DC8CE0191B67A31A11061AB327DD6BB07FCA2B1637");
    let cy = F::from_hex("1A67928E14B11CB3C91CD549C14AF67810F38F29163459EDC4FA31C85C8F2D10");
    let dx = F::from_hex("124713CD5616381192FDB5BB7868ABA8A48952687874B05F8CA79FFECA50FCB9");
    let dy = F::from_hex("1A0CF87C5FE58923F6C85DA6A9B0D0B812C34CE97CC8374F518D9B4E5B54904D");
    let a = G::new(ax, ay);
    let b = G::new(bx, by);
    let c = G::new(cx, cy);
    let d = G::new(dx, dy);
    assert_eq!(a - b, c);
    assert_eq!(-b + a, c);
    assert_eq!(d - d, G::IDENTITY);
    assert_eq!(G::IDENTITY - c, -c);
    assert_eq!(c - G::IDENTITY, c);
    assert_eq!(G::IDENTITY - G::IDENTITY, G::IDENTITY);
}

#[test]
fn group_mul_projective() {
    type G = blacknet_crypto::edwards25519::Edwards25519GroupProjective;
    let ax = F::from_hex("3AED134ED42AD34F18DB7529FB0ED4470DBB0A157D676ECA74F7789208B87678");
    let ay = F::from_hex("278B8C16CEFF8BD165602933BC9CA4E4365C38F3CE8D17519172D76B8B046621");
    let cx = F::from_hex("24E994BFDF037A840793AC3321DBC483E081150B4C888FE8E6B77CBCCA117411");
    let cy = F::from_hex("64967B5AA7773EB87585C0D81E755B917A6AF58EA2F20E6A12B47D98FB9929F4");
    let b = UInt256::from_hex("0E18DDB951F8A3A10C33028E6CD15A9B4480C3C825F515B6DA24B75E7C813623");
    let d = UInt256::from_hex("251D364ED569CBF14184665CE3FA321E9678002959E04609D1A0ECC692CEE9E1");
    let a = G::new(ax, ay);
    let c = G::new(cx, cy);
    assert_eq!(a * b.bits::<255>(), c);
    assert_eq!(a * UInt256::ONE.bits::<255>(), a);
    assert_eq!(a * UInt256::ZERO.bits::<255>(), G::IDENTITY);
    assert_eq!(G::IDENTITY * d.bits::<255>(), G::IDENTITY);
}
