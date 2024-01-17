use tokio;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("is this working?");
    return Result::Ok(());
}
