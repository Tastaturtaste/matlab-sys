use anyhow::Ok;

// Run examples as tests
const TEST_EXAMPLES: &[&str] = &["array-product", "mex-call-matlab"];

pub fn test(_arguments: pico_args::Arguments) -> anyhow::Result<()> {
    // Specify all mex examples to be used for the tests
    // Compile all mex examples
    let mut cargo_process = std::process::Command::new("cargo");
    cargo_process.arg("build");
    for example in TEST_EXAMPLES {
        cargo_process.arg("-p").arg(example);
    }
    cargo_process
        .spawn()
        .expect("Could not build test projects")
        .wait()?;

    // Change directory to the target directory, where this program is assumed to live
    std::env::set_current_dir(
        std::env::current_exe()
            .expect("Couldn't get directory of currently executing executable")
            .parent()
            .unwrap(),
    )
    .expect("Couldn't change directory to target/{debug|release} directory");

    // Windows: *.mexw64
    // Linux: *.mexa64
    let file_extension = if cfg!(target_os = "windows") {
        (".dll", ".mexw64")
    } else if cfg!(target_os = "linux") {
        (".so", ".mexa64")
    } else if cfg!(target_os = "macos") {
        (".dylib", ".mexmeci64")
    } else {
        panic!("Unknown target os")
    };
    for name in TEST_EXAMPLES {
        // Change the file extension from all dynamic link libraries created from the mex examples to the platform specific mex extension.
        let compiled_name = name.replace('-', "_");
        std::fs::rename(
            format!("./{compiled_name}{}", file_extension.0),
            format!("./{compiled_name}{}", file_extension.1),
        )
        .expect("Couldn't change file extension necessary to be usable as a mex function.");
    }
    println!("Running tests in Matlab...");
    let result = std::process::Command::new("matlab")
        .arg("-batch")
        .arg(include_str!("./test.m"))
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
