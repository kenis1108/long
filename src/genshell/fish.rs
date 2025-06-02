use std::path::PathBuf;

use toml::Value;

use crate::utils::get_config_file::{ReadTomlFileParams, read_toml_file};

fn generate_fish_env(key: &String, value: &Value) -> String {
  match value {
    Value::String(s) => format!("set -x {key} \"{s}\"\n"),
    Value::Table(t) => {
      let val = match t.get("value") {
        Some(Value::String(s)) => s.as_str(),
        Some(Value::Array(arr)) => {
          let arr_str: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
          &arr_str.join("\" \"")
        }
        _ => "",
      };
      let splice_before = t
        .get("splice_before")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
      let splice_after = t
        .get("splice_after")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
      if splice_before {
        format!("set -x {key} \"{val}\" ${key}\n")
      } else if splice_after {
        format!("set -x {key} ${key} \"{val}\"\n")
      } else {
        format!("set -x {key} \"{val}\"\n")
      }
    }
    _ => format!("\n"),
  }
}

fn generate_fish_alias(key: &String, value: &Value) -> String {
  match value {
    Value::String(s) => format!("alias {key} '{s}'\n"),
    Value::Table(t) => {
      let system_str = match t.get("system") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => {
          let arr_str: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
          arr_str.join(",")
        }
        _ => String::new(),
      };

      let shell_str = match t.get("shell") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => {
          let arr_str: Vec<String> = arr
            .iter()
            .filter_map(|v| v.as_str().map(|s| s.to_string()))
            .collect();
          arr_str.join(",")
        }
        _ => String::new(),
      };

      // 判断该 alias 是否适用于当前系统和 shell
      if !system_str.is_empty() && !system_str.contains(&std::env::consts::OS) {
        return format!("# alias {key} is not applicable for this system\n");
      }
      if !shell_str.is_empty() && !shell_str.contains("fish") {
        return format!("# alias {key} is not applicable for this shell\n");
      }

      let cmd = t.get("cmd");
      let cmd_str = match cmd {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Array(arr)) => {
          let mut iter = arr.iter();
          if let Some(first) = iter.next().and_then(|v| v.as_str()) {
            let args: Vec<String> = iter
              .filter_map(|v| v.as_str().map(|s| s.to_string()))
              .collect();
            if args.is_empty() {
              first.to_string()
            } else {
              // fish: first (args...)
              let arg_str = args.join(" ");
              format!("{} ({})", first, arg_str)
            }
          } else {
            String::new()
          }
        }
        _ => String::new(),
      };

      // 生成 alias 命令
      if !cmd_str.is_empty() {
        format!("alias {key} '{cmd_str}'\n")
      } else {
        format!("# invalid alias for {key}\n")
      }
    }
    _ => format!("\n"),
  }
}

pub fn generate_fish(config_path: PathBuf) {
  let mut fish_config = String::new();
  let params = ReadTomlFileParams {
    path: config_path,
    gen_env: Some(Box::new(|k, v| generate_fish_env(&k.to_string(), v))),
    gen_alias: Some(Box::new(|k, v| generate_fish_alias(&k.to_string(), v))),
  };
  match read_toml_file(params, Some(&mut fish_config)) {
    Ok(_) => println!("{}", fish_config),
    Err(e) => eprintln!("Error reading TOML file: {}", e),
  }
}
