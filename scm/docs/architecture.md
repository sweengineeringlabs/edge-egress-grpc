# Architecture — edge-transport-grpc-egress

## Sequence

> A domain handler uses `GrpcEgress` to call a downstream gRPC service; `TraceContextInterceptor` propagates the active span before the wire call.

```mermaid
sequenceDiagram
    participant Handler
    participant GrpcEgress
    participant TonicGrpcClient
    participant TraceContextInterceptor
    participant RemoteService

    Handler->>GrpcEgress: call(request, service_name, method)
    GrpcEgress->>TonicGrpcClient: send(request)
    TonicGrpcClient->>TraceContextInterceptor: intercept(outbound)
    TraceContextInterceptor->>TraceContextInterceptor: inject traceparent / tracestate headers
    TraceContextInterceptor-->>TonicGrpcClient: enriched request

    TonicGrpcClient->>RemoteService: gRPC call (mTLS + trace headers)
    RemoteService-->>TonicGrpcClient: tonic::Response<Bytes>

    TonicGrpcClient-->>GrpcEgress: Result<Response, GrpcEgressError>
    GrpcEgress-->>Handler: Result<Resp, GrpcEgressError>
```

## Data Flow

> A typed request enters `GrpcEgress`, is serialised, trace-enriched, and sent over gRPC; the response is deserialised back.

```mermaid
flowchart LR
    A["Request<Payload>\n───────────\nservice_name\nmethod_name\npayload: T"] --> B["TonicGrpcClient\n::send"]
    B --> C["TraceContextInterceptor\ninject W3C trace headers"]
    C --> D["gRPC wire\n(protobuf / JSON bytes\nmTLS optional)"]
    D --> E["Remote gRPC service"]
    E --> F["tonic::Response<Bytes>"]
    F --> G["deserialize<Resp>"]
    G -->|Ok| H["Resp"]
    G -->|Err| I["GrpcEgressError\n::Decode / ::Transport\n::Status(code)"]
```
