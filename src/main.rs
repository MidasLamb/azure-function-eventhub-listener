use std::env;
use std::net::Ipv4Addr;
use warp::{Filter, Rejection};


#[tokio::main]
async fn main() {
    let example1 = warp::body::json().and_then(|_j: serde_json::Value| async move {
        println!("Message handling!");
        Result::<_, Rejection>::Ok(warp::reply())
    });

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    warp::serve(example1)
        .run((Ipv4Addr::UNSPECIFIED, port))
        .await
}
