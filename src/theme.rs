use std::collections::HashMap;

pub struct Theme {
    pub color_definitions: HashMap<&'static str, &'static str>,
    pub known_file_extensions: HashMap<&'static str, &'static str>,
    pub known_file_extension_endings: HashMap<&'static str, &'static str>,
    pub known_file_names: HashMap<&'static str, &'static str>,
}

pub fn default_theme() -> Theme {
    let mut color_definitions = HashMap::new();
    color_definitions.insert("gray", "8");
    color_definitions.insert("purple", "13");
    color_definitions.insert("red", "1");
    color_definitions.insert("green", "2");
    color_definitions.insert("yellow", "11");
    color_definitions.insert("blue", "18");
    color_definitions.insert("dirty-red", "88");
    color_definitions.insert("dirty-yellow", "100");
    color_definitions.insert("dirty-green", "119");
    color_definitions.insert("pink", "125");

    let mut known_file_extensions = HashMap::new();
    known_file_extensions.insert("md", "200");
    known_file_extensions.insert("py", "119");
    known_file_extensions.insert("ts", "153");
    known_file_extensions.insert("tsx", "81");
    known_file_extensions.insert("js", "208");
    known_file_extensions.insert("jsx", "202");
    known_file_extensions.insert("json", "blue");
    known_file_extensions.insert("yml", "blue");
    known_file_extensions.insert("yaml", "blue");
    known_file_extensions.insert("txt", "blue");
    known_file_extensions.insert("png", "pink");
    known_file_extensions.insert("jpg", "pink");
    known_file_extensions.insert("jpeg", "pink");
    known_file_extensions.insert("gif", "pink");
    known_file_extensions.insert("zip", "dirty-yellow");
    known_file_extensions.insert("tar", "dirty-yellow");
    known_file_extensions.insert("gz", "dirty-yellow");

    let mut known_file_extension_endings = HashMap::new();
    known_file_extension_endings.insert("ignore", "gray");
    known_file_extension_endings.insert("rc", "yellow");
    known_file_extension_endings.insert("sh", "yellow");

    let mut known_file_names = HashMap::new();
    known_file_names.insert("README.md", "red");
    known_file_names.insert("LICENSE", "gray");
    known_file_names.insert("Makefile", "red");
    known_file_names.insert("Dockerfile", "red");
    known_file_names.insert("package.json", "red");
    known_file_names.insert("requirements.txt", "red");

    Theme {
        color_definitions,
        known_file_extensions,
        known_file_extension_endings,
        known_file_names,
    }
}

pub fn resolve_color_code(theme: &Theme, key: &str) -> String {
    if key.chars().all(|c| c.is_ascii_digit()) {
        return format!("\x1b[38;5;{key}m");
    }

    if let Some(number) = theme.color_definitions.get(key) {
        return format!("\x1b[38;5;{number}m");
    }

    String::new()
}
