
fn main() {
	let words = download_words().unwrap();
    println!("{:#?}", words.chars().count());
}

#[tokio::main]
async fn download_words() -> Result<String, Box<dyn std::error::Error>> {
	let address = "https://gist.githubusercontent.com/calvinmetcalf/084ab003b295ee70c8fc/raw/314abfdc74b50f45f3dbbfa169892eff08f940f2/wordlist.txt";
    let response = reqwest::get(address)
        .await?
        .text()
		.await?;

    Ok(response)
}