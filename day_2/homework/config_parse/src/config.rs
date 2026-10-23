use std::fmt;
use std::fs;
use std::io;

pub struct ScanConfig {
    pub target: String,
    pub ports: (u16, u16),
    pub threads: usize,
}

#[derive(Debug)]

pub enum ConfigError {
    Read(io::Error),
    Line(usize, String),
    Missing(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Read(error) => {
                write!(f, "cannot read config: {error}")
            }
            ConfigError::Line(line, message) => {
                write!(f, "line {line}: {message}")
            }
            ConfigError::Missing(key) => {
                write!(f, "missing key '{key}'")
            }
        }
    }
}

impl std::error::Error for ConfigError {}

pub fn load(path: &str) -> Result<ScanConfig, ConfigError> {
    let content = fs::read_to_string(path).map_err(ConfigError::Read)?;

    let mut target: Option<String> = None;
    let mut ports: Option<(u16, u16)> = None;
    let mut threads: Option<usize> = None;

    for (index, raw_line) in content.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((key, value)) = line.split_once('=') else {
            return Err(ConfigError::Line(
                line_number,
                "expected key=value".to_string(),
            ));
        };

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() || value.is_empty() {
            return Err(ConfigError::Line(
                line_number,
                "key and value must not be empty".to_string(),
            ));
        }

        match key {
            "target" => {
                target = Some(value.to_string());
            }
            "ports" => {
                let Some((start, end)) = value.split_once('-') else {
                    return Err(ConfigError::Line(
                        line_number,
                        "expected ports in start-end format".to_string(),
                    ));
                };

                let start = start.parse::<u16>().map_err(|_| {
                    ConfigError::Line(line_number, "invalid starting port".to_string())
                })?;

                let end = end.parse::<u16>().map_err(|_| {
                    ConfigError::Line(line_number, "invalid ending port".to_string())
                })?;

                if start > end {
                    return Err(ConfigError::Line(
                        line_number,
                        "starting port must not exceed ending port".to_string(),
                    ));
                }

                ports = Some((start, end));
            }
            "threads" => {
                let count = value.parse::<usize>().map_err(|_| {
                    ConfigError::Line(
                        line_number,
                        "threads must be a non-negative integer".to_string(),
                    )
                })?;

                threads = Some(count);
            }
            _ => {
                return Err(ConfigError::Line(
                    line_number,
                    format!("unknown key '{key}'"),
                ));
            }
        }
    }

    Ok(ScanConfig {
        target: target.ok_or(ConfigError::Missing("target"))?,
        ports: ports.ok_or(ConfigError::Missing("ports"))?,
        threads: threads.ok_or(ConfigError::Missing("threads"))?,
    })
}
