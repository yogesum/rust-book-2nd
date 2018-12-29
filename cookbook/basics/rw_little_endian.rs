#!/usr/bin/env run-cargo-script
// cargo-deps: byteorder="^1.2.3"

extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io;

#[derive(Debug, Default, PartialEq)]
struct Payload {
    kind: u8,
    value: u16,
}

fn encode(payload: &Payload) -> Result<Vec<u8>, io::Error> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, io::Error> {
    Ok(Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    })
}

fn run() -> Result<(), io::Error> {
    let original_payload = Payload::default();
    let encoded_bytes = encode(&original_payload)?;
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decoded_payload);
    println!("successful encoding/decoding of integers in little-endian byte order");
    Ok(())
}

fn main() -> Result<(), io::Error> {
    run()
}