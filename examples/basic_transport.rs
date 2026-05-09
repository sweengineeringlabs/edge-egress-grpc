//! Minimal example: create a gRPC transport from config.

use std::sync::Arc;
use swe_edge_egress_grpc::{GrpcChannelConfig, GrpcOutbound, create_transport_from_config};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = GrpcChannelConfig::new("https://localhost:50051");
    let _transport: Arc<dyn GrpcOutbound> = create_transport_from_config(&config)?;
    println!("transport created");
    Ok(())
}
