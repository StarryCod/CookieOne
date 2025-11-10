use anyhow::Result;

pub fn init() -> Result<()> {
    info!("Audio subsystem initialized (placeholder)");
    Ok(())
}

pub fn play<S: AsRef<str>>(phrase: S) -> Result<()> {
    info!("AUDIO >> {}", phrase.as_ref());
    Ok(())
}
