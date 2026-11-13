use std::fs;

#[path = "src/schema/resume.rs"]
mod resume_schema;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=src/schema/resume.rs");

    // Write the json schema for resume
    let schema = schemars::schema_for!(resume_schema::ResumeInfo);
    let schema_json = serde_json::to_string_pretty(&schema)?;
    fs::create_dir_all("target/jsonschema/")?;
    fs::write("target/jsonschema/resume.schema.json", schema_json)?;

    Ok(())
}
