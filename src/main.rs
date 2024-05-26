use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
//use warp::reject::Rejection;
use warp::{http::Response, Filter}; //, reply::Reply};

//use appinsights::TelemetryClient;
//use appinsights::telemetry::SeverityLevel;



#[tokio::main]
async fn main() {
    let example1 = warp::get()
        //.and(warp::path("api"))
        //.and(warp::path("HttpFunction"))
        .and(warp::query::<HashMap<String, String>>())
        .map(|p: HashMap<String, String>| match p.get("text") {
            Some(text) => {
                let x = text.len();
                Response::builder().body(format!("The text you passed is {} characters long.", x))
            },
            None => Response::builder().body(String::from("Pass in some text and the number of characters will be returned. ie. Add '?text=Your text here' to the end of the url")),
        });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
}






/* 
#[tokio::main]
async fn main() {

    // Configure telemetry client with default settings
    let client = TelemetryClient::new("07b5360c-8c4b-41ca-9b7d-bab55cd18d9b".to_string());
    client.track_event("Rust function started");

    async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
         // Configure telemetry client with default settings
        let cli = TelemetryClient::new("07b5360c-8c4b-41ca-9b7d-bab55cd18d9b".to_string());
        cli.track_trace(format!("unhandled rejection: {:?}", err), SeverityLevel::Warning);
        cli.close_channel().await;
        Ok(warp::reply::json(&format!("unhandled rejection: {:?}", err)))
    }


 
    let example1 = warp::get()
        //.and(warp::path("api"))
        //.and(warp::path("HttpFunction"))
        .and(warp::query::<HashMap<String, String>>())
        
        .map(|_|"Does this work?")
        //.map(|p: HashMap<String, String>| match p.get("text") {
        //    Some(text) => {
        //        let x = text.len();
        //        Response::builder().body(format!("The text you passed is {} characters long.", x))
        //    },
        //    None => Response::builder().body(String::from("Pass in some text and the number of characters will be returned. ie. Add '?text=Your text here' to the end of the url")),
        //})
        .recover(handle_rejection);

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    client.track_event("Rust function ending");
    client.close_channel().await;
    warp::serve(example1).run((Ipv4Addr::LOCALHOST, port)).await
    
    
}
*/




