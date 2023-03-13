use std::str::FromStr;

use anyhow::Ok;

// Run examples as tests
const TEST_EXAMPLES: &[&str] = &["array-product", "mex-call-matlab", "array-size", "explore"];

pub fn test(_arguments: pico_args::Arguments) -> anyhow::Result<()> {
    // Make sure the most recent version gets run
    std::process::Command::new("cargo")
        .arg("clean")
        .arg("-p")
        .arg("matlab-sys")
        .spawn()
        .expect("Could not spawn the cargo clean process")
        .wait()?;
    // Compile all test examples
    let mut cargo_process = std::process::Command::new("cargo");
    cargo_process.arg("build");
    for example in TEST_EXAMPLES {
        cargo_process.arg("-p").arg(example);
    }
    cargo_process
        .spawn()
        .expect("Could not spawn the cargo build process")
        .wait()?;
    // Run doctests
    std::process::Command::new("cargo")
        .arg("test")
        .arg("--doc")
        .spawn()
        .expect("Could not spawn the cargo test --doc process")
        .wait()?;

    // Change directory to the target directory, where this program is assumed to live
    std::env::set_current_dir(
        std::env::current_exe()
            .expect("Couldn't get directory of currently executing executable")
            .parent()
            .unwrap(),
    )
    .expect("Couldn't change directory to target/{debug|release} directory");

    dlls_to_mex(
        std::env::current_exe()
            .expect("Couldn't get directory of currently executing executable")
            .parent()
            .unwrap(),
    )?;

    let matlab_files_dir = std::path::PathBuf::from_str(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../tests/matlab_files/"
    ))
    .unwrap()
    .to_path_buf();
    println!("Running tests in Matlab...");
    // assertSuccess is only available from matlab release 2020a and onward.
    let result = std::process::Command::new("matlab")
        .arg("-batch")
        .arg(format!(
            "addpath('{}'); assertSuccess(runtests('test'))",
            matlab_files_dir.canonicalize().unwrap().to_str().unwrap()
        ))
        .output()
        .expect("Couldn't spawn matlab process for testing");
    let exitcode = result.status;
    if exitcode.success() {
        println!("\nTests successful\n");
        Ok(())
    } else {
        anyhow::bail!(
            "Matlab tests failed! Matlab exited with {:?}.\nStdout: \n{}\n\nStderr: \n{}",
            exitcode
                .code()
                .expect("Matlab exited with no exit code, probably because of a raised signal"),
            std::str::from_utf8(&result.stdout).expect("stdout was not valid utf-8"),
            std::str::from_utf8(&result.stderr).expect("stderr was not valid utf-8")
        )
    }
}
// Renames all dlls created from the example projects in TEST_EXAMPLES to the expected mexfunction name.
// E.g  libarray_product.so -> array_product.mexa64
// or   array_product.dll   -> array_product.mexw64
fn dlls_to_mex(path_to_dlls: &std::path::Path) -> anyhow::Result<()> {
    let (prefix_to_remove, file_ext_change) = if cfg!(target_os = "windows") {
        ("", (".dll", ".mexw64"))
    } else if cfg!(target_os = "linux") {
        ("lib", (".so", ".mexa64"))
    } else if cfg!(target_os = "macos") {
        // (dynamic link library prefix , (".dylib", ".mexmaci64"))
        ("lib", (".dylib", ".mexmaci64"))
    } else {
        panic!("Unknown target os")
    };

    for name in TEST_EXAMPLES {
        let replaced_dash = name.replace('-', "_");
        let compiled_dll_name = format!("{prefix_to_remove}{replaced_dash}{}", file_ext_change.0);
        let new_mex_name = format!("{replaced_dash}{}", file_ext_change.1);
        std::fs::rename(
            path_to_dlls.join(compiled_dll_name),
            path_to_dlls.join(new_mex_name),
        )?;
    }
    Ok(())
}
