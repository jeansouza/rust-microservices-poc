use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tokio::time::{sleep, Duration};
use tonic::{Request, Response};
use uuid::Uuid;

use crate::authentication::{StatusCode, SignOutResponse};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_HOST_NAME will be set to 'auth' when running the health check service in Docker
    // ::0 is required for Docker to work: https://stackoverflow.com/questions/59179831/docker-app-server-ip-address-127-0-0-1-difference-of-0-0-0-0-ip
    let auth_hostname = env::var("AUTH_SERVICE_HOST_NAME").unwrap_or("[::0]".to_owned());

    // Establish connection when auth service
    let mut client = AuthClient::connect(format!("http://{}:50051", auth_hostname)).await?;

    loop {
        let username = Uuid::new_v4().to_string();
        let password = Uuid::new_v4().to_string();

        let request = Request::new(SignUpRequest {
            password: password.clone(),
            username: username.clone(),
        });

        let response = client.sign_up(request).await?;

        // Log the response
        println!(
            "SIGN UP RESPONSE STATUS: {:?}",
            StatusCode::from_i32(response.into_inner().status_code)
        );

        // ---------------------------------------------

        let request = Request::new(SignInRequest {
            password: password.clone(),
            username: username.clone(),
        }); 

        let response = (client.sign_in(request).await?).into_inner();

        println!(
            "SIGN IN RESPONSE STATUS: {:?}",
            response.status_code,
        );

        // ---------------------------------------------

        let request = Request::new(SignOutRequest {
            session_token: response.session_token,
        });

        let response: Response<SignOutResponse> = client.sign_out(request).await?;

        println!(
            "SIGN OUT RESPONSE STATUS: {:?}",
            response.into_inner().status_code,
        );

        println!("--------------------------------------",);

        sleep(Duration::from_secs(3)).await;
    }
}
