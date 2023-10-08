use revm::db::{CacheDB, EmptyDB};

pub struct Provider {
    // db: dyn DatabaseRef<Error = Infallible>,
    pub db: CacheDB<EmptyDB>,
}

impl Default for Provider {
    fn default() -> Self {
        Provider {
            db: CacheDB::new(EmptyDB::default()),
        }
    }
}
