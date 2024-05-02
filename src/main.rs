use color_eyre::eyre::Result;
use ratrace::run;

fn main() -> Result<()> {
    pollster::block_on(run())
}
