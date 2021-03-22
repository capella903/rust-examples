use std::path::PathBuf;
use structopt::StructOpt;
use yup_oauth2::{InstalledFlowAuthenticator, InstalledFlowReturnMethod};

#[derive(Debug, StructOpt)]
struct Opt {
    path: PathBuf,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let secret = yup_oauth2::read_application_secret(opt.path).await?;

    let auth = InstalledFlowAuthenticator::builder(secret, InstalledFlowReturnMethod::Interactive)
        .persist_tokens_to_disk("token.json")
        .build()
        .await?;

    let scopes = &["https://www.googleapis.com/auth/calendar.readonly"];

    let token = auth.token(scopes).await?;

    dbg!(token);

    Ok(())
}
