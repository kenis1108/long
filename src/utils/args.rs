use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
  Command::new(env!("CARGO_PKG_NAME"))
    .version(env!("CARGO_PKG_VERSION"))
    .about("Generate shell alias and env config from a TOML file.")
    .arg_required_else_help(true)
    .arg(
      Arg::new("shell")
        .short('s')
        .long("shell")
        .value_name("SHELL")
        .required(true)
        .value_parser(["bash", "zsh", "fish", "powershell", "nushell"]) // 限制合法值
        .help("Target shell type: bash, zsh, fish, powershell, nushell")
        .num_args(1),
    )
    .arg(
      Arg::new("config")
        .short('c')
        .long("config")
        .value_name("FILE")
        .help("Sets a custom config file path, defaults to ~/.config/long/long.toml")
        .num_args(1),
    )
    .arg(
      Arg::new("no_output")
        .short('n')
        .long("no_output")
        .action(ArgAction::SetTrue)
        .help("don't use output messages, useful for eval/source commands"),
    )
}

#[allow(dead_code)]
fn main() {
  // 测试用例
  let matches = cli().get_matches();
  // 正确处理必填参数（shell 是必填，unwrap 安全）
  let shell = matches.get_one::<String>("shell").unwrap();
  println!("--shell: {}", shell);

  // 处理可选参数 config（避免 unwrap()）
  let config = matches
    .get_one::<String>("config")
    .map(|s| s.as_str())
    .unwrap_or("None");
  println!("--config: {}", config);

  // 处理标志位参数
  let no_output = matches.get_flag("no_output");
  println!("--no_output: {}", no_output);
}
