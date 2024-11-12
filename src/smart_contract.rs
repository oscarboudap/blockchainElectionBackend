use substrate_api_client::{Api, XtStatus};
use sp_core::crypto::Pair;
use sp_keyring::AccountKeyring;

pub fn deploy_voting_contract() {
    let url = "ws://127.0.0.1:9944";
    let api = Api::new(url).unwrap().set_signer(AccountKeyring::Alice.pair());

    let xt = api.compose_extrinsic(
        "Contracts",
        "put_code",
        (include_bytes!("../contracts/VotingContract.wasm").to_vec(),)
    );

    let _ = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
}
