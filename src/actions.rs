use log::info;

pub fn perform_action(script: &str) {
    info!("Executing script '{}'", script);

    std::process::Command::new(script)
        .output()
        .expect("Failed to execute script.");

    std::process::exit(0);
}
