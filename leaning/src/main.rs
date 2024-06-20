use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Person {
    name: String,
    age: u8,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // First open, in read mode
    let mut file = tokio::fs::OpenOptions::new()
        .read(true)
        .open("./test.json")
        .await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let root: Vec<Person> = serde_json::from_str(&contents)?;
    for person in root {
        println!("{person:?}")
    }
    // Second open, in write mode
    let mut file = tokio::fs::OpenOptions::new()
        .write(true)
        .open("./test.json")
        .await?;
    // Test object to write
    let input = vec![Person {
        name: "Gerald".to_string(),
        age: 2,
    }];
    println!("{input:?}");
    // Serialization
    let string = serde_json::to_string(&input).expect("Improper serialization");
    // Write
    let _ = file.write(string.as_bytes()).await?;
    Ok(())
}
