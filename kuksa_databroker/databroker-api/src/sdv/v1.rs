#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StringArray {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BoolArray {
    #[prost(bool, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int32Array {
    #[prost(sint32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Int64Array {
    #[prost(sint64, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<i64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint32Array {
    #[prost(uint32, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Uint64Array {
    #[prost(uint64, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FloatArray {
    #[prost(float, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DoubleArray {
    #[prost(double, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Datapoint {
    /// Timestamp of the value
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    /// values
    #[prost(
        oneof = "datapoint::Value",
        tags = "10, 11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28"
    )]
    pub value: ::core::option::Option<datapoint::Value>,
}
/// Nested message and enum types in `Datapoint`.
pub mod datapoint {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Failure {
        /// The data point is known, but doesn't have a valid value
        InvalidValue = 0,
        /// The data point is known, but no value is available
        NotAvailable = 1,
        /// Unknown datapoint
        UnknownDatapoint = 2,
        /// Access denied
        AccessDenied = 3,
        /// Unexpected internal error
        InternalError = 4,
    }
    /// values
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(enumeration = "Failure", tag = "10")]
        FailureValue(i32),
        #[prost(string, tag = "11")]
        StringValue(::prost::alloc::string::String),
        #[prost(bool, tag = "12")]
        BoolValue(bool),
        #[prost(sint32, tag = "13")]
        Int32Value(i32),
        #[prost(sint64, tag = "14")]
        Int64Value(i64),
        #[prost(uint32, tag = "15")]
        Uint32Value(u32),
        #[prost(uint64, tag = "16")]
        Uint64Value(u64),
        #[prost(float, tag = "17")]
        FloatValue(f32),
        #[prost(double, tag = "18")]
        DoubleValue(f64),
        #[prost(message, tag = "21")]
        StringArray(super::StringArray),
        #[prost(message, tag = "22")]
        BoolArray(super::BoolArray),
        #[prost(message, tag = "23")]
        Int32Array(super::Int32Array),
        #[prost(message, tag = "24")]
        Int64Array(super::Int64Array),
        #[prost(message, tag = "25")]
        Uint32Array(super::Uint32Array),
        #[prost(message, tag = "26")]
        Uint64Array(super::Uint64Array),
        #[prost(message, tag = "27")]
        FloatArray(super::FloatArray),
        #[prost(message, tag = "28")]
        DoubleArray(super::DoubleArray),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// Id to be used in "get" and "subscribe" requests. Ids stay valid during
    /// one power cycle, only.
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "DataType", tag = "5")]
    pub data_type: i32,
    /// CONTINUOUS or STATIC or ON_CHANGE
    #[prost(enumeration = "ChangeType", tag = "6")]
    pub change_type: i32,
    #[prost(string, tag = "7")]
    pub description: ::prost::alloc::string::String,
}
/// Data type of a signal
///
/// Protobuf doesn't support int8, int16, uint8 or uint16.
/// These are mapped to sint32 and uint32 respectively.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    String = 0,
    Bool = 1,
    Int8 = 2,
    Int16 = 3,
    Int32 = 4,
    Int64 = 5,
    Uint8 = 6,
    Uint16 = 7,
    Uint32 = 8,
    Uint64 = 9,
    Float = 10,
    Double = 11,
    Timestamp = 12,
    StringArray = 20,
    BoolArray = 21,
    Int8Array = 22,
    Int16Array = 23,
    Int32Array = 24,
    Int64Array = 25,
    Uint8Array = 26,
    Uint16Array = 27,
    Uint32Array = 28,
    Uint64Array = 29,
    FloatArray = 30,
    DoubleArray = 31,
    TimestampArray = 32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatapointError {
    UnknownDatapoint = 0,
    InvalidType = 1,
    AccessDenied = 2,
    InternalError = 3,
    OutOfBounds = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChangeType {
    /// Value never changes
    Static = 0,
    /// Updates are provided every time the value changes (i.e.
    OnChange = 1,
    /// window is open / closed)
    ///
    /// Value is updated continuously. Broker needs to tell
    Continuous = 2,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatapointsRequest {
    /// A list of requested data points.
    #[prost(string, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatapointsReply {
    /// Contains the values of the requested data points.
    /// If a requested data point is not available, the corresponding Datapoint
    /// will have the respective failure value set.
    #[prost(map = "string, message", tag = "1")]
    pub datapoints: ::std::collections::HashMap<::prost::alloc::string::String, Datapoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    /// Subscribe to a set of data points (or expressions) described
    /// by the provided query.
    /// The query syntax is a subset of SQL and is described in more
    /// detail in the QUERY.md file.
    #[prost(string, tag = "2")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeReply {
    /// Contains the fields specified by the query.
    /// If a requested data point value is not available, the corresponding
    /// Datapoint will have it's respective failure value set.
    #[prost(map = "string, message", tag = "1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Datapoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataRequest {
    /// Request metadata for a list of data points referenced by their names.
    /// e.g. "Vehicle.Cabin.Seat.Row1.Pos1.Position" or "Vehicle.Speed".
    ///
    /// If no names are provided, metadata for all known data points will be
    /// returned.
    #[prost(string, repeated, tag = "1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMetadataReply {
    /// Contains metadata of the requested data points. If a data point
    /// doesn't exist (i.e. not known to the Data Broker) the corresponding
    /// Metadata isn't part of the returned list.
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<Metadata>,
}
#[doc = r" Generated client implementations."]
pub mod broker_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct BrokerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BrokerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BrokerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BrokerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BrokerClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Request a set of datapoints (values)"]
        #[doc = ""]
        #[doc = " Returns a list of requested data points."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        pub async fn get_datapoints(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDatapointsRequest>,
        ) -> Result<tonic::Response<super::GetDatapointsReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/sdv.databroker.v1.Broker/GetDatapoints");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Subscribe to a set of data points or conditional expressions"]
        #[doc = " using the Data Broker Query Syntax (described in QUERY.md)"]
        #[doc = ""]
        #[doc = " Returns a stream of replies."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SubscribeReply>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/sdv.databroker.v1.Broker/Subscribe");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Request the metadata of a set of datapoints"]
        #[doc = ""]
        #[doc = " Returns metadata of the requested data points that exist."]
        pub async fn get_metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMetadataRequest>,
        ) -> Result<tonic::Response<super::GetMetadataReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/sdv.databroker.v1.Broker/GetMetadata");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod broker_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BrokerServer."]
    #[async_trait]
    pub trait Broker: Send + Sync + 'static {
        #[doc = " Request a set of datapoints (values)"]
        #[doc = ""]
        #[doc = " Returns a list of requested data points."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        async fn get_datapoints(
            &self,
            request: tonic::Request<super::GetDatapointsRequest>,
        ) -> Result<tonic::Response<super::GetDatapointsReply>, tonic::Status>;
        #[doc = "Server streaming response type for the Subscribe method."]
        type SubscribeStream: futures_core::Stream<Item = Result<super::SubscribeReply, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to a set of data points or conditional expressions"]
        #[doc = " using the Data Broker Query Syntax (described in QUERY.md)"]
        #[doc = ""]
        #[doc = " Returns a stream of replies."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscribeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
        #[doc = " Request the metadata of a set of datapoints"]
        #[doc = ""]
        #[doc = " Returns metadata of the requested data points that exist."]
        async fn get_metadata(
            &self,
            request: tonic::Request<super::GetMetadataRequest>,
        ) -> Result<tonic::Response<super::GetMetadataReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct BrokerServer<T: Broker> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Broker> BrokerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BrokerServer<T>
    where
        T: Broker,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sdv.databroker.v1.Broker/GetDatapoints" => {
                    #[allow(non_camel_case_types)]
                    struct GetDatapointsSvc<T: Broker>(pub Arc<T>);
                    impl<T: Broker> tonic::server::UnaryService<super::GetDatapointsRequest> for GetDatapointsSvc<T> {
                        type Response = super::GetDatapointsReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetDatapointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_datapoints(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetDatapointsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdv.databroker.v1.Broker/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: Broker>(pub Arc<T>);
                    impl<T: Broker> tonic::server::ServerStreamingService<super::SubscribeRequest> for SubscribeSvc<T> {
                        type Response = super::SubscribeReply;
                        type ResponseStream = T::SubscribeStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SubscribeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).subscribe(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SubscribeSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdv.databroker.v1.Broker/GetMetadata" => {
                    #[allow(non_camel_case_types)]
                    struct GetMetadataSvc<T: Broker>(pub Arc<T>);
                    impl<T: Broker> tonic::server::UnaryService<super::GetMetadataRequest> for GetMetadataSvc<T> {
                        type Response = super::GetMetadataReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetMetadataRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_metadata(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetMetadataSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Broker> Clone for BrokerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Broker> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Broker> tonic::transport::NamedService for BrokerServer<T> {
        const NAME: &'static str = "sdv.databroker.v1.Broker";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatapointsRequest {
    #[prost(map = "int32, message", tag = "1")]
    pub datapoints: ::std::collections::HashMap<i32, Datapoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateDatapointsReply {
    /// If empty, everything went well
    #[prost(map = "int32, enumeration(DatapointError)", tag = "1")]
    pub errors: ::std::collections::HashMap<i32, i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamDatapointsRequest {
    #[prost(map = "int32, message", tag = "1")]
    pub datapoints: ::std::collections::HashMap<i32, Datapoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamDatapointsReply {
    /// If empty, everything went well
    #[prost(map = "int32, enumeration(DatapointError)", tag = "1")]
    pub errors: ::std::collections::HashMap<i32, i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatapointsRequest {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<RegistrationMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistrationMetadata {
    /// Name of the data point
    /// (e.g. "Vehicle.Cabin.Seat.Row1.Pos1.Position" or "Vehicle.Speed")
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "DataType", tag = "2")]
    pub data_type: i32,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(enumeration = "ChangeType", tag = "4")]
    pub change_type: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterDatapointsReply {
    /// Maps each data point name passed in RegisterDatapointsRequest to a data point id
    #[prost(map = "string, int32", tag = "1")]
    pub results: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
#[doc = r" Generated client implementations."]
pub mod collector_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CollectorClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CollectorClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CollectorClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CollectorClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            CollectorClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Register new datapoint (metadata)"]
        #[doc = ""]
        #[doc = " If the registration of at least one of the passed data point fails, the overall registration"]
        #[doc = " is rejected and the gRPC status code ABORTED is returned (to indicate the \"aborted\" registration)."]
        #[doc = " The details, which data point(s) caused the failure and the reason, is passed in back in human-"]
        #[doc = " readable form in the status message. Possible failure resaons are:"]
        #[doc = "  * PERMISSION_DENIED - Not allowed to register this name"]
        #[doc = "  * ALREADY_REGISTERED - The data point is already registered by some other feeder"]
        #[doc = "  * RE_REGISTRATION_MISMATCH - Already registered by this feeder but with differing metadata"]
        #[doc = "  * INVALID_NAME - The passed name of the datapoint has an invalid structure"]
        #[doc = "  * INVALID_VALUE_TYPE - The passed ValueType is not supported"]
        #[doc = "  * INVALID_CHANGE_TYPE - The passed ChangeType is not supported"]
        pub async fn register_datapoints(
            &mut self,
            request: impl tonic::IntoRequest<super::RegisterDatapointsRequest>,
        ) -> Result<tonic::Response<super::RegisterDatapointsReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdv.databroker.v1.Collector/RegisterDatapoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Provide a set of updated datapoint values to the broker."]
        #[doc = " This is the unary equivalent of `StreamDatapoints` below and is better suited for cases"]
        #[doc = " where the frequency of updates is rather low."]
        #[doc = ""]
        #[doc = " NOTE: The values provided in a single request are handled as a single update in the"]
        #[doc = " data broker. This ensures that any clients requesting (or subscribing to) a set of"]
        #[doc = " datapoints will get a consistent update, i.e. that either all values are updated or"]
        #[doc = " none are."]
        #[doc = ""]
        #[doc = " Returns: any errors encountered updating the datapoints"]
        #[doc = ""]
        pub async fn update_datapoints(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateDatapointsRequest>,
        ) -> Result<tonic::Response<super::UpdateDatapointsReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdv.databroker.v1.Collector/UpdateDatapoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Provide a stream with updated datapoint values to the broker."]
        #[doc = " This is the streaming equivalent of `UpdateDatapoints` above and is better suited for"]
        #[doc = " cases where the frequency of updates is high."]
        #[doc = ""]
        #[doc = " NOTE: The values provided in a single request are handled as a single update in the"]
        #[doc = " data broker. This ensures that any clients requesting (or subscribing to) a set of"]
        #[doc = " datapoints will get a consistent update, i.e. that either all values are updated or"]
        #[doc = " none are."]
        #[doc = ""]
        #[doc = " Returns: any errors encountered updating the datapoints"]
        #[doc = ""]
        pub async fn stream_datapoints(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamDatapointsRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamDatapointsReply>>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/sdv.databroker.v1.Collector/StreamDatapoints",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod collector_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with CollectorServer."]
    #[async_trait]
    pub trait Collector: Send + Sync + 'static {
        #[doc = " Register new datapoint (metadata)"]
        #[doc = ""]
        #[doc = " If the registration of at least one of the passed data point fails, the overall registration"]
        #[doc = " is rejected and the gRPC status code ABORTED is returned (to indicate the \"aborted\" registration)."]
        #[doc = " The details, which data point(s) caused the failure and the reason, is passed in back in human-"]
        #[doc = " readable form in the status message. Possible failure resaons are:"]
        #[doc = "  * PERMISSION_DENIED - Not allowed to register this name"]
        #[doc = "  * ALREADY_REGISTERED - The data point is already registered by some other feeder"]
        #[doc = "  * RE_REGISTRATION_MISMATCH - Already registered by this feeder but with differing metadata"]
        #[doc = "  * INVALID_NAME - The passed name of the datapoint has an invalid structure"]
        #[doc = "  * INVALID_VALUE_TYPE - The passed ValueType is not supported"]
        #[doc = "  * INVALID_CHANGE_TYPE - The passed ChangeType is not supported"]
        async fn register_datapoints(
            &self,
            request: tonic::Request<super::RegisterDatapointsRequest>,
        ) -> Result<tonic::Response<super::RegisterDatapointsReply>, tonic::Status>;
        #[doc = " Provide a set of updated datapoint values to the broker."]
        #[doc = " This is the unary equivalent of `StreamDatapoints` below and is better suited for cases"]
        #[doc = " where the frequency of updates is rather low."]
        #[doc = ""]
        #[doc = " NOTE: The values provided in a single request are handled as a single update in the"]
        #[doc = " data broker. This ensures that any clients requesting (or subscribing to) a set of"]
        #[doc = " datapoints will get a consistent update, i.e. that either all values are updated or"]
        #[doc = " none are."]
        #[doc = ""]
        #[doc = " Returns: any errors encountered updating the datapoints"]
        #[doc = ""]
        async fn update_datapoints(
            &self,
            request: tonic::Request<super::UpdateDatapointsRequest>,
        ) -> Result<tonic::Response<super::UpdateDatapointsReply>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamDatapoints method."]
        type StreamDatapointsStream: futures_core::Stream<Item = Result<super::StreamDatapointsReply, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Provide a stream with updated datapoint values to the broker."]
        #[doc = " This is the streaming equivalent of `UpdateDatapoints` above and is better suited for"]
        #[doc = " cases where the frequency of updates is high."]
        #[doc = ""]
        #[doc = " NOTE: The values provided in a single request are handled as a single update in the"]
        #[doc = " data broker. This ensures that any clients requesting (or subscribing to) a set of"]
        #[doc = " datapoints will get a consistent update, i.e. that either all values are updated or"]
        #[doc = " none are."]
        #[doc = ""]
        #[doc = " Returns: any errors encountered updating the datapoints"]
        #[doc = ""]
        async fn stream_datapoints(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamDatapointsRequest>>,
        ) -> Result<tonic::Response<Self::StreamDatapointsStream>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CollectorServer<T: Collector> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Collector> CollectorServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CollectorServer<T>
    where
        T: Collector,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/sdv.databroker.v1.Collector/RegisterDatapoints" => {
                    #[allow(non_camel_case_types)]
                    struct RegisterDatapointsSvc<T: Collector>(pub Arc<T>);
                    impl<T: Collector> tonic::server::UnaryService<super::RegisterDatapointsRequest>
                        for RegisterDatapointsSvc<T>
                    {
                        type Response = super::RegisterDatapointsReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RegisterDatapointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).register_datapoints(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RegisterDatapointsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdv.databroker.v1.Collector/UpdateDatapoints" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateDatapointsSvc<T: Collector>(pub Arc<T>);
                    impl<T: Collector> tonic::server::UnaryService<super::UpdateDatapointsRequest>
                        for UpdateDatapointsSvc<T>
                    {
                        type Response = super::UpdateDatapointsReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateDatapointsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update_datapoints(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateDatapointsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/sdv.databroker.v1.Collector/StreamDatapoints" => {
                    #[allow(non_camel_case_types)]
                    struct StreamDatapointsSvc<T: Collector>(pub Arc<T>);
                    impl<T: Collector>
                        tonic::server::StreamingService<super::StreamDatapointsRequest>
                        for StreamDatapointsSvc<T>
                    {
                        type Response = super::StreamDatapointsReply;
                        type ResponseStream = T::StreamDatapointsStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::StreamDatapointsRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_datapoints(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamDatapointsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Collector> Clone for CollectorServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Collector> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Collector> tonic::transport::NamedService for CollectorServer<T> {
        const NAME: &'static str = "sdv.databroker.v1.Collector";
    }
}
