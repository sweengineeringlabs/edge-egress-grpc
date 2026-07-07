//! Tonic-backed load-balancer SPI adapter.
pub(crate) mod lb_grpc_client;
pub(crate) use lb_grpc_client::TonicLbGrpcClient;
