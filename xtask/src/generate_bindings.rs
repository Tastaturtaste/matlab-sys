//! This is not intended for general use!



use std::{io::Write, path::PathBuf, str::FromStr};

/// Function for generating the bindings for the mex API versions 700 and 800. They will be produced in the working directory with the naming scheme 'bindings_$version$.rs'.
pub fn generate_bindings(mut arguments: pico_args::Arguments) -> anyhow::Result<()> {
    let matlab_include_path: PathBuf = arguments
        .value_from_str("-i")
        .or_else(|_| arguments.value_from_str("--include"))
        .unwrap();
    let output_path: PathBuf = arguments
        .value_from_str("-o")
        .or_else(|_| arguments.value_from_str("--out"))
        .or(PathBuf::from_str("./"))
        .unwrap();
    println!(
        "Using matlab include path {}\n\n",
        matlab_include_path.display()
    );
    let temp_file_name: _ = std::path::Path::new("_temp_.h");
    let mut temp_file = std::fs::File::options()
        .create_new(true)
        .write(true)
        .open(temp_file_name)?;

    temp_file.write_all(
        "#include \"mex.h\"\n#include \"matrix.h\"\n#include \"mat.h\"\n#include \"engine.h\"\n"
            .as_bytes(),
    )?;
    // binding settings common for both api versions
    let bindings_common = bindgen::Builder::default()
        .clang_arg(format!("-I{}", matlab_include_path.to_str().unwrap()))
        .header(temp_file_name.display().to_string())
        .raw_line("#![allow(nonstandard_style)]")   // allow direct translation of C-style naming conventions
        .raw_line(
            r##"#[cfg(not(target_pointer_width = "64"))]
            compile_error!("The bindings are only valid for 64-bit applications. All Matlab versions after 2015b are only available in 64-bit.");"##
        )                                           // The bindings are generated with the assumption of a 64-bit system. This assumption should be reflected and guarded against in the bindings.
        .allowlist_function("mx.*")
        .allowlist_type("mx.*")
        .allowlist_var("mx.*")
        .allowlist_type("mw.*")
        .allowlist_var("MW.*")
        .blocklist_item("MW_FIRST_API_VERSION")     // These would be hardcoded to the values set by the Matlab release used to generate the bindings.
        .blocklist_item("MW_LATEST_API_VERSION")    // Since the bindings get distributed, these variables would not reflect the reality of the user.
        .allowlist_function("mex.*")
        .allowlist_type("mex.*")
        .allowlist_var("mex.*")
        .allowlist_function("mat.*")
        .allowlist_type("mat.*")
        .allowlist_var("mat.*")
        .allowlist_function("eng.*")
        .allowlist_type("Engine")
        .parse_callbacks(Box::new(RemoveVersionParserCallback::new().unwrap())) // Remove version suffixes from function names
        .translate_enum_integer_types(true)
        // .new_type_alias("Engine")        // new_type_alias currently leads to warnings about ffi-unsafe types
        // .new_type_alias("FILE")          // new_type_alias currently leads to warnings about ffi-unsafe types
        // .new_type_alias("MATFile")       // new_type_alias currently leads to warnings about ffi-unsafe types
        .constified_enum_module("mxClassID")
        .constified_enum_module("mxComplexity")
        .blocklist_type("u{0,1}int[0-9]{1,2}_T") // Fixed size integer typedefs are replaced with rusts native fixed size integers.
        .blocklist_type("CHAR16_T") // Fixed size char typedefs are replaced with rusts native fixed size unsigned integers.
        .blocklist_type("wchar_t") // Only used for the definition of CHAR16_T, which is blocklisted
        .blocklist_type("char16_t") // Only used for the definition of CHAR16_T, which is blocklisted
        // Block undocumented typedefs that do not appear in the documented api
        .blocklist_type("mexGlobalTable")
        .blocklist_type("mexGlobalTableEntry")
        .blocklist_type("mexFunctionTable")
        .blocklist_type("mexFunctionTableEntry")
        .blocklist_type("mexLocalFunctionTable")
        .blocklist_type("mexInitTermTableEntry")
        .blocklist_type("mexGlobalTableEntry_Tag")
        .blocklist_type("mexFunctionTableEntry_tag")
        .blocklist_type("_mexInitTermTableEntry")
        .blocklist_type("_mexLocalFunctionTable")
        .size_t_is_usize(true) // Matlab already assumes that size_t is a pointer-sized unsigned integer as can be seen in `tmwtypes.h` on the definition of mwSize for example.
        .sort_semantically(true)
        .merge_extern_blocks(true)
        .layout_tests(false) // Disabled since they do not work correctly if the bindings are used on a different architecture than the one they are generated on, e.g. 32bit/64bit difference.
        .use_core()
        .rustfmt_bindings(true);

    // The passed defines and options for both api versions were extracted from a dry run of the mex command in matlab with windows 10. Defines which were verified to not be referenced in the include headers using ripgrep are removed.
    let separate_complex = bindings_common
        .clone()
        .clang_args([
            "-DNDEBUG",
            "-DMATLAB_DEFAULT_RELEASE=R2017b",
            "-DUSE_MEX_CMD",
            "-DMATLAB_MEX_FILE",
        ])
        .generate()?
        .to_string();

    let interleaved_complex = bindings_common
        .clang_args([
            "-DNDEBUG",
            "-DMATLAB_MEXCMD_RELEASE=R2018a",
            "-DUSE_MEX_CMD",
            "-DMATLAB_MEX_FILE",
        ])
        .generate()?
        .to_string();

    let type_replacements = std::collections::BTreeMap::from_iter([
        ("mxDouble", "f64"),
        ("mxSingle", "f32"),
        ("mxInt8", "i8"),
        ("mxUint8", "u8"),
        ("mxInt16", "i16"),
        ("mxUint16", "u16"),
        ("mxInt32", "i32"),
        ("mxUint32", "u32"),
        ("mxInt64", "i64"),
        ("mxUint64", "u64"),
        ("ptrdiff_t", "isize"), // Matlab already assumes that ptrdiff_t is a pointer-sized signed integer as can be seen in `tmwtypes.h` on the definition of mwSignedIndex.
        ("CHAR16_T", "u16"), // Matlab defines `CHAR16_T` as either a char16_t when available otherwise as a wchar_t when compiling with MSVC or otherwise a uint16_t. In all cases where the target platform has an unsigned 16-bit integer, the resulting type is a 16-bit unsigned integer when compiled as C.
    ]);
    let separate_complex = replace_typedefs(separate_complex, &type_replacements)?;
    let interleaved_complex = replace_typedefs(interleaved_complex, &type_replacements)?;
    std::fs::File::options()
        .create_new(true)
        .write(true)
        .open(output_path.join("separate_complex_impl.rs"))?
        .write_all(separate_complex.as_bytes())?;
    std::fs::File::options()
        .create_new(true)
        .write(true)
        .open(output_path.join("interleaved_complex_impl.rs"))?
        .write_all(interleaved_complex.as_bytes())?;
    std::fs::remove_file(temp_file_name)?;

    Ok(())
}

