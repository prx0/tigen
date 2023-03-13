pub trait PackageManager {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str>;
    fn update(&self) -> Vec<&str>;
    fn upgrade(&self) -> Vec<&str>;
}

#[derive(Default, Debug, Clone)]
pub struct Apt {}

impl PackageManager for Apt {
    fn install<'a>(&'a self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["apt", "install", "-y"];
        install_command.into_iter().chain(packages).collect()
    }

    fn update(&self) -> Vec<&str> {
        vec!["apt", "update", "-y"]
    }

    fn upgrade(&self) -> Vec<&str> {
        vec!["apt", "upgrade", "-y"]
    }
}

#[derive(Default, Debug, Clone)]
pub struct Pacman {}

impl PackageManager for Pacman {
    fn install<'a>(&self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["pacman", "-S", "--noconfirm"];
        install_command.into_iter().chain(packages).collect()
    }

    fn update(&self) -> Vec<&str> {
        vec!["pacman", "-Syy", "--noconfirm"]
    }

    fn upgrade(&self) -> Vec<&str> {
        vec!["pacman", "-Su", "--noconfirm"]
    }
}

#[derive(Default, Debug, Clone)]
pub struct Zypper {}

impl PackageManager for Zypper {
    fn install<'a>(&self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["zypper", "install", "--non-interactive"];
        install_command.into_iter().chain(packages).collect()
    }

    fn update(&self) -> Vec<&str> {
        vec!["zypper", "refresh", "--non-interactive"]
    }

    fn upgrade(&self) -> Vec<&str> {
        vec!["zypper", "update", "--non-interactive"]
    }
}

#[derive(Default, Debug, Clone)]
pub struct Dnf {}

impl PackageManager for Dnf {
    fn install<'a>(&self, packages: impl Iterator<Item = &'a str>) -> Vec<&'a str> {
        let install_command = vec!["dnf", "install", "-y"];
        install_command.into_iter().chain(packages).collect()
    }

    fn update(&self) -> Vec<&str> {
        vec!["dnf", "check-update", "-y"]
    }

    fn upgrade(&self) -> Vec<&str> {
        vec!["dnf", "upgrade", "-y"]
    }
}
