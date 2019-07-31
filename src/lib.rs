//! `xkcd_get` gives access to xkcd's JSON API through the [`Comic`] struct.
//! Basic examples are provided on this page, see the [`Comic`] struct's page for specifics.
//!
//! # Examples
//!
//! ## Getting the Latest Comic
//!
//! ```
//! use xkcd_get::Comic;
//!
//! fn main () {
//!     let data = Comic::latest().unwrap();
//!     println!("Comic Number {}: '{}'", data.num, data.title);
//! }
//! ```
//!
//! ## Getting a Comic by Number
//!
//! ```
//! use xkcd_get::Comic;
//!
//! fn main () {
//!     let data = Comic::get(327).unwrap();
//!     println!("Comic Number {}: '{}'", data.num, data.title);
//!     // Comic Number 327: 'Exploits of a Mom'
//! }
//! ```

use serde::{Deserialize};
use chrono::{Utc, Date, TimeZone};
use std::error::Error;
use reqwest;

/// Intermediate format for comic information, as retrieved from serde
/// Lots of values are strings, or split up, which are cleaned up in the `Comic` struct instead
#[derive(Debug, Deserialize)]
struct ComicRequest {
    month: String,
    num: u32,
    link: String,
    year: String,
    news: String,
    transcript: String,
    alt: String,
    img: String,
    title: String,
    day: String
}

/// Representation of an xkcd comic
#[derive(Debug)]
pub struct Comic {
    /// The full title of the comic
    pub title: String,
    /// A URL to this comic on the xkcd site
    pub link: String,
    /// The comic number
    pub num: u32,
    /// A URL to this comic's image
    pub img: String,
    /// The alt-text / title-text for this image
    pub alt: String,
    /// The news field is somewhat vague but should contain information on occasion
    pub news: String,
    /// A transcript of this comic, if available
    pub transcript: String,
    /// The publication date of this comic, presumably in UTC
    pub date: Date<Utc>
}

impl ComicRequest {
    /// Convert a ComicRequest into a proper Comic structure
    fn comic (self) -> Comic {
        Comic {
            title: self.title,
            link: self.link,
            num: self.num,
            img: self.img,
            alt: self.alt,
            news: self.news,
            transcript: self.transcript,
            date: Utc.ymd(
                self.year.parse::<i32>().unwrap(),
                self.month.parse::<u32>().unwrap(),
                self.day.parse::<u32>().unwrap()
            )
        }
    }
}

impl Comic {

    /// Retrieve a comic by its full JSON file URL
    fn get_by_url (url: &str) -> Result<Comic, Box<dyn Error>> {
        Ok(reqwest::get(url)?.json::<ComicRequest>()?.comic())
    }

    /// Get a comic by its comic number
    /// This will fail with a parsing error if `number` is less than 1 or greater than the latest
    /// comic's number, or if the page simply fails to load.
    pub fn get (number: u32) -> Result<Comic, Box<dyn Error>> {
        Comic::get_by_url(format!("https://xkcd.com/{}/info.0.json", number).as_str())
    }

    /// Get the latest comic
    pub fn latest () -> Result<Comic, Box<dyn Error>> {
        Comic::get_by_url("https://xkcd.com/info.0.json")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get () {
        let data = Comic::get(10).unwrap();
        assert_eq!(data.title, String::from("Pi Equals"));
        assert_eq!(data.link, String::from(""));
        assert_eq!(data.num, 10);
        assert_eq!(data.img, String::from("https://imgs.xkcd.com/comics/pi.jpg"));
        assert_eq!(data.alt, String::from("My most famous drawing, and one of the first I did for the site"));
        assert_eq!(data.news, String::from(""));
        assert_eq!(data.transcript, String::from("Pi = 3.141592653589793helpimtrappedinauniversefactory7108914..."));
        assert_eq!(data.date, Utc.ymd(2006, 01, 01));
        println!("{:?}", data);
    }

    #[test]
    #[should_panic]
    fn test_get_zero () {
        Comic::get(0).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_too_high () {
        Comic::get(999999999).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_get_bad_url () {
        Comic::get_by_url("https://xkcd.com/100").unwrap();
    }

    #[test]
    fn test_latest () {
        Comic::latest().unwrap();
    }
}
