use std::env;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use owo_colors::OwoColorize;
use toml::Value;

pub fn get_config_file(path: Option<&str>) -> PathBuf {
  // 如果参数为空，使用默认路径
  let raw_path = path.unwrap_or("~/.config/long/long.toml");

  // 处理 ~ 展开为家目录
  if raw_path.starts_with('~') {
    // 获取家目录（跨平台方式）
    let home_dir = env::var("HOME")
      .or_else(|_| env::var("USERPROFILE")) // Windows 支持
      .expect("Failed to get home directory");

    // 替换 ~ 为家目录路径
    PathBuf::from(raw_path.replacen('~', &home_dir, 1))
  } else {
    PathBuf::from(raw_path)
  }
}

pub struct ReadTomlFileParams<'a> {
  pub path: PathBuf,
  pub gen_env: Option<Box<dyn Fn(&str, &Value) -> String + 'a>>,
  pub gen_alias: Option<Box<dyn Fn(&str, &Value) -> String + 'a>>,
}

pub fn read_toml_file<'a>(
  params: ReadTomlFileParams<'a>,
  mut result_content: Option<&mut String>,
) -> Result<toml::Value, Box<dyn std::error::Error>> {
  // 打开文件
  let mut file = File::open(params.path)?;

  // 读取文件内容到字符串
  let mut content = String::new();
  file.read_to_string(&mut content)?;

  // 解析 TOML 内容
  let value: toml::Value = toml::from_str(&content)?;

  // 生成环境变量配置
  if let Some(env_var_table) = value.get("env_var").and_then(|v| v.as_table()) {
    if let Some(gen_env) = params.gen_env {
      for (k, v) in env_var_table {
        let env_line = gen_env(k, v);
        if let Some(result_content) = result_content.as_deref_mut() {
          result_content.push_str(&env_line);
        } else {
          // 如果没有提供 result_content，则直接打印
          println!("{}", env_line.bold().blue());
        }
      }
    }
  }

  if let Some(result_content) = result_content.as_deref_mut() {
    result_content.push_str("\n");
  }

  // 生成别名配置
  if let Some(alias_table) = value.get("alias").and_then(|v| v.as_table()) {
    if let Some(gen_alias) = params.gen_alias {
      for (k, v) in alias_table {
        let alias_line = gen_alias(k, v);
        if let Some(result_content) = result_content.as_deref_mut() {
          result_content.push_str(&alias_line);
        } else {
          // 如果没有提供 result_content，则直接打印
          println!("{}", alias_line.bold().blue());
        }
      }
    }
  }

  Ok(value)
}

#[allow(dead_code)]
fn main() {
  // 测试用例
  println!("Default: {:?}", get_config_file(None)); // ~/.config/long/long.toml
  println!("Custom: {:?}", get_config_file(Some("/etc/long.toml"))); // /etc/long.toml
  println!("With ~: {:?}", get_config_file(Some("~/my_config.toml"))); // ~/my_config.toml → 展开为家目录

  let config_path = get_config_file(Some("long_template.toml"));
  // 读取并解析 TOML 文件
  match read_toml_file(
    ReadTomlFileParams {
      path: config_path,
      gen_env: None,
      gen_alias: None,
    },
    None,
  ) {
    Ok(value) => {
      println!("TOML Data: {:?}\n", value);
      if let Some(env_var) = value.get("env_var") {
        println!("Env Var: {}\n", env_var);
      }
      if let Some(alias) = value.get("alias") {
        println!("Alias: {}", alias);
      }
    }
    Err(e) => eprintln!("Failed to read TOML file: {}", e),
  }
}
