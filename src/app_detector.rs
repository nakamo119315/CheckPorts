//! Application type detection.
//!
//! This module provides functionality to detect what type of application
//! is running based on process information (command line, name, etc.).

use crate::models::{AppType, ProcessInfo};

/// Detects the application type from process information.
///
/// Uses pattern matching on the command line and process name
/// to identify common development frameworks and servers.
pub fn detect_app_type(process: &ProcessInfo) -> AppType {
    // Combine name and command for matching
    let name_lower = process.name.to_lowercase();
    let command_lower = process
        .command
        .as_ref()
        .map(|c| c.to_lowercase())
        .unwrap_or_default();

    // Check patterns in order of specificity

    // Node.js patterns
    if name_lower.contains("node")
        || command_lower.contains("node ")
        || command_lower.contains("npm ")
        || command_lower.contains("yarn ")
        || command_lower.contains("npx ")
        || command_lower.contains("next ")
        || command_lower.contains("react-scripts")
        || command_lower.contains("vite")
        || command_lower.contains("webpack")
    {
        return AppType::NodeJs;
    }

    // Python patterns
    if name_lower.contains("python")
        || command_lower.contains("python ")
        || command_lower.contains("python3 ")
        || command_lower.contains("uvicorn ")
        || command_lower.contains("gunicorn ")
        || command_lower.contains("flask ")
        || command_lower.contains("django")
        || command_lower.contains("fastapi")
    {
        return AppType::Python;
    }

    // .NET patterns
    if name_lower.contains("dotnet")
        || command_lower.contains("dotnet ")
        || command_lower.ends_with(".dll")
        || command_lower.contains(".dll ")
    {
        return AppType::DotNet;
    }

    // Java patterns
    if name_lower.contains("java")
        || command_lower.contains("java ")
        || command_lower.ends_with(".jar")
        || command_lower.contains(".jar ")
        || command_lower.contains("spring")
        || command_lower.contains("tomcat")
        || command_lower.contains("jetty")
    {
        return AppType::Java;
    }

    // Rust patterns (must be before Go since "cargo run" contains "go run")
    if command_lower.contains("cargo run")
        || command_lower.contains("/target/debug/")
        || command_lower.contains("/target/release/")
    {
        return AppType::Rust;
    }

    // Go patterns
    if command_lower.contains("go run")
        || command_lower.contains("gin")
        || command_lower.contains("echo")
        || command_lower.contains("fiber")
    {
        return AppType::Go;
    }

    // Ruby patterns
    if name_lower.contains("ruby")
        || command_lower.contains("ruby ")
        || command_lower.contains("rails ")
        || command_lower.contains("puma ")
        || command_lower.contains("unicorn ")
        || command_lower.contains("bundle exec")
    {
        return AppType::Ruby;
    }

    // PHP patterns
    if name_lower.contains("php")
        || command_lower.contains("php ")
        || command_lower.contains("artisan ")
        || command_lower.contains("laravel")
    {
        return AppType::Php;
    }

    // Nginx patterns
    if name_lower.contains("nginx") || command_lower.contains("nginx") {
        return AppType::Nginx;
    }

    // Apache patterns
    if name_lower.contains("httpd")
        || name_lower.contains("apache")
        || command_lower.contains("httpd")
        || command_lower.contains("apache")
    {
        return AppType::Apache;
    }

    AppType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_process(name: &str, command: Option<&str>) -> ProcessInfo {
        let mut p = ProcessInfo::new(1234, name);
        if let Some(cmd) = command {
            p.command = Some(cmd.to_string());
        }
        p
    }

    #[test]
    fn test_detect_nodejs() {
        let process = make_process("node", Some("node server.js"));
        assert_eq!(detect_app_type(&process), AppType::NodeJs);

        let process = make_process("npm", Some("npm run dev"));
        assert_eq!(detect_app_type(&process), AppType::NodeJs);
    }

    #[test]
    fn test_detect_python() {
        let process = make_process("python3", Some("python3 app.py"));
        assert_eq!(detect_app_type(&process), AppType::Python);

        let process = make_process("uvicorn", Some("uvicorn main:app"));
        assert_eq!(detect_app_type(&process), AppType::Python);
    }

    #[test]
    fn test_detect_dotnet() {
        let process = make_process("dotnet", Some("dotnet run"));
        assert_eq!(detect_app_type(&process), AppType::DotNet);

        let process = make_process("MyApp", Some("/app/MyApp.dll"));
        assert_eq!(detect_app_type(&process), AppType::DotNet);
    }

    #[test]
    fn test_detect_java() {
        let process = make_process("java", Some("java -jar app.jar"));
        assert_eq!(detect_app_type(&process), AppType::Java);
    }

    #[test]
    fn test_detect_rust() {
        let process = make_process("myapp", Some("cargo run"));
        assert_eq!(detect_app_type(&process), AppType::Rust);

        let process = make_process("myapp", Some("/target/debug/myapp"));
        assert_eq!(detect_app_type(&process), AppType::Rust);
    }

    #[test]
    fn test_detect_unknown() {
        let process = make_process("someapp", Some("/usr/bin/someapp"));
        assert_eq!(detect_app_type(&process), AppType::Unknown);
    }
}
