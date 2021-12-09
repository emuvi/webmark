use tl;

pub type WebError = Box<dyn std::error::Error>;

pub async fn make(from: &str, to: &str) -> Result<(), WebError> {
    let res = reqwest::get(from).await?;
    let doc = res.text().await?;
    let dom = tl::parse(&doc, tl::ParserOptions::default());
    for node in dom.nodes() {
        if let Some(tag) = node.as_tag() {
            println!("Tag: {:?} HTML: {:?}", tag.name(), tag.inner_html());
        }
    }
    
    Ok(())
}