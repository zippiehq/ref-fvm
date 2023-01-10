// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
mod bundles;
use bundles::*;
use std::fs::File;
use std::io::prelude::*;
use fvm::executor::{ApplyKind, Executor};
use fvm_integration_tests::dummy::DummyExterns;
use fvm_ipld_blockstore::MemoryBlockstore;
use fvm_ipld_encoding::RawBytes;
use fvm_shared::address::Address;
use fvm_shared::econ::TokenAmount;
use fvm_shared::message::Message;
use fvm_shared::state::StateTreeVersion;
use fvm_shared::version::NetworkVersion;
use num_traits::Zero;
use serde_tuple::*;
use fvm_integration_tests::tester::Account;
use fvm_integration_tests::tester::Tester;
use fvm_ipld_encoding::to_vec;

/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}
#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Eq, Clone, Debug)]
struct Params {
    receipt_data: Vec<u8>,
    method_id: Vec<u8>,
}

#[test]
fn verification() {

    // Instantiate tester
    let mut tester = new_tester(
        NetworkVersion::V18,
        StateTreeVersion::V5,
        MemoryBlockstore::default(),
    )
    .unwrap();

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Get wasm bin
    let wasm_bin = Some(include_bytes!("/home/dymchenko/ref-fvm2/target/debug/wbuild/verify/verify.compact.wasm")).unwrap();


    // Set actor state
    let actor_state = State::default();
    let state_cid = tester.set_state(&actor_state).unwrap();

    // Set actor
    let actor_address = Address::new_id(10000);

    tester
        .set_actor_from_bin(wasm_bin, state_cid, actor_address, TokenAmount::zero())
        .unwrap();

    // Instantiate machine
    tester.instantiate_machine(DummyExterns).unwrap();

    let mut file = File::open("/home/dymchenko/ref-fvm2/receipt").unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();

    // Send message
    let params = Params {
        receipt_data: buffer.to_vec(),
        method_id:hex::decode("0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000049866ff0a61c533c9d7775c66bfe0cd21506b3f4c3f03ce811e1c91dc928ab99404b09e1f36948d9a3c8a728170581827a2e86c2e4cfb725c9cd89543b34858831b1c344d8fa14997492499b27e5544c835724a242899e2ed5aeab3a4d991fdc66eac88a3e917cb7de54f1464abc0e0031e08730574777ccebdb54d2e08af4126982bc47aeab672b3259b948424ff90fcf79a0e1d471a6f7627b8b0cd9b4c0ec77274c12cccca53e49363a19a1c2bbc191e979fe21368d6fe1fe178e0f46a5fc6a6be3bf1d50fe2e1cd711770734d673c4043907629faf699868f385f014720d3a2de544a6bd1fab0d6c6b10d0647bf8f8c41d1cbfdab5a360e82839c0e472626bde9485f6dc062da81c7d1a49a8e155a471f8f22398bb90c612295b8ff31da5").unwrap(),
    };
    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 1000000000,
        method_num: 2,
        params: to_vec(&params).unwrap().into(),
        ..Message::default()
    };

    let res = tester
        .executor
        .unwrap()
        .execute_message(message, ApplyKind::Explicit, 100)
        .unwrap();
    println!("{:#?}", RawBytes::deserialize::<String>(&res.msg_receipt.return_data));
}