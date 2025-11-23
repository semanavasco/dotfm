use std::env;
use std::path::PathBuf;

pub fn expand_path(input: &str) -> PathBuf {
    let expanded = expand_env_vars(&expand_tilde(input));

    let p = PathBuf::from(expanded);
    if p.is_absolute() {
        p
    } else {
        env::current_dir().map(|cwd| cwd.join(&p)).unwrap_or(p)
    }
}

fn expand_tilde(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return format!("{}/{}", home.display(), stripped);
        }
    }
    if path == "~" {
        if let Some(home) = home_dir() {
            return home.display().to_string();
        }
    }
    path.to_string()
}

fn expand_env_vars(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '$' {
            if matches!(chars.peek(), Some('{')) {
                chars.next();
                let mut name = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc == '}' {
                        chars.next();
                        break;
                    }
                    name.push(nc);
                    chars.next();
                }
                if !name.is_empty() {
                    out.push_str(&env::var(&name).unwrap_or_default());
                }
                continue;
            }

            let mut name = String::new();
            while let Some(&nc) = chars.peek() {
                if nc.is_ascii_alphanumeric() || nc == '_' {
                    name.push(nc);
                    chars.next();
                } else {
                    break;
                }
            }
            if !name.is_empty() {
                out.push_str(&env::var(&name).unwrap_or_default());
                continue;
            }

            out.push('$');
            continue;
        }
        out.push(c);
    }
    out
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME").map(PathBuf::from)
}
