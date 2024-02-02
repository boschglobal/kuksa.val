# kuksa

Kuksa client library.

```toml
[dependencies]
kuksa = "0.1.0"
```

## Basic operation
```rust
fn main() {
    let mut client = kuksa::Client::connect("127.0.0.1:55555");

    let speed: f32 = client.get_value("Vehicle.Speed")?;
    println!("Current speed: {speed}");
    
    let stream = client.subscribe_value("Vehicle.Speed")?;
    while let Some(item) = stream.next() {
        match stream {
            Ok(value) => {
                let speed: f32 = value;
                println!("speed: {speed}");
            },
            Err(err) => {
                println!(err);
            }
        }
    }
}
```