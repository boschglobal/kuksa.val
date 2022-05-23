# Vehicle Data Broker

- [Vehicle Data Broker](#vehicle-data-broker)
  - [Intro](#intro)
  - [Interface](#interface)
  - [Relation to the COVESA Vehicle Signal Specification (VSS)](#relation-to-the-covesa-vehicle-signal-specification-vss)
  - [Building](#building)
    - [Build all](#build-all)
    - [Build all release](#build-all-release)
  - [Running](#running)
    - [Broker](#broker)
    - [Test the broker - run client/cli](#test-the-broker---run-clientcli)
    - [Vehicle Data Broker Query Syntax](#vehicle-data-broker-query-syntax)
    - [Configuration](#configuration)
    - [Build and run databroker container](#build-and-run-databroker-container)
  - [Limitations](#limitations)
  - [GRPC overview](#grpc-overview)

## Intro

Vehicle Data Broker is a GRPC service acting as a broker of vehicle data / data points / signals.

## Interface

The main interface, used by clients is defined as follows (see [file](proto/sdv/databroker/v1/broker.proto) in the proto folder):

```proto
service Broker {
    rpc GetDatapoints(GetDatapointsRequest) returns (GetDatapointsReply);
    rpc Subscribe(SubscribeRequest) returns (stream Notification);

    rpc GetMetadata(GetMetadataRequest) returns (GetMetadataReply);
}
```

There is also a [Collector](proto/sdv/databroker/v1/collector.proto) interface which is used by data point providers to feed data into the broker.

```proto
service Collector {
    rpc RegisterDatapoints(RegisterDatapointsRequest) returns (RegisterDatapointsReply);

    rpc UpdateDatapoints(UpdateDatapointsRequest) returns (UpdateDatapointsReply);

    rpc StreamDatapoints(stream StreamDatapointsRequest) returns (stream StreamDatapointsReply);
}
```

## Relation to the COVESA Vehicle Signal Specification (VSS)

The data broker is designed to support data entries and branches as defined by the [VSS](https://covesa.github.io/vehicle_signal_specification/).

In order to generate metadata from a VSS specification that can be loaded by the data broker, it's possible to use the `vspec2json.py` tool
that's available in the VSS repository. E.g.

```shell
./vss-tools/vspec2json.py -I spec -i :uuid.txt spec/VehicleSignalSpecification.vspec vss.json
```

The resulting vss.json can be loaded at startup my supplying the data broker with the command line argument:

```shell
--metadata vss.json
```

## Building

Prerequsites:
- [Rust](https://www.rust-lang.org/tools/install)
- Linux os for build needed (temporary, because of the symlink: proto folder)

### Build all

`cargo build --examples --bins`

### Build all release

`cargo build --examples --bins --release`

## Running

### Broker
Run the broker with:

`cargo run --bin vehicle-data-broker`

Get help, options and version number with:

`cargo run --bin vehicle-data-broker -- -h`

```shell
Vehicle Data Broker

USAGE:
    vehicle-data-broker [OPTIONS]

OPTIONS:
        --address <ADDR>    Bind address [default: 127.0.0.1]
        --port <PORT>       Bind port [default: 55555]
        --metadata <FILE>   Populate data broker with metadata from file [env:
                            VEHICLE_DATA_BROKER_METADATA_FILE=]
        --dummy-metadata    Populate data broker with dummy metadata
    -h, --help              Print help information
    -V, --version           Print version information

```

### Test the broker - run client/cli

Run the cli with:

`cargo run --bin vehicle-data-cli`

To get help and an overview to the offered commands, run the cli and type :

```
client> help
```


If server wasn't running at startup

```
client> connect
```


The server holds the metadata for the available properties, which is fetched on client startup.
This will enable `TAB`-completion for the available properties in the client. Run "metadata" in order to update it.


Get data points by running "get"
```
client> get Vehicle.ADAS.CruiseControl.Error
-> Vehicle.ADAS.CruiseControl.Error: NotAvailable
```

Set data points by running "set"
```
client> set Vehicle.ADAS.CruiseControl.Error Nooooooo!
-> Ok
```

### Vehicle Data Broker Query Syntax

Detailed information about the databroker rule engine can be found in [QUERY.md](doc/QUERY.md)


You can try it out in the client using the subscribe command in the client:

```
client> subscribe
SELECT
  Vehicle.ADAS.ABS.Error
WHERE
  Vehicle.ADAS.ABS.IsActive 

-> status: OK
```

### Configuration

| parameter      | default value | cli parameter    | environment variable              | description                                  |
|----------------|---------------|------------------|-----------------------------------|----------------------------------------------|
| metadata       | <no active>   | --metadata       | VEHICLE_DATA_BROKER_METADATA_FILE | Populate data broker with metadata from file |
| dummy-metadata | <no active>   | --dummy-metadata | <no active>                       | Populate data broker with dummy metadata     |
| listen_address | "127.0.0.1"   | --address        | VEHICLE_DATA_BROKER_ADDR          | Listen for rpc calls                         |
| listen_port    | 55555         | --port           | VEHICLE_DATA_BROKER_PORT          | Listen for rpc calls                         |

To change the default configuration use the arguments during startup see [run section](#running) or environment variables.

### Build and run databroker container

From the terminal, make the vehicle_data_broker as your working directory:

```shell
cd vehicle_data_broker
```

When you are inside the vehicle_data_broker directory, create rust binaries:

```shell
RUSTFLAGS='-C link-arg=-s' cargo build --release --bins --examples

#Use follwing commands for aarch64
cargo install cross

RUSTFLAGS='-C link-arg=-s' cross build --release --bins --examples --target aarch64-unknown-linux-gnu
```
Build tar file from generated binaries.
```shell
# For amd64
tar -czvf bin_release_databroker_x86_64.tar.gz \
    target/release/vehicle-data-cli \
    target/release/vehicle-data-broker \
    target/release/examples/perf_setter \
    target/release/examples/perf_subscriber
```
```shell
# For aarch64
tar -czvf bin_release_databroker_aarch64.tar.gz \
    target/aarch64-unknown-linux-gnu/release/vehicle-data-cli \
    target/aarch64-unknown-linux-gnu/release/vehicle-data-broker \
    target/aarch64-unknown-linux-gnu/release/examples/perf_setter \
    target/aarch64-unknown-linux-gnu/release/examples/perf_subscriber
```
To build the image execute following commands from root directory as context.
```shell
docker build -f vehicle_data_broker/Dockerfile -t databroker:<tag> .

#Use follwing command if buildplatform is required
DOCKER_BUILDKIT=1 docker build -f vehicle_data_broker/Dockerfile -t databroker:<tag> .
```
The image creation may take around 2 minutes.
After the image is created the databroker container can be ran from any directory of the project:
```shell
#By default the container will execute the ./vehicle-data-broker command.
docker run --rm -it  -p 55555:55555/tcp databroker
```
To run any specific command, just append you command at the end.

```shell
docker run --rm -it  -p 55555:55555/tcp databroker <command>
```

## Limitations

- Arrays are not supported in conditions as part of queries (i.e. in the WHERE clause).
- Arrays are not supported by the cli (except for displaying them)

## GRPC overview
This implementation uses GRPC to generate the server & client skeleton and stubs used for communication / (de)serialization.

HTTP/2 over TCP is used for transport with protocol buffers as the serialization format.<br>

A GRPC service uses `.proto` files to specify the services and the data exchanged between server and client.
From this `.proto`, code is generated for the target language (it's available for C#, C++, Dart, Go, Java, Kotlin, Node, Objective-C, PHP, Python, Ruby, Rust...)

The same `.proto` file can be used to generate skelton and stubs for other transports and serialization formats as well.

HTTP/2 is a binary replacement for HTTP/1.1 used for handling connections / multiplexing (channels) & and providing a standardized way to add authorization headers for authorization & TLS for encryption / authentication. It support two way streaming between client and server.

Protobuf is a binary serialization format. There is a lot of tooling around GRPC / protobuf with support in Wireshark to parse what goes on the wire. ![BloomRPC](https://github.com/uw-labs/bloomrpc) can be used to interact with the service using a GUI (by providing the .proto and endpoint).
