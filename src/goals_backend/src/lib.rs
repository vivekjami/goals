#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

// import management canister
use ic_cdk::api::management_canister::http_request::{http_request, CanisterHttpRequestArgument, HttpMethod};

// update method using the HTTPS outcalls feature
#[ic_cdk::update]
async fn get_goals() -> String {

    // setup the URL for the HTTP GET request
    let url = "https://65b896bbb71048505a88e8ff.mockapi.io/goals".to_string();

    // prepare headers for the system http_request call
    let request_headers = vec![];

    // setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };

    // make the HTTPS request and wait for the response
    // 1_603_079_600 cycles are needed for this operation
    match http_request(request,).await {
        Ok((response,)) => { 
            String::from_utf8(response.body).expect("Transformed response is not UTF-8 encoded.")
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}

