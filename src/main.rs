use clap::{arg, command, Parser};
use std::{fmt::Display, str::FromStr};
use tera::{Context, Tera};

const DOCKERFILE: &str = "Dockerfile";
const TEMPLATE_DIR: &str = "templates/*";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    distro: String,

    #[arg(short, long, default_value = "latest")]
    release: String,
}

trait Installer {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str>;
}

trait Updater {
    fn update(&self) -> Vec<&str>;
}

trait Upgrader {
    fn upgrade(&self) -> Vec<&str>;
}

#[derive(Default, Debug, Clone)]
struct Apt {}

impl Installer for Apt {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["apt", "install", "-y"];
        install_command.into_iter().chain(packages).collect()
    }
}

impl Updater for Apt {
    fn update(&self) -> Vec<&str> {
        vec!["apt", "update", "-y"]
    }
}

impl Upgrader for Apt {
    fn upgrade(&self) -> Vec<&str> {
        vec!["apt", "upgrade", "-y"]
    }
}

#[derive(Default, Debug, Clone)]
struct Pacman {}

impl Installer for Pacman {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["pacman", "-S", "--noconfirm"];
        install_command.into_iter().chain(packages).collect()
    }
}

impl Updater for Pacman {
    fn update(&self) -> Vec<&str> {
        vec!["pacman", "-Syy", "--noconfirm"]
    }
}

impl Upgrader for Pacman {
    fn upgrade(&self) -> Vec<&str> {
        vec!["pacman", "-Su", "--noconfirm"]
    }
}

#[derive(Default, Debug, Clone)]
struct Zypper {}

impl Installer for Zypper {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["zypper", "install", "--non-interactive"];
        install_command.into_iter().chain(packages).collect()
    }
}

impl Updater for Zypper {
    fn update(&self) -> Vec<&str> {
        vec!["zypper", "refresh", "--non-interactive"]
    }
}

impl Upgrader for Zypper {
    fn upgrade(&self) -> Vec<&str> {
        vec!["zypper", "update", "--non-interactive"]
    }
}

#[derive(Default, Debug, Clone)]
struct Dnf {}

impl Installer for Dnf {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["dnf", "install", "-y"];
        install_command.into_iter().chain(packages).collect()
    }
}

impl Updater for Dnf {
    fn update(&self) -> Vec<&str> {
        vec!["dnf", "check-update", "-y"]
    }
}

impl Upgrader for Dnf {
    fn upgrade(&self) -> Vec<&str> {
        vec!["dnf", "upgrade", "-y"]
    }
}

#[derive(Debug, Clone)]
enum Distro {
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

#[derive(Debug)]
pub struct DecodingError(String);

impl Display for DecodingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl std::error::Error for DecodingError {}

#[derive(Debug)]
enum Error {
    Decoding(DecodingError),
    Templating(tera::Error),
}

impl From<DecodingError> for Error {
    fn from(err: DecodingError) -> Self {
        Self::Decoding(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Self::Templating(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

impl std::error::Error for Error {}

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

fn run_layer<P>(package_manager: P) -> String
where
    P: Installer + Updater + Upgrader,
{
    let update = package_manager.update().join(" ");
    let upgrade = package_manager.upgrade().join(" ");
    let packages = vec!["sudo"];
    let install = package_manager.install(packages.into_iter()).join(" ");
    format!("{} && {} && {}", update, upgrade, install)
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let distro = Distro::from_str(&args.distro)?;

    let tera = Tera::new(TEMPLATE_DIR)?;

    let mut context = Context::new();
    context.insert("name", &args.distro);
    context.insert("version", &args.release);
    context.insert("run_layer", &distro.run_layer());

    let rendered = tera.render(DOCKERFILE, &context)?;
    println!("{}", rendered);
    Ok(())
}
