#!/usr/bin/env run-cargo-script
// cargo-deps: url, data-encoding, base64, csv, serde, serde_derive, serde_json, toml, byteorder

extern crate url;
extern crate data_encoding;
extern crate base64;
extern crate csv;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate serde;
extern crate toml;
extern crate byteorder;

fn percent_encode() -> Result<(), std::str::Utf8Error> {
    use url::percent_encoding::{utf8_percent_encode, percent_decode, DEFAULT_ENCODE_SET};

    let input = "confident, productive systems programming";
    let iter = utf8_percent_encode(input, DEFAULT_ENCODE_SET);
    let encoded: String = iter.collect();
    assert_eq!(encoded, "confident,%20productive%20systems%20programming");

    let iter = percent_decode(encoded.as_bytes());
    let decoded = iter.decode_utf8()?;
    assert_eq!(decoded, "confident, productive systems programming");

    Ok(())
}

fn form_urlencode() {
    use url::form_urlencoded::{byte_serialize, parse};

    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}

fn hex_encode() -> Result<(), data_encoding::DecodeError> {
    use data_encoding::HEXUPPER;

    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}

fn base64_encode() {
    use std::str;
    use base64::{encode, decode};

    let hello = b"hello rustaceans";
    let encoded = encode(hello);
    let decoded = decode(&encoded).unwrap();

    println!("origin: {}", str::from_utf8(hello).unwrap());
    println!("base64 encoded: {}", encoded);
    println!("back to origin: {}", str::from_utf8(&decoded).unwrap());
}

fn csv_read() -> Result<(), csv::Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());
    for record in reader.records() {
        let record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            &record[0],
            &record[1],
            &record[2],
            &record[3],
        );
    }

    Ok(())
}

#[derive(Deserialize)]
struct Record {
    year: u16,
    make: String,
    model: String,
    description: String,
}

fn serde_csv_read() -> Result<(), csv::Error> {
    let csv = "year,make,model,description
1948,Porsche,356,Luxury sports car
1967,Ford,Mustang fastback 1967,American car";

    let mut reader = csv::Reader::from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let record: Record = record?;
        println!(
            "In {}, {} built the {} model. It is a {}.",
            record.year,
            record.make,
            record.model,
            record.description,
        );
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct PersonRecord {
    name: String,
    place: String,
    #[serde(deserialize_with = "csv::invalid_option")]
    id: Option<u64>,
}

fn serde_csv_read_delimiter() -> Result<(), csv::Error> {
    let data = "name\tplace\tid
Mark\tMelbourne\t46
Ashley\tZurich\t92";

    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());

    for result in reader.deserialize::<PersonRecord>() {
        println!("{:?}", result?);
    }

    Ok(())
}

fn csv_filter() -> Result<(), csv::Error> {
    let query = "CA";
    let data = "\
City,State,Population,Latitude,Longitude
Kenai,AK,7610,60.5544444,-151.2583333
Oakman,AL,,33.7133333,-87.3886111
Sandfort,AL,,32.3380556,-85.2233333
West Hollywood,CA,37031,34.0900000,-118.3608333";
    
    let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == query) {
            wtr.write_record(&record).unwrap();
        }
    }

    wtr.flush().unwrap();
    Ok(())
}

fn invalid_csv() -> Result<(), csv::Error> {
    let data = "name,place,id
mark,sydney,46.5
ashley,zurich,92
akshat,delhi,37
alisha,colombo,xyz";

    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    for result in rdr.deserialize() {
        let record: PersonRecord = result?;
        println!("{:?}", record);
    }

    Ok(())
}

fn csv_serialize() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    wtr.write_record(&["Name", "Place", "ID"])?;

    wtr.serialize(("Mark", "Sydney", 87))?;
    wtr.serialize(("Ashley", "Dublin", 32))?;
    wtr.serialize(("Akshat", "Delhi", 11))?;

    wtr.flush()?;
    Ok(())
}

