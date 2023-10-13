use crate::provider::db::ExecRes;
use crate::Provider;

pub trait Contract {
    fn call(&mut self, provider: &mut Provider, data: Vec<u8>) -> ExecRes;
    fn staticcall(&mut self, provider: &mut Provider, data: Vec<u8>) -> ExecRes;
    fn send(
        &mut self,
        provider: &mut Provider,
        value: revm::primitives::alloy_primitives::Uint<256, 4>,
    ) -> ExecRes;
}
