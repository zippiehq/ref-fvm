#[cfg(not(target_arch = "wasm32"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use fvm_ipld_encoding::{RawBytes, to_vec};
use fvm_sdk as sdk;
use fvm_shared::address::Address;
use fvm_shared::bigint::Zero;
use fvm_shared::econ::TokenAmount;
use fvm_shared::error::ExitCode;
use fvm_shared::event::{Entry, Flags};
use serde_tuple::*;
use log::{debug, error, log_enabled, info, Level};
use risc0_zkvm::Receipt;

/// Placeholder invoke for testing
#[no_mangle]
pub fn invoke(blk: u32) -> u32 {
    verify(blk)
}

#[allow(dead_code)]
fn verify(number: u32) -> ! {
    
    let method = sdk::message::method_number();
    let exit_code = match method {
        0 | 1 | 2 => 0,
        _ => 0x42,
    };
    
        let params: Params = {
            let msg_params = sdk::message::params_raw(number).unwrap().unwrap();
            assert_eq!(msg_params.codec, fvm_ipld_encoding::DAG_CBOR);
            fvm_ipld_encoding::from_slice(msg_params.data.clone().as_slice()).unwrap()
        };
    
    let receipt:Receipt = serde_cbor::from_slice(&params.receipt_data).unwrap();   
        
    if receipt.verify(params.method_id.as_slice()).is_ok() {
       sdk::vm::exit(
        fvm_shared::error::ExitCode::OK.value(),
        RawBytes::from(to_vec("verification successfull").unwrap()),
        None,
    )
    }
    else {
        sdk::vm::exit(
            fvm_shared::error::ExitCode::FIRST_USER_EXIT_CODE,
            RawBytes::from(to_vec("verification not successfull").unwrap()),
            None,
        )
    }
    
}

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Eq, Clone, Debug)]
struct Params {
    receipt_data: Vec<u8>,
    method_id: Vec<u8>,
}