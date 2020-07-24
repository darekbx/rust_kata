use itertools::Itertools;

fn main() {
    let anagramBase = "documenting";
    
    let response = download_words().unwrap();
    let lines:Vec<&str> = response.split("\n").skip(1).collect();
    
    let mut words = Vec::new();

    for line in lines {
        let chunks = line.split_whitespace();
        for chunk in chunks {
            words.push(chunk)
        }
    }

    //let sortedBase = anagramBase.chars().sorted();//.collect::<String>()

    for word1 in &words {
        for word2 in &words {
            word1.to_string().push_str(&word2);
            let sortedWord = word1.chars().sorted();
            
            let sortedBase = anagramBase.chars().sorted();
            if sortedBase.eq(sortedWord) {
                println!("{}{}", word1, word2);
            }
        }
    }

   // println!("{:#?}", sortedBase);
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