use log::info;

pub fn form_action(script: &str) {
    info!("Executing script '{}'", script);

    std::process::Command::new("sh")
        .arg(script)
        .output()
        .expect("Failed to execute script.");

    std::process::exit(0);
}
