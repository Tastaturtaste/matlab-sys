use std::io::{BufRead, Write};

use anyhow::{self, Context, Ok};
use pico_args;
use regex;
// The command line arguments should be key-value pairs, with the api version as the key and the corresponding filepath as the value, e.g. '-API700=./bindings_700.rs or -API700 ./bindings_700.rs'
pub fn reexport_versionless(mut arguments: pico_args::Arguments) -> anyhow::Result<()> {
    let mut version_files: Vec<(_, String)> = Vec::new();
    if let Some(path) = arguments
        .opt_value_from_str("--API700")
        .context("Failed while parsing --API700")?
    {
        version_files.push((700, path))
    }
    if let Some(path) = arguments
        .opt_value_from_str("--API800")
        .context("Failed while parsing --API800")?
    {
        version_files.push((800, path))
    }
    let remaining_arguments = arguments.finish();
    if !remaining_arguments.is_empty() {
        anyhow::bail!("Unknown arguments{:?}", remaining_arguments);
    }
    if version_files.is_empty() {
        return Ok(());
    }
    let reexport_file = std::fs::File::options()
        .create_new(true)
        .write(true)
        .open("./reexports.rs")?;
    let mut reexport_writer = std::io::BufWriter::new(reexport_file);

    let fn_pattern = regex::Regex::new(r"pub fn ([a-zA-Z0-9_]+?)(_700|_730|_800)?\s*\(")?;
    let type_pattern = regex::Regex::new(r"pub type ([a-zA-Z0-9_]+?) =")?;
    let const_pattern = regex::Regex::new(r"pub const ([a-zA-Z0-9_]+?):\s*[a-zA-Z0-9]+?\s*=")?;
    let struct_pattern = regex::Regex::new(r"pub struct ([a-zA-Z0-9_]+?)\s*\{")?;
    for (version, path) in version_files {
        let path = std::path::PathBuf::from(path);
        let import_name = path
            .file_stem()
            .ok_or(anyhow::anyhow!(
                "Could not extract import name from path for api version {version}"
            ))?
            .to_str()
            .ok_or(anyhow::anyhow!("File name is not valid UTF8"))?;
        let version_file_reader = std::io::BufReader::new(std::fs::File::open(&path)?);
        reexport_writer.write_all(format!("mod {import_name};\n").as_bytes())?;
        reexport_writer.write_all(format!("pub use {import_name}:: {{\n").as_bytes())?;
        for line in version_file_reader.lines() {
            let line = line?;
            if let Some(captures) = fn_pattern.captures(&line) {
                let function_stem = &captures[1];
                if let Some(version_suffix) = captures.get(2).map(|m| m.as_str()) {
                    reexport_writer.write_all(
                        format!("{0}{1} as {0},\n", function_stem, version_suffix).as_bytes(),
                    )?;
                } else {
                    reexport_writer.write_all(format!("{function_stem},\n").as_bytes())?;
                }
            } else if let Some(captures) = type_pattern.captures(&line) {
                let type_name = &captures[1];
                reexport_writer.write_all(format!("{type_name},\n").as_bytes())?;
            } else if let Some(captures) = const_pattern.captures(&line) {
                let const_name = &captures[1];
                reexport_writer.write_all(format!("{},\n", const_name).as_bytes())?;
            } else if let Some(captures) = struct_pattern.captures(&line) {
                let struct_name = &captures[1];
                reexport_writer.write_all(format!("{struct_name},\n").as_bytes())?;
            }
        }
        reexport_writer.write_all("};\n\n".as_bytes())?;
    }
    Ok(())
}
