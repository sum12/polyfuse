#![allow(clippy::unnecessary_mut_passed)]
#![deny(clippy::unimplemented)]

use polyfuse_examples::prelude::*;

use examples::memfs::MemFs;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    examples::init_tracing()?;

    let mountpoint = examples::get_mountpoint()?;
    anyhow::ensure!(mountpoint.is_dir(), "the mountpoint must be a directory");

    let memfs = MemFs::default();
    polyfuse_tokio::run(memfs, mountpoint).await?;

    Ok(())
}