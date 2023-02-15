pub fn publish(arguments: pico_args::Arguments) -> anyhow::Result<()> {
    // Run tests
    crate::test::test(pico_args::Arguments::from_vec(vec![]))?;
    // Tests were sucessful, we can publish the crate. Prepare cargo.
    let mut process = std::process::Command::new("cargo");
    // Only the matlab-sys crate should get published.
    process.arg("publish").arg("-p").arg("matlab-sys");
    // Forward the options given to the xtask publish command as if cargo publish was called manually
    for arg in arguments.finish() {
        process.arg(arg);
    }
    process.spawn()?;

    Ok(())
}
