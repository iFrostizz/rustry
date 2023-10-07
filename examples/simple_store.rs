// use ethers::{
//     middleware::SignerMiddleware,
//     prelude::{abigen, ContractFactory},
//     providers::{Http, Provider},
//     signers::{LocalWallet, Signer},
//     solc::{Artifact, Project, ProjectPathsConfig},
//     types::{H160, U256},
//     utils::Anvil,
// };
// use rustry::rustry_test;
// use std::{path::PathBuf, sync::Arc};

// // abigen!(SimpleStore, "./examples/simple_store/SimpleStore.json");

// // type Middleware = SignerMiddleware<Provider<Http>, LocalWallet>;

// async fn set_up() {
//     // let anvil = Anvil::default().spawn();
//     // let wallet: LocalWallet = anvil.keys()[0].clone().into();

//     // let provider = Provider::<Http>::try_from(anvil.endpoint()).unwrap();
//     // let client = SignerMiddleware::new(provider, wallet.with_chain_id(anvil.chain_id()));
//     // let client = Arc::new(client);

//     // let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("examples");
//     // let paths = ProjectPathsConfig::builder()
//     //     .root(&root)
//     //     .sources(&root)
//     //     .build()
//     //     .unwrap();
//     // let project = Project::builder()
//     //     .paths(paths)
//     //     .ephemeral()
//     //     .no_artifacts()
//     //     .build()
//     //     .unwrap();
//     // let output = project.compile().unwrap();
//     // let contract = output
//     //     .find_first("SimpleStore")
//     //     .expect("could not find contract")
//     //     .clone();
//     // let (abi, bytecode, _) = contract.into_parts();
//     // let factory = ContractFactory::new(abi.unwrap(), bytecode.unwrap(), client.clone());
//     // let contract = factory.deploy(()).unwrap().send().await.unwrap();
//     // let address = contract.address();
//     // let simple_store = SimpleStore::new(address, client);
//     let value = 4;
// }

// #[rustry_test]
// fn test_deployment() {
//     assert_eq!(2 + 2, value);
//     // assert_ne!(simple_store.address(), H160::zero());
// }

// #[rustry_test]
// fn test_store() {
//     let to_store = U256::from(69);
//     // simple_store.set(to_store).send().await.unwrap();
//     // let number = simple_store.get().call().await.unwrap();
//     // assert_eq!(number, to_store);
// }

// fn main() {}
