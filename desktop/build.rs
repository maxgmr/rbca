use vergen_git2::{BuildBuilder, Emitter, Git2Builder, SysinfoBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let build = BuildBuilder::default().build_date(true).build()?;
    let git2 = Git2Builder::default()
        .commit_author_name(true)
        .commit_date(true)
        .build()?;
    let si = SysinfoBuilder::default()
        .memory(true)
        .name(true)
        .os_version(true)
        .build()?;

    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git2)?
        .add_instructions(&si)?
        .emit()?;

    Ok(())
}
