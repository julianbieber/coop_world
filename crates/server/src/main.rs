mod grpc_server;

fn main() {
    let grpc_handle = grpc_server::start_in_background();
    println!("Hello, world!");

    grpc_handle.join().unwrap();
}
