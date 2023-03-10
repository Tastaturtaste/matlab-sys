use std::{
    io::{BufRead, Write},
    path::PathBuf,
    str::FromStr,
};

use anyhow::{self, Context, Ok};


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
    let out_file = arguments
        .value_from_str("-o")
        .or_else(|_| arguments.value_from_str("--out"))
        .or(PathBuf::from_str("./reexports.rs"))
        .unwrap();
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
        .open(out_file)?;
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
                let function_name = &captures[1];
                reexport_writer.write_all(format!("{function_name},\n").as_bytes())?;
            } else if let Some(captures) = type_pattern.captures(&line) {
                let type_name = &captures[1];
                reexport_writer.write_all(format!("{type_name},\n").as_bytes())?;
            } else if let Some(captures) = const_pattern.captures(&line) {
                let const_name = &captures[1];
                reexport_writer.write_all(format!("{},\n", const_name).as_bytes())?;
            } else if let Some(captures) = struct_pattern.captures(&line) {
                let struct_name = &captures[1];
                // Skip the types with leading underscore as they are not part of the intended public API and only used for type aliases.
                if struct_name.starts_with('_') {
                    continue;
                }
                // Skip the types with a _tag or _Tag suffix as they are not part of the intended public API and only used for type aliases.
                if struct_name.ends_with("_tag") || struct_name.ends_with("_Tag") {
                    continue;
                }
                // Skip the struct `engine` specifically as it gets aliased by the type alias `Engine`, which is the public and documented type.
                if struct_name == "engine" {
                    continue;
                }
                reexport_writer.write_all(format!("{struct_name},\n").as_bytes())?;
            }
        }
        reexport_writer.write_all("};\n\n".as_bytes())?;
    }
    Ok(())
}
