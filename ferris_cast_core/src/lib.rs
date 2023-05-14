use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use rss::{Channel,Item};
use std::convert::From;
use std::vec::Vec;

pub struct Podcast {
    pub title: String,
    pub link: String,
    pub image: rss::Image, // TODO: If necessary add an image if needed
    pub episodes: Vec<Item>, // TODO: Implement episode type
    pub last_build_date: String
}

impl From<Channel> for Podcast {
    fn from(channel : Channel) -> Self {
        Podcast 
        {
            title: String::from(channel.title()),
            link: String::from(channel.link()),
            image: channel.image().unwrap().clone(),
            episodes: channel.items().to_vec(),
            last_build_date: String::from(channel.last_build_date.unwrap_or(String::from("Unknown")))
        }
    }
}

pub struct Episode; 

impl From<Item> for Episode {
    fn from(item : Item) -> Self {
        Episode {}
    }
}

pub fn parse_rss(buf : &PathBuf) -> Result<Podcast> {

    let file = match File::open(&buf) {
        Ok(file) => file,
        Err(error) => {
            return Err(anyhow!("Failed to open RSS file. {}", error))
        }
    };

    let channel = Channel::read_from(BufReader::new(file)).unwrap();


    Ok(Podcast::from(channel))
}  