use substrate_api_client::{Api, XtStatus};
use sp_core::crypto::Pair;
use sp_keyring::AccountKeyring;

pub fn submit_vote(vote_data: &str) {
    let url = "ws://127.0.0.1:9944"; // URL for Substrate node
    let api = Api::new(url).unwrap().set_signer(AccountKeyring::Alice.pair());

    let xt = api.compose_extrinsic(
        "VotingModule",
        "submit_vote",
        (vote_data,)
    );

    let _ = api.send_extrinsic(xt.hex_encode(), XtStatus::InBlock).unwrap();
}
