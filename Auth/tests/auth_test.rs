use mini_tiktok_auth::{
    block_on,
    proto::{
        auth_response::AuthStatusCode, auth_service_client::AuthServiceClient, AuthRequest,
        AuthResponse,
    },
    start_up,
};
use tonic::transport::Channel;

fn wrapper(f: impl std::future::Future<Output = ()>) {
    block_on(async {
        let handle = start_up();

        f.await;

        drop(handle)
    })
    .unwrap()
}

async fn create_client() -> AuthServiceClient<Channel> {
    AuthServiceClient::connect("http://localhost:14514")
        .await
        .unwrap()
}

#[test]
fn auth() {
    wrapper(async {
        let mut channel = create_client().await;

        assert_eq!(
            channel
                .auth(AuthRequest {
                    token: "1919810".into(),
                })
                .await
                .unwrap()
                .into_inner(),
            AuthResponse {
                status_code: AuthStatusCode::Success.into(),
                user_id: 114514,
            }
        );
    })
}

// #[test]
// fn retrive_token() {
//     wrapper(async {
//         let mut channel = create_client().await;

//         assert_eq!(
//             channel
//                 .retrive_token(TokenRequest {
//                     username: "114514".into(),
//                     password: "1919810".into(),
//                 })
//                 .await
//                 .unwrap()
//                 .into_inner(),
//             TokenResponse {
//                 status_code: TokenStatusCode::Success.into(),
//                 token: "1919810".into(),
//                 user_id: 114514
//             }
//         );
//     })
// }
