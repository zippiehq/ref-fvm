// Copyright 2021-2023 Protocol Labs
// SPDX-License-Identifier: Apache-2.0, MIT
mod bundles;
use bundles::*;
use bls::WASM_BINARY;
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
use fvm_ipld_encoding::to_vec;
/// The state object.
#[derive(Serialize_tuple, Deserialize_tuple, Clone, Debug, Default)]
pub struct State {
    pub count: u64,
}
#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Eq, Clone, Debug)]
struct Params {
    public_key: String,
    signature: String,
    message: String,
}

#[test]
fn bls() {

    // Instantiate tester
    let mut tester = new_tester(
        NetworkVersion::V18,
        StateTreeVersion::V5,
        MemoryBlockstore::default(),
    )
    .unwrap();

    let sender: [Account; 1] = tester.create_accounts().unwrap();

    // Get wasm bin
    let wasm_bin = WASM_BINARY.unwrap();
    //Some(include_bytes!("/home/dymchenko/ref-fvm2/target/debug/wbuild/bls/bls.compact.wasm")).unwrap();

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

    let params = Params {
        public_key: "a2359fa8a69cc2bc8c71017402fc9faf43159354173c72822c0b4685c2c0acd4cfb06349dc883c7c23150f93eeb390780b486303eea37af882f62cb3ab5cecd03dffbbb8a11f99950007764da9c730d2e1604b4f7e4a1c570aa88f84e3bd22c1".to_string(),
        signature: "8613038c1684b048b63328b35d31fe14a144010316fe25bde58a268674ab3ae5bfdb910c9fa7c347e4c3d4063d305c8c".to_string(),
        message: "080907000102040506070809020c0304080907000102040506070809020c0304".to_string(),
    };

    let message = Message {
        from: sender[0].1,
        to: actor_address,
        gas_limit: 10000000000,
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