fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = prost_build::Config::new();
    config.type_attribute("configmanager.Config", "#[derive(serde::Serialize, serde::Deserialize)]");
    config.type_attribute("configmanager.ConfigInformation", "#[derive(serde::Serialize, serde::Deserialize)]");

    tonic_build::configure()
        .compile_with_config(config, &["proto/config-manager.proto"], &["proto"])?;
    Ok(())
}