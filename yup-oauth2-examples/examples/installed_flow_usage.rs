use tokio::io::{self, AsyncReadExt};

#[tokio::main(flavor = "current_thread")]
async fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).await?;
    let secret = yup_oauth2::parse_application_secret(buffer)?;
    dbg!(secret);

    Ok(())
}
