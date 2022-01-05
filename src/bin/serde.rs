use serde_transcode::transcode;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut deserializer = serde_json::Deserializer::from_reader(stdin());
    let mut serializer = serde_json::Serializer::pretty(stdout());

    transcode(&mut deserializer, &mut serializer)?;
    serializer.into_inner().flush()?;

    Ok(())
}
