// Project: long
// Description: A Rust CLI tool to generate shell aliases and environment configurations from a TOML file.
// src/main.rs

mod utils;
use utils::args::cli;
use utils::get_config_file::get_config_file;
use utils::info::{print_ascii_art, print_command, print_config_path, print_os_info};

mod genshell;
use genshell::{
  bash_zsh::generate_bash_zsh, fish::generate_fish, nushell::generate_nushell,
  powershell::generate_powershell,
};

fn main() {
  // 获取 CLI 参数
  let matches = cli().get_matches();
  let shell = matches.get_one::<String>("shell").unwrap();
  let config = matches
    .get_one::<String>("config")
    .map(|s| s.as_str())
    .unwrap_or("None");
  let no_output = matches.get_flag("no_output");
  let config_path = get_config_file(if config == "None" { None } else { Some(config) });

  if !no_output {
    print_ascii_art();
    print_os_info();
    print_config_path(config_path.to_str().unwrap());
    print_command(
      shell.as_str(),
      if config == "None" {
        None
      } else {
        config_path.to_str()
      },
    );
    println!("")
  }

  // 生成 shell 配置
  match shell.as_str() {
    "bash" => generate_bash_zsh(config_path),
    "zsh" => generate_bash_zsh(config_path),
    "fish" => generate_fish(config_path),
    "powershell" => generate_powershell(config_path),
    "nushell" => generate_nushell(config_path),
    _ => println!("未知的 shell 类型"),
  }
}
