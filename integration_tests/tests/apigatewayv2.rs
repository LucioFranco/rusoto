#![cfg(feature = "apigatewayv2")]

extern crate rusoto_apigatewayv2;
extern crate rusoto_core;

use rusoto_apigatewayv2::{ApiGatewayV2, ApiGatewayV2Client};
use rusoto_core::Region;

#[test]
fn should_work() {
    let client = ApiGatewayV2Client::new(Region::UsEast1);
    let response = client
        .get_apis(Default::default())
        .sync()
        .expect("expected an ok response");
    println!("response is {:#?}", response);
}
