use revm::primitives::ruint::Uint;

pub const ADDRESS_ZERO: [u8; 20] = [0; 20];
// TODO const
pub fn ether() -> Uint<256, 4> {
    Uint::from(10).pow(Uint::from(18))
}
