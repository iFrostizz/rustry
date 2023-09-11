use ethers::{
    middleware::SignerMiddleware,
    prelude::{abigen, ContractFactory},
    providers::{Http, Provider},
    signers::{LocalWallet, Signer},
    solc::{Artifact, Project, ProjectPathsConfig},
    utils::{Anvil, AnvilInstance},
};
use rstest::*;
use std::{path::PathBuf, sync::Arc};
// use rustry::utils::{constants::ADDRESS_ZERO, deploy_contract};

abigen!(SimpleStore, "./examples/SimpleStore.json");

type Middleware = SignerMiddleware<Provider<Http>, LocalWallet>;

#[fixture]
async fn set_up() -> (AnvilInstance, SimpleStore<Middleware>) {
    let anvil = Anvil::default().spawn();
    let wallet: LocalWallet = anvil.keys()[0].clone().into();

    let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
    let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
    let client = Arc::new(client);

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
    let paths = ProjectPathsConfig::builder()
        .root(&root)
        .sources(&root)
        .build()
        .unwrap();
    let project = Project::builder()
        .paths(paths)
        .ephemeral()
        .no_artifacts()
        .build()
        .unwrap();
    let output = project.compile().unwrap();
    let contract = output
        .find_first("SimpleStore")
        .expect("could not find contract")
        .clone();
    let (abi, bytecode, _) = contract.into_parts();
    let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
    let contract = factory.deploy(()).unwrap().send().await.unwrap();
    let address = contract.address();
    let simple_store = SimpleStore::new(address, client);

    (anvil, simple_store)
}

#[rstest]
async fn test_deployment(#[future] set_up: (AnvilInstance, SimpleStore<Middleware>)) {
    let addr = set_up.await.1.address();
    assert_ne!(addr, H160::zero());
}

#[rstest]
async fn test_store(#[future] set_up: (AnvilInstance, SimpleStore<Middleware>)) {
    let set_up = set_up.await;
    let to_store = U256::from(69);
    let (anvil, simple_store) = set_up;
    simple_store.set(to_store).send().await.unwrap();
    let number = simple_store.get().call().await.unwrap();
    assert_eq!(number, to_store);
}

fn main() {}
