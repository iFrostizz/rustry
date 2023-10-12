use crate::provider::db::ExecRes;

pub trait Contract {
    fn call(&mut self, data: Vec<u8>) -> ExecRes;
    fn staticcall(&mut self, data: Vec<u8>) -> ExecRes;
    fn send(
        &mut self,
        value: revm::primitives::alloy_primitives::Uint<256, 4>,
    ) -> ExecRes;
}
