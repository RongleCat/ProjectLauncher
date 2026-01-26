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
        use std::path::Path;

        let app_name = Path::new(app_path)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        println!("[Launcher] 执行命令: {} {}", app_path, project_path);

        // Windows Terminal 需要 -d 参数指定工作目录
        if app_name == "wt.exe" || app_name == "windowsterminal.exe" {
            Command::new(app_path)
                .args(["-d", project_path])
                .spawn()?;
        } else {
            Command::new(app_path)
                .arg(project_path)
                .spawn()?;
        }
        Ok(())
    }

    #[cfg(target_os = "macos")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        println!("[Launcher] 执行命令: open -a \"{}\" \"{}\"", app_path, project_path);
        Command::new("open")
            .arg("-a")
            .arg(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    #[cfg(target_os = "linux")]
    fn open_with_app(app_path: &str, project_path: &str) -> Result<()> {
        println!("[Launcher] 执行命令: {} {}", app_path, project_path);
        Command::new(app_path)
            .arg(project_path)
            .spawn()?;
        Ok(())
    }

    fn execute_command(cmd: &str, project_path: &str) -> Result<()> {
        let cmd_replaced = cmd.replace("{project}", project_path);
        println!("[Launcher] 执行命令: {}", cmd_replaced);

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;

            // 解析命令，提取可执行文件和参数
            let parts: Vec<&str> = cmd_replaced.split_whitespace().collect();
            if let Some((exe, _)) = parts.split_first() {
                // 重新组装参数，替换 {project} 后的参数
                let args_str: String = cmd_replaced
                    .trim_start_matches(exe)
                    .trim()
                    .to_string();

                Command::new(exe)
                    .raw_arg(&args_str)
                    .creation_flags(CREATE_NO_WINDOW)
                    .spawn()?;
            }
        }

        #[cfg(not(target_os = "windows"))]
        {
            Command::new("sh")
                .args(["-c", &cmd_replaced])
                .spawn()?;
        }

        Ok(())
    }
}
