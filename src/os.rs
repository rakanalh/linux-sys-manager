use std::process::ExitStatus;

use crate::{
    constants::ARCH_SUPPORTED_PACKAGE_MANAGERS, traits::OperatingSystem, utils::execute_command,
};
use anyhow::bail;
use quale::which;

pub struct Arch {
    package_manager: &'static str,
}

impl OperatingSystem for Arch {
    fn init() -> anyhow::Result<Self> {
        let available = ARCH_SUPPORTED_PACKAGE_MANAGERS
            .iter()
            .filter(|exec| which(exec).is_some())
            .collect::<Vec<_>>();
        let exec = available.first();
        let package_manager = if let Some(exec) = exec {
            exec
        } else {
            bail!("Could not find the executable for the supported package manager");
        };
        Ok(Self { package_manager })
    }

    fn package_manager(&self) -> &'static str {
        self.package_manager
    }

    fn install_packages(&self, packages: Vec<String>) -> anyhow::Result<ExitStatus> {
        let packages = packages.join(" ");
        execute_command(format!(
            "{} -Sy --answerclean y --answerdiff n --answeredit n --answerupgrade y --removemake --cleanafter --noconfirm {}",
            self.package_manager,
            packages
        ))
    }
}
