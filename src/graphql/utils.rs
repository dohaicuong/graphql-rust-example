use std::fs::File;
use std::io::prelude::*;

use super::schema::AppSchema;

pub fn write_schema(schema: &AppSchema) -> Result<(), std::io::Error> {
    let mut schema_file = File::create("schema.graphql")?;
    schema_file.write_all(schema.sdl().as_bytes())?;

    Ok(())
}
