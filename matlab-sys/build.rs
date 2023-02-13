//! Currently all environment variables are assumed to be and contain valid utf-8. If this does not apply to your use-case please consider reporting this as an issue at https://github.com/Tastaturtaste/matlab-sys/issues.
//! In the meantime this build script can be circumvented by using a cargo .config file with the links key. https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinks.

fn main() {
    // Check if the link-lib feature is disabled. If it is, the user wants full control and has full responsibility to link to the correct libraries.
    if std::env::var("CARGO_FEATURE_LINK_LIB").is_err() {
        return;
    }
    // Check which platform we run on.
    // TODO: Generalize for other platforms like linux or mac
    let platform = match std::env::var("CARGO_CFG_TARGET_OS")
        .as_deref()
        .expect("Environment variable 'CARGO_CFG_TARGET_OS' not found.")
    {
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        "macos" => OS::MacOS,
        unsupported_target => panic!("Target {unsupported_target} are currently unsupported."),
    };

    let matlabpath = get_matlab_path();

    // For better error messages check if the path to the Matlab installation actually exists and is readable.
    assert!(
        std::path::Path::new(&matlabpath)
            .try_exists()
            .expect(&format!("Cannot check existence of path {matlabpath}")),
        "The path to the matlab installation does not exist: {matlabpath}"
    );

    // Tell cargo to look for shared libraries in the specified directory. This is platform specific and currently only windows is supported.
    assert_eq!(platform, OS::Windows, "Currently only windows is supported. Please raise an issue at https://github.com/Tastaturtaste/matlab-sys/issues with a description of your environment.");
    let mut link_search_path = format!("{matlabpath}/extern/lib/win64/");
    // This value is defined as the empty string if it is not needed for disambiguation for historical reasons
    // The possible values are
    let target_env =
        std::env::var("CARGO_CFG_TARGET_ENV").expect("'CARGO_CFG_TARGET_ENV' not found");
    match target_env.as_str() {
        "msvc" => link_search_path.push_str("microsoft/"),
        "gnu" => link_search_path.push_str("mingw64/"),
        "musl" | "sgx" | "" => unimplemented!("Linking against the libraries for the provided target abi {target_env} is currently not implemented"),
        _ => unreachable!("The possible values for target_env can only be the ones checked against above according to https://doc.rust-lang.org/reference/conditional-compilation.html#target_env"),
    }
    assert!(
        std::path::Path::new(&link_search_path)
            .try_exists()
            .expect(&format!(
                "Cannot check existence of path {link_search_path}"
            )),
        "The path to the matlab link libraries does not exist: {link_search_path}"
    );
    println!("cargo:rustc-link-search={link_search_path}");

    // Check if all required libraries are present and instruct cargo to link them.
    check_link_lib_existence_and_queue_link(&link_search_path, "libmex", platform);
    check_link_lib_existence_and_queue_link(&link_search_path, "libmx", platform);
    check_link_lib_existence_and_queue_link(&link_search_path, "libmat", platform);
}

fn check_link_lib_existence_and_queue_link(search_path: &str, libname: &str, platform: OS) {
    assert!(
        std::path::Path::new(&format!(
            "{search_path}{}",
            add_link_lib_extension(libname, platform)
        ))
        .try_exists()
        .expect(&format!(
            "Cannot check existence of path {search_path}{}",
            add_link_lib_extension(libname, platform)
        )),
        "The path to the matlab link libraries does not exist: {search_path}libmex"
    );
    print!("cargo:rustc-link-lib=");
    println!("{libname}");
}

fn add_link_lib_extension(name: impl Into<String>, platform: OS) -> String {
    let mut name = name.into();
    match platform {
        OS::Windows => name.push_str(".lib"),
        OS::Linux => name.push_str(".so"),
        OS::MacOS => name.push_str(".dylib"),
    }
    name
}

// Get the path to the matlab installation to link against. Prioritize an explicitly set path, otherwise try to run Matlab and ask it for its directory.
fn get_matlab_path() -> String {
    if let Ok(path) = std::env::var("MATLABPATH") {
        path
    } else if let Ok(cmd_output) = std::process::Command::new("matlab")
        .arg("-batch")
        .arg("disp(matlabroot)")
        .output()
    {
        String::from_utf8(cmd_output.stdout)
            .expect("The path to the Matlab installation is not valid utf-8")
            .trim() // Strip the newline matlab appends when using the disp() function
            .to_owned()
    } else {
        panic!("Matlab installation to link against not found. Specify the path to the installation to link against in the environment variable 'MATLABPATH' or make sure Matlab is callable from the command line.")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OS {
    Windows,
    Linux,
    MacOS,
}