#[derive(Serialize)]
struct PersonDetail<'a> {
    name: &'a str,
    place: &'a str,
    id: u64,
}

fn serde_csv_serialize() -> Result<(), csv::Error> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());

    let rec1 = PersonDetail { name: "Mark", place: "Melbourne", id: 56};
    let rec2 = PersonDetail { name: "Ashley", place: "Sydney", id: 64};
    let rec3 = PersonDetail { name: "Akshat", place: "Delhi", id: 98};

    wtr.serialize(rec1)?;
    wtr.serialize(rec2)?;
    wtr.serialize(rec3)?;

    wtr.flush().unwrap();

    Ok(())
}

#[derive(Debug)]
struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize)]
struct Row {
    color_name: String,
    color: HexColor,
}

use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

impl FromStr for HexColor {
    type Err = std::num::ParseIntError;

    fn from_str(hex_color: &str) -> Result<Self, Self::Err> {
        let trimmed = hex_color.trim_matches('#');
        if trimmed.len() != 6 {
            panic!("Invalid length of hex string")
        } else {
            Ok(HexColor {
                red: u8::from_str_radix(&trimmed[..2], 16)?,
                green: u8::from_str_radix(&trimmed[2..4], 16)?,
                blue: u8::from_str_radix(&trimmed[4..6], 16)?,
            })
        }
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

fn csv_col_transform() -> Result<(), csv::Error> {
    let data = "color_name,color
red,#ff0000
green,#00ff00
blue,#0000FF
periwinkle,#ccccff
magenta,#ff00ff".to_owned();

    let mut out = csv::Writer::from_writer(vec![]);
    let mut reader = csv::Reader::from_reader(data.as_bytes());

    for result in reader.deserialize::<Row>() {
        let res = result?;
        out.serialize((
            res.color_name,
            res.color.red,
            res.color.green,
            res.color.blue,
        ))?;
    }

    let written = String::from_utf8(out.into_inner().unwrap()).unwrap();
    assert_eq!(Some("magenta,255,0,255"), written.lines().last());
    println!("{}", written);
    Ok(())
}

fn json_parse() -> Result<(), serde_json::Error> {
    let j = r#"{
        "userid": 103609,
        "verified": true,
        "access_privileges": [
          "user",
          "admin"
        ]
    }"#;

    let parsed: serde_json::Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}

fn toml_parse() -> Result<(), toml::de::Error> {
    let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
    "#;

    let package_info: toml::Value = toml::from_str(toml_content)?;

    assert_eq!(package_info["dependencies"]["serde"].as_str(), Some("1.0"));
    assert_eq!(
        package_info["package"]["name"].as_str(),
        Some("your_package"),
    );

    Ok(())
}

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: std::collections::HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn serde_toml_parse() -> Result<(), toml::de::Error> {
    let toml_content = r#"
    [package]
    name = "your_package"
    version = "0.1.0"
    authors = ["You! <you@example.org>"]

    [dependencies]
    serde = "1.0"
    "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

#[derive(Debug, Default, PartialEq)]
struct Payload {
    kind: u8,
    value: u16,
}

fn encode(payload: &Payload) -> Result<Vec<u8>, std::io::Error> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, std::io::Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };

    Ok(payload)
}

fn rw_little_endian() -> Result<(), std::io::Error> {
    let original_payload = Payload::default();
    let encoded_bytes = encode(&original_payload)?;
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decoded_payload);
    Ok(())
}

fn main() {
    percent_encode().unwrap();
    form_urlencode();
    hex_encode().unwrap();
    base64_encode();

    csv_read().unwrap();
    serde_csv_read().unwrap();
    serde_csv_read_delimiter().unwrap();
    csv_filter().unwrap();
    invalid_csv().unwrap();
    csv_serialize().unwrap();
    serde_csv_serialize().unwrap();
    csv_col_transform().unwrap();

    json_parse().unwrap();
    toml_parse().unwrap();
    serde_toml_parse().unwrap();
    rw_little_endian().unwrap();
}