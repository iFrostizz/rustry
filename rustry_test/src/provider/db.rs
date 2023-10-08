use revm::{
    db::{CacheDB, DbAccount, EmptyDB},
    primitives::{create_address, AccountInfo, Address, Bytecode, U256},
};
use std::convert::Infallible;

pub struct Provider {
    // db: dyn DatabaseRef<Error = Infallible>,
    db: CacheDB<EmptyDB>,
    /// The active sender
    pub sender: Address,
}

impl Default for Provider {
    fn default() -> Self {
        Provider {
            db: CacheDB::new(EmptyDB::default()),
            sender: Address::ZERO,
        }
    }
}

impl Provider {
    fn deploy_code(&mut self, from: Address, code: Bytecode) -> Result<Address, Infallible> {
        let account = self.db.load_account(from)?;
        let nonce = if let Some(info) = account.info() {
            info.nonce
        } else {
            0
        };
        let new_address = create_address(from, nonce);
        let mut new_account = AccountInfo::new(U256::ZERO, 0, code.hash_slow(), code);
        self.db.insert_contract(&mut new_account);

        Ok(new_address)
    }

    pub fn deploy(&mut self, code: Bytecode) -> Result<Address, Infallible> {
        self.deploy_code(self.sender, code)
    }

    pub fn sender_state(&mut self) -> Result<DbAccount, Infallible> {
        self.db.load_account(self.sender).cloned()
    }
}

pub trait Cheats {
    fn impersonate(&mut self, who: Address);
}

impl Cheats for Provider {
    fn impersonate(&mut self, who: Address) {
        self.sender = who
    }
}
