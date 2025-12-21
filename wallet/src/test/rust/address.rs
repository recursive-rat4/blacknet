/*
 * Copyright (c) 2020-2025 Pavel Vasin
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
use blacknet_kernel::ed25519::PublicKey;
use blacknet_wallet::address::{AddressCodec, AddressKind};
use data_encoding::HEXUPPER;

#[test]
fn account() {
    let string = "blacknet1klnycx794hg9jvuhua0gy75d5v374rrwrlnpg25xpykfxkg30egqq83tj0";
    let bytes: PublicKey = HEXUPPER
        .decode(b"B7E64C1BC5ADD0593397E75E827A8DA323EA8C6E1FE6142A86092C9359117E50")
        .unwrap()
        .try_into()
        .unwrap();
    let address_codec = AddressCodec::new(&Mode::mainnet()).unwrap();
    assert_eq!(address_codec.encode(bytes).unwrap(), string);
    assert_eq!(address_codec.decode(string).unwrap(), bytes);
}

#[test]
fn htlc() {
    let string = "blacknet1q8llal0ul0a0n78h7m6lfulj78cwlmhdan47460gulnwte8ruts7q6rcsw5";
    let address_codec = AddressCodec::new(&Mode::mainnet()).unwrap();
    let address = address_codec
        .decode_with_kind(AddressKind::HTLC, string)
        .unwrap();
    assert_eq!(
        address_codec
            .encode_with_kind(AddressKind::HTLC, &address)
            .unwrap(),
        string
    );
}

#[test]
fn multisig() {
    let string = "blacknet1qtl0ml8mltul3alk7h608uh37rh7am0va04wn688umj7fclzu8sd7467cge";
    let address_codec = AddressCodec::new(&Mode::mainnet()).unwrap();
    let address = address_codec
        .decode_with_kind(AddressKind::Multisig, string)
        .unwrap();
    assert_eq!(
        address_codec
            .encode_with_kind(AddressKind::Multisig, &address)
            .unwrap(),
        string
    );
}

#[test]
fn blob() {
    let string = "blacknet1q07le7l69mvwv3";
    let address_codec = AddressCodec::new(&Mode::mainnet()).unwrap();
    let address = address_codec
        .decode_with_kind(AddressKind::Blob, string)
        .unwrap();
    assert_eq!(
        address_codec
            .encode_with_kind(AddressKind::Blob, &address)
            .unwrap(),
        string
    );
}
