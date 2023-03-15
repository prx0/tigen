pub use error::*;
use nom::bytes::complete::take_while1;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::IResult;
use package_manager::{Apt, Dnf, PackageManager, Pacman, Zypper};
use std::io;
use std::process::{Command, Output};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    str::FromStr,
};
use tera::{Context, Tera};
pub mod error;
pub mod package_manager;

const DOCKERFILE: &str = "Dockerfile";
const TEMPLATE_DIR: &str = "templates/*";

#[derive(Debug, Clone)]
pub enum Distro {
    Ubuntu(Apt),
    Debian(Apt),
    Archlinux(Pacman),
    OpenSuse(Zypper),
    Fedora(Dnf),
}

impl Distro {
    fn run_layer(self) -> String {
        match self {
            Self::Archlinux(pm) => run_layer(pm),
            Self::Debian(pm) => run_layer(pm),
            Self::Ubuntu(pm) => run_layer(pm),
            Self::OpenSuse(pm) => run_layer(pm),
            Self::Fedora(pm) => run_layer(pm),
        }
    }
}

impl FromStr for Distro {
    type Err = DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let distro = match s.to_lowercase().as_str() {
            "ubuntu" => Distro::Ubuntu(Apt::default()),
            "debian" => Distro::Debian(Apt::default()),
            "archlinux" => Distro::Archlinux(Pacman::default()),
            "opensuse" => Distro::OpenSuse(Zypper::default()),
            "fedora" => Distro::Fedora(Dnf::default()),
            _ => unimplemented!("no support for {}", s),
        };
        Ok(distro)
    }
}

pub struct Toolbox {
    bin: String,
}

impl Toolbox {
    pub fn new() -> Self {
        Self {
            bin: "toolbox".to_string(),
        }
    }

    pub fn create(&self, image: &ImageName<'_>) -> Result<Output, Error> {
        let child = Command::new(&self.bin)
            .args(vec!["create", "-i", &image.to_string()])
            .spawn()?;
        let output = child.wait_with_output()?;
        Ok(output)
    }

    pub fn enter(&self) -> Result<Output, Error> {
        let child = Command::new(&self.bin).args(vec!["enter"]).spawn()?;
        let output = child.wait_with_output()?;
        Ok(output)
    }
}

impl Default for Toolbox {
    fn default() -> Self {
        Toolbox::new()
    }
}

pub trait ImageBuilder {
    fn build_image(&self, image: &ImageMetadata<'_>) -> Result<Output, Error>;
}

pub struct Docker {
    bin: String,
}

impl Docker {
    fn new() -> Self {
        Self {
            bin: "docker".to_string(),
        }
    }
}

impl Default for Docker {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageBuilder for Docker {
    fn build_image(&self, image: &ImageMetadata<'_>) -> Result<Output, Error> {
        let s = image.dir().join(Path::new("Dockerfile"));
        let child = Command::new(&self.bin)
            .args(vec![
                "build",
                "-f",
                s.to_str().unwrap(),
                "-t",
                &image.name(),
                ".",
            ])
            .spawn()?;
        let output = child.wait_with_output()?;
        Ok(output)
    }
}

pub struct Podman {
    bin: String,
}

impl Podman {
    fn new() -> Self {
        Self {
            bin: "podman".to_string(),
        }
    }
}

impl Default for Podman {
    fn default() -> Self {
        Self::new()
    }
}

impl ImageBuilder for Podman {
    fn build_image(&self, image: &ImageMetadata<'_>) -> Result<Output, Error> {
        let output = Command::new(&self.bin)
            .args(vec![
                "build",
                "-t",
                &image.name(),
                "-",
                "<",
                &image.dockerfile,
            ])
            .output()?;
        Ok(output)
    }
}

pub fn run_layer(package_manager: impl PackageManager) -> String {
    let update = package_manager.update().join(" ");
    let upgrade = package_manager.upgrade().join(" ");
    let packages = vec!["sudo"];
    let install = package_manager.install(packages.into_iter()).join(" ");
    format!("{} && {} && {}", update, upgrade, install)
}

#[derive(Debug)]
pub struct ImageMetadata<'a> {
    image: &'a ImageName<'a>,
    dockerfile: String,
}

impl<'a> ImageMetadata<'a> {
    pub fn try_new(image: &'a ImageName<'a>) -> Result<Self, Error> {
        let dist = Distro::from_str(image.name)?;
        let tera = Tera::new(TEMPLATE_DIR)?;

        let mut context = Context::new();
        context.insert("name", image.name);
        context.insert("version", image.tag);
        context.insert("run_layer", &dist.run_layer());

        let dockerfile = tera.render(DOCKERFILE, &context)?;
        Ok(Self { image, dockerfile })
    }

    fn dir(&self) -> PathBuf {
        PathBuf::from(format!("images/{}/{}", self.image.name, self.image.tag))
    }

    fn name(&self) -> String {
        self.image.to_string()
    }
}

pub fn write_dockerfile(image: &ImageMetadata<'_>) -> Result<(), Error> {
    let path = image.dir();
    fs::create_dir_all(&path)?;
    let mut file = fs::File::create(path.join(Path::new("Dockerfile")))?;
    file.write_all(image.dockerfile.as_bytes())?;
    Ok(())
}

pub fn build_image(builder: impl ImageBuilder, image: &ImageMetadata<'_>) -> Result<(), Error> {
    let output = builder.build_image(image)?;
    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;
    Ok(())
}

#[derive(Debug)]
pub struct ImageName<'a> {
    name: &'a str,
    tag: &'a str,
}

fn word(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| is_alphanumeric(c as u8) || c == '-' || c == '_' || c == '.')(input)
}

impl<'a> ImageName<'a> {
    pub fn parse(s: &'a str) -> Result<Self, Error> {
        let (tag, name) = word(s).map_err(|e| Error::Nom(e.to_string()))?;

        let (_, tag) = opt(tuple((char(':'), word)))(tag).map_err(|e| Error::Nom(e.to_string()))?;
        Ok(Self {
            name,
            tag: tag.map(|(_, t)| t).unwrap_or("latest"),
        })
    }
}

impl<'a> ToString for ImageName<'a> {
    fn to_string(&self) -> String {
        format!("{}:{}", self.name, self.tag)
    }
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn parse_image_name() {
        struct Test<'a> {
            args: &'a str,
            expected: &'a str,
        }

        let images = vec![
            Test {
                args: "ubuntu:20.04",
                expected: "ubuntu:20.04",
            },
            Test {
                args: "archlinux",
                expected: "archlinux:latest",
            },
            Test {
                args: "fedora:37",
                expected: "fedora:37",
            },
            Test {
                args: "ubuntu:lunar-20230301",
                expected: "ubuntu:lunar-20230301",
            },
        ];

        for image in images {
            let parsed = ImageName::parse(image.args).expect("A valid image name");
            assert_eq!(image.expected, &parsed.to_string())
        }
    }
}
