use tokio;

mod clip;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut list: Vec<&'static str> = Vec::new();
    list.push("https://www.google.com");
    let tst = reqwest::get(list.pop().unwrap());
    let sdb = async {
        let res = reqwest::get("https://www.google.com").await?;
        println!("Status: {}", res.status());
        let body = res.text().await?;
        println!("Body:\n\n{}", body);
        let rst: Result<(), reqwest::Error> = Ok(());
        rst
    };
    let handle = tokio::spawn(sdb);
    Ok(())
}