fn replace_typedefs(
    mut s: String,
    type_replacements: &std::collections::BTreeMap<&str, &str>,
) -> anyhow::Result<String> {
    // Remove the type aliases for the key-types and replace all usages with the value-types
    let typedef_pattern = regex::Regex::new(r"type\s+([a-zA-Z0-9_]+?)\s*=")?;
    // Remove all type alias declarations for the key-types
    s = s
        .lines()
        .map(|line| match typedef_pattern.captures(line) {
            Some(cap) => {
                if type_replacements.contains_key(&cap[1]) {
                    ""
                } else {
                    line
                }
            }
            None => line,
        })
        .collect();
    for (old, new) in type_replacements.iter() {
        // This way of finding and replacing the key type with the value type is brittle, because _every_ substring where the name of the key type prepended with a space appears gets replaced.
        // This could fail for example if the key type gets referenced (&T) or the type is used in a function signature and there is no space after the colon. That said, for the formating of the code bindgen produces this works.
        s = s.replace(format!(" {old}").as_str(), format!(" {new}").as_str());
    }

    Ok(s)
}


#[derive(Debug, Clone)]
struct RemoveVersionParserCallback {
    fn_version_pattern: regex::Regex,
}
impl RemoveVersionParserCallback {
    fn new() -> anyhow::Result<Self> {
        Ok(Self {
            // Matches only if a version suffix is present
            fn_version_pattern: regex::Regex::new(r"([a-zA-Z0-9_]+)(_700|_730|_800)")?,
        })
    }
}
impl bindgen::callbacks::ParseCallbacks for RemoveVersionParserCallback {
    fn generated_name_override(&self, function_name: &str) -> Option<String> {
        let captures = self.fn_version_pattern.captures(function_name)?;
        Some(captures[1].to_string())
    }
}
