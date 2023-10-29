use std::thread::{spawn, JoinHandle};

use tokio::runtime;
use tonic::{transport::Server, Request, Response, Status};

mod coop {
    tonic::include_proto!("coop");
}

use coop::coop_server::{Coop, CoopServer};
use coop::Empty;

type GrpcResponse<A> = Result<Response<A>, Status>;

pub fn start_in_background() -> JoinHandle<()> {
    spawn(|| {
        let rt = runtime::Builder::new_multi_thread()
            .thread_name("Grpc")
            .worker_threads(8)
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(start_server());
    })
}

async fn start_server() {
    let addr = "[::1]:50051".parse().unwrap();
    let server = GrpcServer {};
    Server::builder()
        .add_service(CoopServer::new(server))
        .serve(addr)
        .await
        .unwrap();
}
pub struct GrpcServer {}

#[tonic::async_trait]
impl Coop for GrpcServer {
    async fn health(&self, _: Request<Empty>) -> GrpcResponse<Empty> {
        Ok(Response::new(Empty {}))
    }
}
