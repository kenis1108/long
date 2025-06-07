use owo_colors::OwoColorize;

/// 打印 ASCII 艺术字
pub fn print_ascii_art() {
  println!(
    "{}",
    r#"
▄▄▄     ▄▄▄▄▄▄▄ ▄▄    ▄ ▄▄▄▄▄▄▄
█   █   █       █  █  █ █       █
█   █   █   ▄   █   █▄█ █   ▄▄▄▄█
█   █   █  █ █  █       █  █  ▄▄
█   █▄▄▄█  █▄█  █  ▄    █  █ █  █
█       █       █ █ █   █  █▄▄█ █
█▄▄▄▄▄▄▄█▄▄▄▄▄▄▄█▄█  █▄▄█▄▄▄▄▄▄▄█ 
    "#
    .bold()
    .red()
  );
}

/// 打印操作系统信息
pub fn print_os_info() {
  let os_info = format!("OS: {} {}", std::env::consts::OS, std::env::consts::ARCH);
  println!("{}", os_info.bold().green());
}

/// 打印配置文件路径
pub fn print_config_path(config_path: &str) {
  let config_path_info = format!("Config: {}", config_path);
  println!("{}", config_path_info.bold().green());
}

/// 打印快捷使用的命令
pub fn print_command(shell: &str, config_path: Option<&str>) {
  let mut command = String::new();
  let mut config_path_str = String::new();
  if !config_path.is_none() {
    config_path_str = format!("--config {}", config_path.unwrap());
  }
  match shell {
    "bash" | "zsh" => {
      command = format!(
        "Command: eval \"$(long --shell {} {} --no_output)\"",
        shell, config_path_str
      );
    }
    "fish" => {
      command = format!(
        "Command: long --shell {} {} --no_output | source",
        shell, config_path_str
      );
    }
    "powershell" => {
      command = format!(
        "Command: long --shell {} {} --no_output | Out-String | Invoke-Expression",
        shell, config_path_str
      );
    }
    "nushell" => {
      command = format!(
        "Command: \nmkdir ($nu.data-dir | path join \"vendor/autoload\")\nlong --shell {} {}  --no_output | save -f ($nu.data-dir | path join \"vendor/autoload/long.nu\")",
        shell, config_path_str
      );
    }
    _ => println!("Unsupported shell type: {}", shell),
  }
  println!("{}", command.bold().green())
}
