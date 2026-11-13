pub struct ScanConfig {
    pub target: String,
    pub ports: (u16, u16),
    pub threads: usize,
}

#[derive(Debug)]
pub enum ConfigError {
    Line(usize, String),
    Missing(&'static str),
}


pub fn parse(content: &str) -> Result<ScanConfig, Vec<ConfigError>> {
    let mut target = None;
    let mut ports = None;
    let mut threads = None;
    let mut errors = Vec::new();

    let mut target_found = false;
    let mut ports_found = false;
    let mut threads_found = false;

    for (index, raw_line) in content.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let (key, value) = match line.split_once('=') {
            Some(pair) => pair,
            None => {
                errors.push(ConfigError::Line(line_number, "expected key=value".into()));
                continue;
            }
        };

        let key = key.trim();
        let value = value.trim();

        if key.is_empty() || value.is_empty() {
            errors.push(ConfigError::Line(line_number, "empty key or value".into()));
            continue;
        }

        match key {
            "target" => {
                target_found = true;
                target = Some(value.to_string());
            }

            "ports" => {
                ports_found = true;
                let (start, end) = match value.split_once('-') {
                    Some(pair) => pair,
                    None => {
                        errors.push(ConfigError::Line(line_number, "expected start-end".into()));
                        continue;
                    }
                };

                let start = match start.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        errors.push(ConfigError::Line(line_number, "invalid starting port".into()));
                        continue;
                    }
                };

                let end = match end.parse::<u16>() {
                    Ok(port) => port,
                    Err(_) => {
                        errors.push(ConfigError::Line(line_number, "invalid ending port".into()));
                        continue;
                    }
                };

                if start > end {
                    errors.push(ConfigError::Line(line_number, "invalid port range".into()));
                    continue;
                }

                ports = Some((start, end));
            }

            "threads" => {
                threads_found = true;
                match value.parse::<usize>() {
                    Ok(count) => threads = Some(count),
                    Err(_) => errors.push(ConfigError::Line(line_number, "invalid threads".into())),
                }
            }

            _ => errors.push(ConfigError::Line(
                line_number,
                format!("unknown key '{key}'"),
            )),
        }
    }

    if !target_found {
        errors.push(ConfigError::Missing("target"));
    }
    if !ports_found {
        errors.push(ConfigError::Missing("ports"));
    }
    if !threads_found {
        errors.push(ConfigError::Missing("threads"));
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(ScanConfig {
        target: target.unwrap(),
        ports: ports.unwrap(),
        threads: threads.unwrap(),
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_config() {
        let content = "target=localhost\nports=1-1024\nthreads=4";

        let config = parse(content).unwrap();

        assert_eq!(config.target, "localhost");
        assert_eq!(config.ports, (1, 1024));
        assert_eq!(config.threads, 4);
    }

    #[test]
    fn missing_target() {
        let content = "ports=1-1024\nthreads=4";

        assert!(matches!(
            parse(content),
            Err(ConfigError::Missing("target"))
        ));
    }

    #[test]
    fn invalid_ports() {
        let content = "target=localhost\nports=abc\nthreads=4";

        assert!(parse(content).is_err());
    }

    #[test]
    fn invalid_line() {
        let content = "target=localhost\nwrong_line\nthreads=4";

        assert!(parse(content).is_err());
    }
}
