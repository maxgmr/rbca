use vergen::{BuildBuilder, Emitter, SysinfoBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::default().build_date(true).build()?;
    let si = SysinfoBuilder::default()
        .memory(true)
        .os_version(true)
        .build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&si)?
        .emit()?;

    Ok(())
}
