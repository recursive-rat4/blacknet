import * as nacl from './nacl.js';
import * as bip39 from './bip39.js';
import * as bech32_addr from './bech32_addr.js';

const blacknet_seedWords = 12;
const blacknet_secretKeyLength = nacl.box_secretKeyLength;
const blacknet_bech32_hrp = "blacknet";

function strToArray(str) {
  return new TextEncoder("utf-8").encode(str);
}

export function blacknet_sk_check_version(sk) {
  return (sk[0] & 0xF0) == 0x10;
}

export function blacknet_mnemonic_sk(mnemonic) {
  let sk = nacl.hash(strToArray(mnemonic), blacknet_secretKeyLength);
  return blacknet_sk_check_version(sk) ? sk : null;
}

export function blacknet_mnemonic_keypair(mnemonic) {
  let sk = blacknet_mnemonic_sk(mnemonic);
  if (!sk)
    return null;
  return nacl.sign_keyPair_fromSeed(sk);
}

export function blacknet_mnemonic_check_version(mnemonic) {
  return blacknet_mnemonic_sk(mnemonic) != null;
}

export function blacknet_mnemonic() {
  let seed = new Uint16Array(blacknet_seedWords);
  let mnemonic = "";

  while (true) {
    crypto.getRandomValues(seed);
    for (let i = 0; i < blacknet_seedWords; i++) {
      mnemonic += bip39.english[seed[i] % 2048];
      if (i < blacknet_seedWords - 1) mnemonic += " ";
    }
    if (blacknet_mnemonic_check_version(mnemonic))
      break;
    mnemonic = "";
  }

  nacl.cleanup(seed);
  return mnemonic;
}

export function blacknet_pk_account(pk) {
  return bech32_addr.encode(blacknet_bech32_hrp, Array.from(pk));
}

window.blacknet_mnemonic = blacknet_mnemonic;
window.blacknet_mnemonic_keypair = blacknet_mnemonic_keypair;
window.blacknet_pk_account = blacknet_pk_account;
