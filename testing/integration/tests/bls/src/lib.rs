#[cfg(not(target_arch = "wasm32"))]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use fvm_ipld_encoding::{RawBytes, to_vec, DAG_CBOR};
use fvm_sdk as sdk;
use serde_tuple::*;
use data_encoding::HEXLOWER;
use fvm_ipld_encoding::ipld_block::IpldBlock;
use signature_bls::{PublicKey, Signature};

#[no_mangle]
pub fn invoke(blk: u32) -> u32 {
    on_verify_signature(blk)
}

#[allow(dead_code)]
fn on_verify_signature(number: u32) -> !{
       
    let params: Params = {
        let msg_params = sdk::message::params_raw(number).unwrap().unwrap();
        assert_eq!(msg_params.codec, fvm_ipld_encoding::DAG_CBOR);
        fvm_ipld_encoding::from_slice(msg_params.data.clone().as_slice()).unwrap()
    };

    let signature =  Signature::from_bytes(&HEXLOWER.decode(&params.signature.as_bytes()).unwrap().as_slice().try_into().unwrap()).unwrap();
    let public_key = PublicKey::from_bytes(&HEXLOWER.decode(&params.public_key.as_bytes()).unwrap().as_slice().try_into().unwrap()).unwrap();
    let message = &HEXLOWER.decode(&params.message.as_bytes()).unwrap();
    let res = signature.verify(public_key, message);

    let result:Option<IpldBlock>;
    if res.unwrap_u8() == 1 {
        let message = RawBytes::from(to_vec("verification successfull").unwrap());
        result = (!message.is_empty()).then(|| IpldBlock {
            codec: DAG_CBOR,
            data: message.into(),
        }); 
    }
    else {
        let message = RawBytes::from(to_vec("verification not successfull").unwrap());
        result = (!message.is_empty()).then(|| IpldBlock {
            codec: DAG_CBOR,
            data: message.into(),
        }); 
    }
    sdk::vm::exit(
        fvm_shared::error::ExitCode::FIRST_USER_EXIT_CODE,
        result,
        None,
    )
    
}

#[derive(Serialize_tuple, Deserialize_tuple, PartialEq, Eq, Clone, Debug)]
struct Params {
    public_key: String,
    signature: String,
    message: String,
}