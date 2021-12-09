use tokio;

mod clip;

use webbase::WebError;

#[tokio::main]
async fn main() -> Result<(), WebError> {
    webbase::make("http://uol.com.br", "").await?;
    Ok(())
}
