#!/usr/bin/env run-cargo-script
// cargo-deps: reqwest, select, url, regex, lazy_static, mime, serde, serde_derive, tempdir

extern crate reqwest;
extern crate select;
extern crate url;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate mime;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate tempdir;

use select::{document::Document, predicate::Name};

fn extract_link() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;

    Document::from_read(res).unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    Ok(())
}

use std::collections::HashSet;
use url::{Url, Position};
use reqwest::StatusCode;

fn get_base_url(url: &Url, doc: &Document) -> Result<Url, std::io::Error> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    ).unwrap();

    Ok(base_url)
}

fn check_link(url: &Url) -> Result<bool, reqwest::Error> {
    let res = reqwest::get(url.as_ref())?;

    Ok(res.status() != StatusCode::NOT_FOUND)
}

fn broken_link() -> Result<(), reqwest::Error> {
    let url = Url::parse("https://www.rust-lang.org/en-US/").unwrap();

    let res = reqwest::get(url.as_ref())?;
    let document = Document::from_read(res).unwrap();

    let base_url = get_base_url(&url, &document).unwrap();
    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    links
        .iter()
        .filter(|link| check_link(link).ok() == Some(false))
        .for_each(|x| println!("{} is broken", x));

    Ok(())
}

use std::io::Read;
use std::borrow::Cow;
use regex::Regex;

fn extract_links(content: &str) -> Result<HashSet<Cow<str>>, regex::Error> {
    lazy_static!{
        static ref WIKI_REGEX: Regex = Regex::new(r"(?x)
            \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
            |
            (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
        ").unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    Ok(links)
}

fn uniq_links() -> Result<(), reqwest::Error> {
    let mut content = String::new();
    reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )?.read_to_string(&mut content).unwrap();

    println!("{:#?}", extract_links(&content).unwrap());

    Ok(())
}

fn url_parse() -> Result<(), url::ParseError> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}

fn base_url(mut url: Url) -> Url {
    url.path_segments_mut().unwrap().clear();

    url.set_query(None);
    url
}

fn url_base() -> Result<(), url::ParseError> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url);

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}

fn build_gh_url(path: &str) -> Result<Url, url::ParseError> {
    const GITHUB: &'static str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}
    
fn create_url() -> Result<(), url::ParseError> {
    let path = "/rust-lang/cargo";

    let gh = build_gh_url(path)?;

    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
    println!("The joined URL is: {}", gh);

    Ok(())
}

fn url_info() -> Result<(), url::ParseError> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(url::Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    let expected_scheme = "ftp".to_owned();
    let expected_host = url::Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = url::Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}

fn modify_url() -> Result<(), url::ParseError> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..url::Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}

use mime::{Mime, APPLICATION_OCTET_STREAM};
fn str_to_mime() {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} used default value {:?}",
        invalid_mime_type, default_mime,
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

        println!(
            "MIME for {:?} was parsed as {:?}",
            valid_mime_type, parsed_mime,
        );
}

fn find_mimetype(filename: &String) -> Mime {
    let parts: Vec<&str> = filename.split(".").collect();

    let res = match parts.last() {
        Some(v) => match *v {
            "png" => mime::IMAGE_PNG,
            "jpg" => mime::IMAGE_JPEG,
            "json" => mime::APPLICATION_JSON,
            &_ => mime::TEXT_PLAIN,
        },
        None => mime::TEXT_PLAIN,
    };

    res
}

fn filename_to_mime() {
    let filename = vec!("foobar.jpg", "foo.bar", "foobar.png");
    for file in filename {
        let mime = find_mimetype(&file.to_owned());
        println!("MIME for {}: {}", file, mime);
    }
}

use std::str::FromStr;

fn parse_http_mime() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")?;
    let headers = response.headers();

    match headers.get(reqwest::header::CONTENT_TYPE) {
        None => println!("The response does not contain a Content-Type header."),
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str().unwrap()).unwrap();
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };

            println!("The response contains {}.", media_type);
        },
    };

    Ok(())
}

fn http_get() -> Result<(), reqwest::Error> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}

fn main() {
    extract_link().unwrap();
    broken_link().unwrap();
    uniq_links().unwrap();

    url_parse().unwrap();
    url_base().unwrap();
    create_url().unwrap();
    url_info().unwrap();
    modify_url().unwrap();

    str_to_mime();
    filename_to_mime();
    parse_http_mime().unwrap();

    http_get().unwrap();
    // calling a web api
    // downloads
}