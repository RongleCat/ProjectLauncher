use std::process::Command;
use crate::models::launcher::Launcher;
use anyhow::Result;

pub struct LauncherService;

impl LauncherService {
    /// 使用启动器打开项目
    pub fn launch(launcher: &Launcher, project_path: &str) -> Result<()> {
        if launcher.is_command {
            // 命令模式：执行自定义命令
            if let Some(cmd) = &launcher.command {
                Self::execute_command(cmd, project_path)?;
            }
        } else {
            // 应用模式：直接调用应用打开项目
            Self::open_with_app(&launcher.path, project_path)?;
        }
        Ok(())
    }

    #[cfg(target_os = "windows")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new("open")
            .arg("-a")
            .arg(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        Command::new(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    fn execute_command(cmd: &str, project_path: &str) -> Result<()> {
        let cmd_replaced = cmd.replace("{project}", project_path);

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(&["/C", &cmd_replaced])
                .spawn()?;
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new("sh")
                .args(&["-c", &cmd_replaced])
                .spawn()?;
        }

        Ok(())
    }
}
