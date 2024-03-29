use ludex_rust_sdk::challenge::{Challenge, ChallengeResponse, CreateChallengeRequest, FungibleTokenPayout, JoinChallengeRequest, LeaveChallengeRequest, ResolveChallengeRequest, Pot};
use ludex_rust_sdk::ClientScoped;
use once_cell::sync::Lazy;
use std::env;

// Define a lazy static variable to hold the OrganizationScoped instance
static LUDEX_CLIENT_SCOPED: Lazy<ClientScoped> = Lazy::new(|| {
    let api_key = env::var("LUDEX_CLIENT_API_KEY")
        .expect("LUDEX_CLIENT_API_KEY environment variable not set");
    ClientScoped::new(&api_key)
});

#[tokio::test]
// #[ignore]
async fn get_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;
    let response = challenge_scoped.get_challenge(90323).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
// #[ignore]
async fn get_challenges() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;
    let response = challenge_scoped.get_challenges().await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn create_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let challenge: CreateChallengeRequest = CreateChallengeRequest {
        payout_id: 379,
        limit: 2,
        is_verified: true,
    };

    let response = challenge_scoped.create_challenge(challenge).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn generate_join() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let challenge: JoinChallengeRequest = JoinChallengeRequest {
        challenge_id: 90323,
        player_pubkey: String::from("0x9dD82EE27cc23B343f186756771904E0386973f1"),
        // gasless: None,
        // offerings: None,
    };

    let response = challenge_scoped.generate_join(challenge).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn generate_leave() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let leave: LeaveChallengeRequest = LeaveChallengeRequest {
        challenge_id: 90323,
        player_pubkey: String::from("0x9dD82EE27cc23B343f186756771904E0386973f1"),
        // gasless: None,
        // offerings: None,
    };

    let response = challenge_scoped.generate_leave(leave).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn lock_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let response = challenge_scoped.lock_challenge(87805).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn cancel_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let response = challenge_scoped.cancel_challenge(87805).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}

#[tokio::test]
#[ignore]
async fn resolve_challenge() {
    let challenge_scoped: &Challenge = &LUDEX_CLIENT_SCOPED.challenge;

    let challenge: ChallengeResponse = challenge_scoped.get_challenge(87947).await.unwrap();
    println!("Challenge Response {:?}", challenge);

    let total_pot: Vec<Pot> = challenge.total_pot;
    println!("Total pot {:?}", total_pot);

    let resolve_payout: FungibleTokenPayout = FungibleTokenPayout {
        amount: total_pot[0].amount.clone(),
        to: String::from("0x469443b1A8e764543C46F304272273ECBBB1d5E9")
    };

    let resolve_request_body: ResolveChallengeRequest = ResolveChallengeRequest {
        challenge_id: 87947,
        payout: vec![resolve_payout]
    };


    let response = challenge_scoped.resolve_challenge(resolve_request_body).await;

    // cargo test --test integration_test_challenge -- --nocapture
    match &response {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{}", e),
    }

    assert!(response.is_ok());
}