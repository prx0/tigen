use std::process::ExitCode;

use clap::{arg, command, Parser};
use libtigen::{build_image, write_dockerfile, Docker, Error, ImageMetadata, ImageName, Toolbox};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Linux distribution")]
    image: String,

    #[arg(
        short,
        long,
        help = "Enter into a new toolbox",
        default_value = "false"
    )]
    enter: bool,
}

fn main() -> Result<ExitCode, Error> {
    let args = Args::parse();
    let image = args.image.as_ref();
    let image_name = ImageName::parse(image)?;
    let image = ImageMetadata::try_new(&image_name)?;
    write_dockerfile(&image)?;
    let oci_image_builder = Docker::default();
    build_image(oci_image_builder, &image)?;
    if args.enter {
        let toolbox = Toolbox::default();
        toolbox.create(&image_name)?;
        toolbox.enter()?;
    }
    Ok(ExitCode::SUCCESS)
}
