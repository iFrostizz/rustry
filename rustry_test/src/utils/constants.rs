use revm::primitives::ruint::Uint;
use revm::primitives::Bytes;

pub const ADDRESS_ZERO: [u8; 20] = [0; 20];
// TODO const
pub fn ether() -> Uint<256, 4> {
    Uint::from(10).pow(Uint::from(18))
}
// TODO const
pub fn bytes_zero() -> Bytes {
    Bytes::from_static(&[0u8; 32])
}
