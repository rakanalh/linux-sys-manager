use std::process::ExitStatus;

use crate::utils::execute_command;

pub trait OperatingSystem {
    fn init() -> anyhow::Result<Self>
    where
        Self: Sized;
    fn package_manager(&self) -> &'static str;

    fn install_packages(&self, packages: Vec<String>) -> anyhow::Result<ExitStatus>;

    fn user_exists(&self, user: &str) -> anyhow::Result<bool> {
        execute_command(format!("id -u {}", user)).map(|status| status.success())
    }

    fn user_create(
        &self,
        user: &str,
        groups: Vec<String>,
        default_shell: String,
    ) -> anyhow::Result<ExitStatus> {
        execute_command(format!(
            "useradd -m -G {} -s {} {}",
            groups.join(","),
            default_shell,
            user
        ))
    }

    fn user_update(
        &self,
        user: &str,
        groups: Vec<String>,
        default_shell: String,
    ) -> anyhow::Result<ExitStatus> {
        execute_command(format!(
            "usermod -G {} -s {} {}",
            groups.join(","),
            default_shell,
            user
        ))
    }
}
