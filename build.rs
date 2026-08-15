use {
    std::error::Error,
    vergen_gitcl::{Build, Cargo, Emitter, Gitcl, Rustc},
};

fn main() -> Result<(), Box<dyn Error>> {
    let gitcl = Gitcl::builder().sha(false).commit_timestamp(true).build();
    let build = Build::builder().build_timestamp(true).build();
    let cargo = Cargo::builder()
        .target_triple(true)
        .debug(true)
        .opt_level(true)
        .features(true)
        .build();
    let rustc = Rustc::builder().semver(true).build();
    Emitter::default()
        .add_instructions(&gitcl)?
        .add_instructions(&build)?
        .add_instructions(&cargo)?
        .add_instructions(&rustc)?
        .emit()?;
    Ok(())
}
