use clap::{arg, command, Parser};
use libtigen::{build_image, write_dockerfile, Docker, Error, ImageMetadata, ImageName};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "Linux distribution")]
    image: String,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let image = args.image.as_ref();
    let image = ImageName::parse(image)?;
    let image = ImageMetadata::try_new(&image)?;
    write_dockerfile(&image)?;
    let oci_image_builder = Docker::default();
    build_image(oci_image_builder, &image)
}
