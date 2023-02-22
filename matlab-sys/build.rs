//! Currently all environment variables are assumed to be and contain valid utf-8. If this does not apply to your use-case please consider reporting this as an issue at https://github.com/Tastaturtaste/matlab-sys/issues.
//! In the meantime this build script can be circumvented by using a cargo .config file with the links key. https://doc.rust-lang.org/cargo/reference/config.html#targettriplelinks.

const LINUX_LINKNAMES: &[&str] = &["mex", "mx", "mat", "eng"];
const WIN_LINKNAMES: &[&str] = &["libmex", "libmx", "libmat", "libeng"];

fn main() {
    // Check which platform we run on.
    let platform = match std::env::var("CARGO_CFG_TARGET_OS")
        .as_deref()
        .expect("Environment variable 'CARGO_CFG_TARGET_OS' not found.")
    {
        "windows" => OS::Windows,
        "linux" => OS::Linux,
        "macos" => OS::MacOS,
        unsupported_target => panic!("Target {unsupported_target} are currently unsupported."),
    };
    // This value is defined as the empty string if it is not needed for disambiguation for historical reasons.
    // https://doc.rust-lang.org/reference/conditional-compilation.html#target_env
    let target_env =
        std::env::var("CARGO_CFG_TARGET_ENV").expect("'CARGO_CFG_TARGET_ENV' not found");

    let matlabpath = get_matlab_path();
    // For better error messages check if the path to the Matlab installation actually exists and is readable.
    assert!(
        std::path::Path::new(&matlabpath)
            .try_exists()
            .unwrap_or_else(|_| panic!("Cannot check existence of path {matlabpath}")),
        "The path to the matlab installation does not exist: {matlabpath}"
    );

    // Tell cargo to look for shared libraries in the specified directory.
    let link_search_path = format!("{matlabpath}/{}", match (platform, target_env.as_str()) {
        (OS::Windows, "msvc") => "extern/lib/win64/microsoft/",
        (OS::Windows, "gnu") => "extern/lib/win64/mingw64/",
        (OS::Linux, _) => "bin/glnxa64/",
        _ => unimplemented!("Combination of {platform:?} and {target_env:?} not supported. Please raise an issue at https://github.com/Tastaturtaste/matlab-sys/issues with a description of your environment."),
    });

    assert!(
        std::path::Path::new(&link_search_path)
            .try_exists()
            .unwrap_or_else(|_| panic!("Cannot check existence of path {link_search_path}")),
        "The path to the matlab link libraries does not exist: {link_search_path}"
    );
    println!("cargo:rustc-link-search={link_search_path}");

    // Tell cargo which libraries to link. On linux the standard linker 'ld' prepends the prefix 'lib' automatically for all libraries
    // while on windows the linker 'link.exe' uses the full filename, this step is handled separately depending on target platform.
    match platform {
        OS::Windows => {
            for lib in WIN_LINKNAMES {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        OS::Linux => {
            for lib in LINUX_LINKNAMES {
                println!("cargo:rustc-link-lib={lib}");
            }
        }
        OS::MacOS => unimplemented!(),
    }
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
