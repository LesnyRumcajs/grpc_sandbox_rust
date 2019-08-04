use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use protos::adder::AddRequest;
use protos::adder_grpc::AdderClient;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 4 {
        panic!("Expected exactly three arguments, the port to connect to and two numbers to add")
    }
    let port = args[1]
        .parse::<u16>()
        .unwrap_or_else(|_| panic!("{} is not a valid port number", args[1]));

    let num1 = args[2]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("{} is not a valid number", args[1]));

    let num2 = args[3]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("{} is not a valid number", args[1]));

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = AdderClient::new(ch);

    let mut add_req = AddRequest::new();
    add_req.set_first(num1);
    add_req.set_second(num2);

    let check = client.add_numbers(&add_req).expect("RPC Failed!");
    println!("{} + {} = {}", num1, num2, check.get_result());
}
