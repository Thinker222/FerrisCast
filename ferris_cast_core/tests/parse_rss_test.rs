use ferris_cast_core::{parse_rss, Podcast};
use std::path::PathBuf;


#[test]
fn is_ok() {
    let buf = PathBuf::from("./tests/assets/test_rss.xml");
    let result = parse_rss(&buf);
    match result {
        Err(_) => assert!(false, "parse rss did not return ok"),
        _ => ()
    }
}

#[test]
fn is_not_ok() {
    let buf = PathBuf::from("./tests/assets/test_not_real_rss.xml");
    let result = parse_rss(&buf);
    match result {
        Ok(_) => assert!(false, "parse rss should not return ok"),
        _ => ()
    }
}

#[allow(dead_code)]
fn make_me_podcast() -> Podcast {
    let buf = PathBuf::from("./tests/assets/test_rss.xml");
    let result = parse_rss(&buf);
    result.expect("Failed to open xml for test")
}

#[tokio::test]
async fn test_get_rss()  -> Result<(), Box<dyn std::error::Error>> {
    let shapiro_link: String = String::from("https://feeds.megaphone.fm/WWO8086402096");
    let content = reqwest::get(&shapiro_link)
    .await?
    .bytes()
    .await?;
    let channel = rss::Channel::read_from(&content[..])?;
    println!("{:?}", channel.namespaces());
    Ok(())
}