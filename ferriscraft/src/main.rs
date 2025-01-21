use color_eyre;

use voxlrs;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    voxlrs::welcome_message();

    Ok(())
}
