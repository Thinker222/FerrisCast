use anyhow::{anyhow, Result};
use chrono::prelude::DateTime;
use chrono::{NaiveDate, ParseError, Utc};
use rss::{Channel, Item};
use std::convert::From;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::vec::Vec;

pub static BAD_STRING: &str = "Unknown";

pub struct Podcast {
    pub title: String,
    pub link: String,
    pub image: rss::Image,      // TODO: If necessary add an image if needed
    pub episodes: Vec<Episode>, // TODO: Implement episode type
    pub last_build_date: String,
}

impl From<Channel> for Podcast {
    fn from(channel: Channel) -> Self {
        Podcast {
            title: String::from(channel.title()),
            link: String::from(channel.link()),
            image: channel.image().unwrap().clone(),
            episodes: channel
                .items()
                .iter()
                .map(|item| Episode::from(item))
                .collect(),
            last_build_date: String::from(
                channel.last_build_date.unwrap_or(String::from("Unknown")),
            ),
        }
    }
}

pub struct Episode {
    pub title: String,
    pub description: String,
    pub enclosure: String,
    pub pub_date: Result<NaiveDate, ParseError>,
}

impl From<&Item> for Episode {
    fn from(item: &Item) -> Self {
        let make_str = |opt_string: Option<&str>| String::from(opt_string.unwrap_or(BAD_STRING));
        Episode {
            title: (make_str(item.title())),
            description: (make_str(item.description())),
            enclosure: {
                let enc: Option<&rss::Enclosure> = item.enclosure();
                match enc {
                    Some(e) => String::from(e.url()),
                    _ => String::from(BAD_STRING),
                }
            },
            pub_date: (NaiveDate::parse_from_str(
                item.pub_date().unwrap_or(""),
                "%a, %d %b %Y %H:%M:%S %z",
            )),
        }
    }
}

pub fn parse_rss(buf: &PathBuf) -> Result<Podcast> {
    let file = match File::open(&buf) {
        Ok(file) => file,
        Err(error) => return Err(anyhow!("Failed to open RSS file. {}", error)),
    };

    let channel = Channel::read_from(BufReader::new(file)).unwrap();

    Ok(Podcast::from(channel))
}
