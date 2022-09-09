#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    #[prost(message, optional, tag = "1")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(
        oneof = "value::Value",
        tags = "11, 12, 13, 14, 15, 16, 17, 18, 21, 22, 23, 24, 25, 26, 27, 28"
    )]
    pub value: ::core::option::Option<value::Value>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        #[prost(string, tag = "11")]
        String(::prost::alloc::string::String),
        #[prost(bool, tag = "12")]
        Bool(bool),
        #[prost(sint32, tag = "13")]
        Int32(i32),
        #[prost(sint64, tag = "14")]
        Int64(i64),
        #[prost(uint32, tag = "15")]
        Uint32(u32),
        #[prost(uint64, tag = "16")]
        Uint64(u64),
        #[prost(float, tag = "17")]
        Float(f32),
        #[prost(double, tag = "18")]
        Double(f64),
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
/// Describes a VSS datapoint
/// When requesting a datapoint, the amount of information returned can
/// be controlled by specifying either a `View` or a set of `Field`s.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Datapoint {
    /// Defines the full VSS path of the datapoint.
    ///
    /// [field: FIELD_PATH]
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// The value of the datapoint
    ///
    /// [field: FIELD_VALUE]
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Value>,
    /// Metadata for this datapoint
    ///
    /// [field: FIELD_METADATA]
    #[prost(message, optional, tag = "10")]
    pub metadata: ::core::option::Option<Metadata>,
    /// Actuator target (only used if the Datapoint is an actuator)
    #[prost(oneof = "datapoint::OptionalActuatorTarget", tags = "3")]
    pub optional_actuator_target: ::core::option::Option<datapoint::OptionalActuatorTarget>,
}
/// Nested message and enum types in `Datapoint`.
pub mod datapoint {
    /// Actuator target (only used if the Datapoint is an actuator)
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalActuatorTarget {
        /// [field: FIELD_ACTUATOR_TARGET]
        #[prost(message, tag = "3")]
        ActuatorTarget(super::Value),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// \[optional\]
    /// Restrict which values are allowed.
    /// Only restrictions matching the DataType {datatype} above is valid/used.
    ///
    /// [field: FIELD_METADATA__VALUE_RESTRICTION]
    #[prost(message, optional, tag = "17")]
    pub value_restriction: ::core::option::Option<ValueRestriction>,
    /// The VSS data type of the entry.
    ///
    /// NOTE: protobuf doesn't have int8, int16, uint8 or uint16 so the actual
    /// value will be serialized as int32 and uint32 respectively
    /// (in the `value` field).
    #[prost(oneof = "metadata::OptionalDataType", tags = "11")]
    pub optional_data_type: ::core::option::Option<metadata::OptionalDataType>,
    /// Describes the meaning and content of the datapoint.
    #[prost(oneof = "metadata::OptionalDescription", tags = "12")]
    pub optional_description: ::core::option::Option<metadata::OptionalDescription>,
    /// Entry type
    #[prost(oneof = "metadata::OptionalEntryType", tags = "13")]
    pub optional_entry_type: ::core::option::Option<metadata::OptionalEntryType>,
    /// \[optional\]
    /// A comment can be used to provide additional informal information
    /// on a datapoint.
    #[prost(oneof = "metadata::OptionalComment", tags = "14")]
    pub optional_comment: ::core::option::Option<metadata::OptionalComment>,
    /// \[optional\]
    /// Whether this datapoint is deprecated.
    #[prost(oneof = "metadata::OptionalDeprecation", tags = "15")]
    pub optional_deprecation: ::core::option::Option<metadata::OptionalDeprecation>,
    /// \[optional\]
    /// The unit of measurement
    #[prost(oneof = "metadata::OptionalUnit", tags = "16")]
    pub optional_unit: ::core::option::Option<metadata::OptionalUnit>,
    /// Entry type specific metadata
    #[prost(oneof = "metadata::EntrySpecific", tags = "20, 30, 40")]
    pub entry_specific: ::core::option::Option<metadata::EntrySpecific>,
}
/// Nested message and enum types in `Metadata`.
pub mod metadata {
    /// The VSS data type of the entry.
    ///
    /// NOTE: protobuf doesn't have int8, int16, uint8 or uint16 so the actual
    /// value will be serialized as int32 and uint32 respectively
    /// (in the `value` field).
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalDataType {
        /// [field: FIELD_METADATA__DATA_TYPE]
        #[prost(enumeration = "super::DataType", tag = "11")]
        DataType(i32),
    }
    /// Describes the meaning and content of the datapoint.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalDescription {
        /// [field: FIELD_METADATA__DESCRIPTION]
        #[prost(string, tag = "12")]
        Description(::prost::alloc::string::String),
    }
    /// Entry type
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalEntryType {
        /// [field: FIELD_METADATA__ENTRY_TYPE]
        #[prost(enumeration = "super::EntryType", tag = "13")]
        EntryType(i32),
    }
    /// \[optional\]
    /// A comment can be used to provide additional informal information
    /// on a datapoint.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalComment {
        /// [field: FIELD_METADATA__COMMENT]
        #[prost(string, tag = "14")]
        Comment(::prost::alloc::string::String),
    }
    /// \[optional\]
    /// Whether this datapoint is deprecated.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalDeprecation {
        /// [field: FIELD_METADATA__DEPRECATION]
        #[prost(string, tag = "15")]
        Deprecation(::prost::alloc::string::String),
    }
    /// \[optional\]
    /// The unit of measurement
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalUnit {
        /// [field: FIELD_METADATA__UNIT]
        #[prost(string, tag = "16")]
        Unit(::prost::alloc::string::String),
    }
    /// Entry type specific metadata
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntrySpecific {
        /// [field: FIELD_METADATA__ACTUATOR]
        #[prost(message, tag = "20")]
        Actuator(super::Actuator),
        /// [field: FIELD_METADATA__SENSOR]
        #[prost(message, tag = "30")]
        Sensor(super::Sensor),
        /// [field: FIELD_METADATA__ATTRIBUTE]
        #[prost(message, tag = "40")]
        Attribute(super::Attribute),
    }
}
////////////////////////
/// Actuator specific fields
///
/// Nothing for now
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actuator {}
/////////////////////////
/// Sensor specific
///
/// Nothing for now
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sensor {}
/////////////////////////
/// Attribute specific
///
/// Nothing for now.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {}
/// Value restriction
///
/// One ValueRestriction{type} for each type, since
/// they don't make sense unless the types match
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestriction {
    #[prost(oneof = "value_restriction::RestrictionType", tags = "21, 22, 23, 24")]
    pub restriction_type: ::core::option::Option<value_restriction::RestrictionType>,
}
/// Nested message and enum types in `ValueRestriction`.
pub mod value_restriction {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum RestrictionType {
        #[prost(message, tag = "21")]
        StringRestriction(super::ValueRestrictionsString),
        /// For signed VSS integers
        #[prost(message, tag = "22")]
        IntRestriciton(super::ValueRestrictionInt),
        /// For unsigned VSS integers
        #[prost(message, tag = "23")]
        UintRestriction(super::ValueRestrictionUint),
        /// For floating point VSS values (float and double)
        #[prost(message, tag = "24")]
        FloatRestriction(super::ValueRestrictionFloat),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestrictionInt {
    #[prost(sint64, repeated, tag = "3")]
    pub allowed_values: ::prost::alloc::vec::Vec<i64>,
    #[prost(oneof = "value_restriction_int::OptionalMin", tags = "1")]
    pub optional_min: ::core::option::Option<value_restriction_int::OptionalMin>,
    #[prost(oneof = "value_restriction_int::OptionalMax", tags = "2")]
    pub optional_max: ::core::option::Option<value_restriction_int::OptionalMax>,
}
/// Nested message and enum types in `ValueRestrictionInt`.
pub mod value_restriction_int {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMin {
        #[prost(sint64, tag = "1")]
        Min(i64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMax {
        #[prost(sint64, tag = "2")]
        Max(i64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestrictionUint {
    #[prost(uint64, repeated, tag = "3")]
    pub allowed_values: ::prost::alloc::vec::Vec<u64>,
    #[prost(oneof = "value_restriction_uint::OptionalMin", tags = "1")]
    pub optional_min: ::core::option::Option<value_restriction_uint::OptionalMin>,
    #[prost(oneof = "value_restriction_uint::OptionalMax", tags = "2")]
    pub optional_max: ::core::option::Option<value_restriction_uint::OptionalMax>,
}
/// Nested message and enum types in `ValueRestrictionUint`.
pub mod value_restriction_uint {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMin {
        #[prost(uint64, tag = "1")]
        Min(u64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMax {
        #[prost(uint64, tag = "2")]
        Max(u64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestrictionFloat {
    /// allowed for doubles/floats not recommended
    #[prost(double, repeated, tag = "3")]
    pub allowed_values: ::prost::alloc::vec::Vec<f64>,
    #[prost(oneof = "value_restriction_float::OptionalMin", tags = "1")]
    pub optional_min: ::core::option::Option<value_restriction_float::OptionalMin>,
    #[prost(oneof = "value_restriction_float::OptionalMax", tags = "2")]
    pub optional_max: ::core::option::Option<value_restriction_float::OptionalMax>,
}
/// Nested message and enum types in `ValueRestrictionFloat`.
pub mod value_restriction_float {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMin {
        #[prost(double, tag = "1")]
        Min(f64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMax {
        #[prost(double, tag = "2")]
        Max(f64),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestrictionDouble {
    #[prost(double, repeated, tag = "3")]
    pub allowed_values: ::prost::alloc::vec::Vec<f64>,
    #[prost(oneof = "value_restriction_double::OptionalMin", tags = "1")]
    pub optional_min: ::core::option::Option<value_restriction_double::OptionalMin>,
    #[prost(oneof = "value_restriction_double::OptionalMax", tags = "2")]
    pub optional_max: ::core::option::Option<value_restriction_double::OptionalMax>,
}
/// Nested message and enum types in `ValueRestrictionDouble`.
pub mod value_restriction_double {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMin {
        #[prost(double, tag = "1")]
        Min(f64),
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum OptionalMax {
        #[prost(double, tag = "2")]
        Max(f64),
    }
}
/// min, max doesn't make much sense for a string
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValueRestrictionsString {
    #[prost(string, repeated, tag = "3")]
    pub allowed_values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Error response shall be an HTTP-like code.
/// Should follow <https://www.w3.org/TR/viss2-transport/#status-codes.>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Error {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    #[prost(string, tag = "2")]
    pub reason: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
/// Used in get/set requests to report errors for specific datapoints
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatapointError {
    /// vss path
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub error: ::core::option::Option<Error>,
}
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
/// VSS Data type of a signal
///
/// Protobuf doesn't support int8, int16, uint8 or uint16.
/// These are mapped to int32 and uint32 respectively.
///
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    Unspecified = 0,
    String = 1,
    Boolean = 2,
    Int8 = 3,
    Int16 = 4,
    Int32 = 5,
    Int64 = 6,
    Uint8 = 7,
    Uint16 = 8,
    Uint32 = 9,
    Uint64 = 10,
    Float = 11,
    Double = 12,
    Timestamp = 13,
    StringArray = 20,
    BooleanArray = 21,
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
}
/// Entry type
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EntryType {
    Unspecified = 0,
    Attribute = 1,
    Sensor = 2,
    Actuator = 3,
}
/// A `View` specifies a set of fields which should
/// be populated in a `Datapoint` (in a response message)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum View {
    /// Unspecified. Equivalent to VIEW_VALUE unless `fields` are explicitly set.
    Unspecified = 0,
    /// Populate Datapoint with value.
    CurrentValue = 1,
    /// Populate Datapoint with actuator target.
    TargetValue = 2,
    /// Populate Datapoint with metadata.
    Metadata = 3,
    /// Populate Datapoint only with requested fields.
    Fields = 10,
    /// Populate Datapoint with everything.
    All = 20,
}
/// A `Field` corresponds to a specific field of a `Datapoint`.
///
/// It can be used to:
///   * populate only specific fields of a `Datapoint` response.
///   * specify which fields of a `Datapoint` should be set as
///     part of a `Set` request.
///   * subscribe to only specific fields of a datapoint.
///   * convey which fields of an updated `Datapoint` have changed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Field {
    /// "*" i.e. everything
    Unspecified = 0,
    /// path
    Path = 1,
    /// value
    Value = 2,
    /// actuator_target
    ActuatorTarget = 3,
    /// metadata.*
    Metadata = 10,
    /// metadata.data_type
    MetadataDataType = 11,
    /// metadata.description
    MetadataDescription = 12,
    /// metadata.entry_type
    MetadataEntryType = 13,
    /// metadata.comment
    MetadataComment = 14,
    /// metadata.deprecation
    MetadataDeprecation = 15,
    /// metadata.unit
    MetadataUnit = 16,
    /// metadata.value_restriction.*
    MetadataValueRestriction = 17,
    /// metadata.actuator.*
    MetadataActuator = 20,
    /// metadata.sensor.*
    MetadataSensor = 30,
    /// metadata.attribute.*
    MetadataAttribute = 40,
}
/// Describes which data we want
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDatapoint {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration = "View", tag = "2")]
    pub view: i32,
    #[prost(enumeration = "Field", tag = "3")]
    pub fields: i32,
}
/// Request a number of VSS datapoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    #[prost(message, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<GetDatapoint>,
}
/// Global errors are specified in `error`.
/// Errors for individual datapoints are specified in `errors`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<Datapoint>,
    #[prost(message, repeated, tag = "2")]
    pub errors: ::prost::alloc::vec::Vec<DatapointError>,
    #[prost(message, optional, tag = "3")]
    pub error: ::core::option::Option<Error>,
}
/// Describes which data we want to set
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetDatapoint {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub datapoint: ::core::option::Option<Datapoint>,
    #[prost(enumeration = "Field", repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<i32>,
}
/// A list of datapoints to be set
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRequest {
    #[prost(message, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<SetDatapoint>,
}
/// Global errors are specified in `error`.
/// Errors for individual datapoints are specified in `errors`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetResponse {
    #[prost(message, optional, tag = "1")]
    pub error: ::core::option::Option<Error>,
    #[prost(message, repeated, tag = "2")]
    pub datapoint_errors: ::prost::alloc::vec::Vec<DatapointError>,
}
/// Describes what to subscribe to
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeDatapoint {
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    #[prost(enumeration = "View", tag = "2")]
    pub view: i32,
    #[prost(enumeration = "Field", repeated, tag = "3")]
    pub fields: ::prost::alloc::vec::Vec<i32>,
}
/// Subscribe to changes in datapoints.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeRequest {
    #[prost(message, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<SubscribeDatapoint>,
}
/// The updated datapoint
/// `fields` specify which fields where changed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdatedDatapoint {
    #[prost(message, optional, tag = "1")]
    pub datapoint: ::core::option::Option<Datapoint>,
    #[prost(enumeration = "Field", repeated, tag = "2")]
    pub fields: ::prost::alloc::vec::Vec<i32>,
}
/// A subscription response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeResponse {
    #[prost(message, repeated, tag = "1")]
    pub datapoints: ::prost::alloc::vec::Vec<UpdatedDatapoint>,
}
/// Nothing yet
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerInfoRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerInfoResponse {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
}
#[doc = r" Generated client implementations."]
pub mod val_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct ValClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ValClient<tonic::transport::Channel> {
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
    impl<T> ValClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> ValClient<InterceptedService<T, F>>
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
            ValClient::new(InterceptedService::new(inner, interceptor))
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
        #[doc = "  Get datapoints"]
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kuksa.val.v1.VAL/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Set datapoints"]
        pub async fn set(
            &mut self,
            request: impl tonic::IntoRequest<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kuksa.val.v1.VAL/Set");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Subscribe to a set of data points"]
        #[doc = ""]
        #[doc = " Returns a stream of replies."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        pub async fn subscribe(
            &mut self,
            request: impl tonic::IntoRequest<super::SubscribeRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::SubscribeResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kuksa.val.v1.VAL/Subscribe");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " Shall return information that allows the client to determine"]
        #[doc = " what server/server implementation/version it is talking to"]
        #[doc = " eg. kuksa-databroker 0.5.1"]
        pub async fn get_server_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetServerInfoRequest>,
        ) -> Result<tonic::Response<super::GetServerInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/kuksa.val.v1.VAL/GetServerInfo");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod val_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ValServer."]
    #[async_trait]
    pub trait Val: Send + Sync + 'static {
        #[doc = "  Get datapoints"]
        async fn get(
            &self,
            request: tonic::Request<super::GetRequest>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        #[doc = " Set datapoints"]
        async fn set(
            &self,
            request: tonic::Request<super::SetRequest>,
        ) -> Result<tonic::Response<super::SetResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the Subscribe method."]
        type SubscribeStream: futures_core::Stream<Item = Result<super::SubscribeResponse, tonic::Status>>
            + Send
            + 'static;
        #[doc = " Subscribe to a set of data points"]
        #[doc = ""]
        #[doc = " Returns a stream of replies."]
        #[doc = ""]
        #[doc = " InvalidArgument is returned if the request is malformed."]
        async fn subscribe(
            &self,
            request: tonic::Request<super::SubscribeRequest>,
        ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status>;
        #[doc = " Shall return information that allows the client to determine"]
        #[doc = " what server/server implementation/version it is talking to"]
        #[doc = " eg. kuksa-databroker 0.5.1"]
        async fn get_server_info(
            &self,
            request: tonic::Request<super::GetServerInfoRequest>,
        ) -> Result<tonic::Response<super::GetServerInfoResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct ValServer<T: Val> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Val> ValServer<T> {
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
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ValServer<T>
    where
        T: Val,
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
                "/kuksa.val.v1.VAL/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Val>(pub Arc<T>);
                    impl<T: Val> tonic::server::UnaryService<super::GetRequest> for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
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
                "/kuksa.val.v1.VAL/Set" => {
                    #[allow(non_camel_case_types)]
                    struct SetSvc<T: Val>(pub Arc<T>);
                    impl<T: Val> tonic::server::UnaryService<super::SetRequest> for SetSvc<T> {
                        type Response = super::SetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetSvc(inner);
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
                "/kuksa.val.v1.VAL/Subscribe" => {
                    #[allow(non_camel_case_types)]
                    struct SubscribeSvc<T: Val>(pub Arc<T>);
                    impl<T: Val> tonic::server::ServerStreamingService<super::SubscribeRequest> for SubscribeSvc<T> {
                        type Response = super::SubscribeResponse;
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
                "/kuksa.val.v1.VAL/GetServerInfo" => {
                    #[allow(non_camel_case_types)]
                    struct GetServerInfoSvc<T: Val>(pub Arc<T>);
                    impl<T: Val> tonic::server::UnaryService<super::GetServerInfoRequest> for GetServerInfoSvc<T> {
                        type Response = super::GetServerInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetServerInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_server_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetServerInfoSvc(inner);
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
    impl<T: Val> Clone for ValServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Val> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Val> tonic::transport::NamedService for ValServer<T> {
        const NAME: &'static str = "kuksa.val.v1.VAL";
    }
}
