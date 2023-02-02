use std::env;

fn main() {
    if env::var("CARGO_FEATURE_LINK_LIB").is_err() {
        return;
    }
    let matlabpath = env::var("MATLABPATH").expect(
        "Environment variable 'MATLABPATH' with path to matlab install directory not found",
    );

    // Tell cargo to look for shared libraries in the specified directory. This is platform specific and currently only windows is supported.
    // TODO: Generalize for other platforms like linux or mac
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        eprintln!("Non-windows targets are currently unsupported.")
    }
    let mut link_search_path = format!("{matlabpath}/extern/lib/win64/");
    // This value is defined as the empty string if it is not needed for disambiguation for historical reasons
    // The possible values are
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("'CARGO_CFG_TARGET_ENV' not found");
    match target_env.as_str() {
        "msvc" => link_search_path.push_str("microsoft/"),
        "gnu" => link_search_path.push_str("mingw64/"),
        "musl" | "sgx" | "" => unimplemented!("Linking against the libraries for the provided target abi {target_env} is currently not implemented"),
        _ => unreachable!("The possible values for target_env can only be the ones checked against above according to https://doc.rust-lang.org/reference/conditional-compilation.html#target_env"),
    }
    println!("cargo:rustc-link-search={link_search_path}");

    // Tell cargo to tell rustc to link the matlab libmex library
    println!("cargo:rustc-link-lib=libmex");
    println!("cargo:rustc-link-lib=libmx");
    println!("cargo:rustc-link-lib=libmat");
}
