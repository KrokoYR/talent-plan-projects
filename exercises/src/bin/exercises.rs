use bson::Document;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Write};
use std::io::{BufReader, BufWriter};

#[derive(Debug, Deserialize, Serialize)]
struct Move {
    direction: Direction,
    steps: i32,
}

impl Move {
    fn new(direction: Direction, steps: i32) -> Self {
        Self { direction, steps }
    }
}

#[derive(Debug, Deserialize, Serialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn serialize_to_json() -> io::Result<()> {
    let a = Move::new(Direction::Up, 4);

    let output_file = File::create("output.json")?;
    {
        let writer = BufWriter::new(output_file);
        serde_json::to_writer(writer, &a)?;
    }
    println!("Value of a: {:?}", a);

    Ok(())
}

fn deserialize_from_json() -> io::Result<()> {
    let input_file = File::open("output.json")?;
    let reader = BufReader::new(input_file);
    let b: Move = serde_json::from_reader(reader)?;
    println!("Value of b: {:?}", b);

    Ok(())
}

fn serialize_to_ron() -> ron::Result<()> {
    let a = Move::new(Direction::Up, 4);

    let mut output_buf: Vec<u8> = vec![];
    let move_str = ron::to_string(&a)?;
    output_buf.append(&mut move_str.into_bytes());

    println!("Vector buf: {:?}", output_buf);
    let utf8_buf_str = String::from_utf8(output_buf)?;
    println!("String from utf8: {}", utf8_buf_str);

    Ok(())
}

fn serialize_bson() -> io::Result<()> {
    let output_file = File::create("output.bson")?;
    let mut writer = BufWriter::new(output_file);

    for i in 0..1000 {
        let direction = match i % 4 {
            3 => Direction::Left,
            2 => Direction::Down,
            1 => Direction::Right,
            _ => Direction::Up,
        };

        let mv = Move::new(direction, i);
        let buf = bson::to_vec(&mv).unwrap();
        writer.write_all(&buf).unwrap();
    }

    Ok(())
}

fn deserialize_bson() -> io::Result<()> {
    let mut f = File::open("output.bson").unwrap();

    while let Ok(deserialized) = Document::from_reader(&mut f) {
        println!("{:?}", deserialized);
    }

    Ok(())
}

fn main() -> io::Result<()> {
    // serialize_to_json()?;
    // deserialize_from_json()?;

    // serialize_to_ron().unwrap();

    serialize_bson()?;
    deserialize_bson()?;

    Ok(())
}
