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
    known_file_extensions.insert("ds_store", "gray");
    known_file_extensions.insert("old", "gray");
    known_file_extensions.insert("swp", "gray");
    known_file_extensions.insert("tmp", "gray");
    known_file_extensions.insert("temp", "gray");
    known_file_extensions.insert("bak", "gray");
    known_file_extensions.insert("bkp", "gray");
    known_file_extensions.insert("log", "gray");
    known_file_extensions.insert("map", "gray");
    known_file_extensions.insert("pyc", "gray");
    known_file_extensions.insert("license", "gray");
    known_file_extensions.insert("mjs", "gray");
    known_file_extensions.insert("cjs", "gray");
    known_file_extensions.insert("mts", "gray");
    known_file_extensions.insert("cts", "gray");
    known_file_extensions.insert("eslintcache", "gray");
    known_file_extensions.insert("cache", "gray");
    known_file_extensions.insert("gitkeep", "gray");

    known_file_extensions.insert("pub", "red");
    known_file_extensions.insert("private", "red");
    known_file_extensions.insert("key", "red");
    known_file_extensions.insert("lock", "red");

    known_file_extensions.insert("cfg", "purple");
    known_file_extensions.insert("conf", "purple");
    known_file_extensions.insert("ini", "purple");
    known_file_extensions.insert("properties", "purple");
    known_file_extensions.insert("config", "purple");
    known_file_extensions.insert("gitignore", "purple");
    known_file_extensions.insert("npmignore", "purple");
    known_file_extensions.insert("dockerignore", "purple");
    known_file_extensions.insert("stylelintignore", "purple");
    known_file_extensions.insert("editorconfig", "purple");

    known_file_extensions.insert("asm", "yellow");
    known_file_extensions.insert("c", "yellow");
    known_file_extensions.insert("class", "yellow");
    known_file_extensions.insert("cpp", "yellow");
    known_file_extensions.insert("cs", "yellow");
    known_file_extensions.insert("h", "yellow");
    known_file_extensions.insert("hpp", "yellow");
    known_file_extensions.insert("php", "yellow");
    known_file_extensions.insert("jar", "yellow");

    known_file_extensions.insert("bat", "yellow");
    known_file_extensions.insert("exe", "yellow");
    known_file_extensions.insert("bin", "yellow");
    known_file_extensions.insert("wsf", "yellow");
    known_file_extensions.insert("msi", "yellow");

    known_file_extensions.insert("csv", "blue");
    known_file_extensions.insert("tsv", "blue");
    known_file_extensions.insert("psv", "blue");
    known_file_extensions.insert("json", "blue");
    known_file_extensions.insert("wasm", "blue");
    known_file_extensions.insert("txt", "blue");
    known_file_extensions.insert("xml", "blue");
    known_file_extensions.insert("yaml", "blue");
    known_file_extensions.insert("yml", "blue");
    known_file_extensions.insert("dat", "blue");

    known_file_extensions.insert("css", "123");
    known_file_extensions.insert("htm", "green");
    known_file_extensions.insert("html", "green");
    known_file_extensions.insert("js", "208");
    known_file_extensions.insert("jsx", "202");
    known_file_extensions.insert("md", "200");
    known_file_extensions.insert("py", "119");
    known_file_extensions.insert("ts", "153");
    known_file_extensions.insert("tsx", "81");
    known_file_extensions.insert("sql", "94");

    known_file_extensions.insert("jpg", "pink");
    known_file_extensions.insert("jpeg", "pink");
    known_file_extensions.insert("svg", "pink");
    known_file_extensions.insert("png", "pink");
    known_file_extensions.insert("gif", "pink");
    known_file_extensions.insert("bmp", "pink");
    known_file_extensions.insert("webp", "pink");
    known_file_extensions.insert("tif", "pink");
    known_file_extensions.insert("tiff", "pink");
    known_file_extensions.insert("psd", "pink");
    known_file_extensions.insert("ai", "pink");
    known_file_extensions.insert("ico", "pink");
    known_file_extensions.insert("heic", "pink");

    known_file_extensions.insert("dmg", "yellow");
    known_file_extensions.insert("iso", "yellow");

    known_file_extensions.insert("webm", "dirty-green");
    known_file_extensions.insert("ogg", "dirty-green");
    known_file_extensions.insert("flac", "dirty-green");
    known_file_extensions.insert("aac", "dirty-green");
    known_file_extensions.insert("aif", "dirty-green");
    known_file_extensions.insert("mp3", "dirty-green");
    known_file_extensions.insert("wav", "dirty-green");
    known_file_extensions.insert("mp4", "dirty-green");
    known_file_extensions.insert("avi", "dirty-green");
    known_file_extensions.insert("mov", "dirty-green");
    known_file_extensions.insert("otf", "dirty-green");
    known_file_extensions.insert("ttf", "dirty-green");
    known_file_extensions.insert("mkv", "dirty-green");
    known_file_extensions.insert("mpg", "dirty-green");
    known_file_extensions.insert("mpeg", "dirty-green");
    known_file_extensions.insert("wmv", "dirty-green");
    known_file_extensions.insert("m4a", "dirty-green");

    known_file_extensions.insert("pkg", "dirty-yellow");
    known_file_extensions.insert("apk", "dirty-yellow");
    known_file_extensions.insert("deb", "dirty-yellow");
    known_file_extensions.insert("rar", "dirty-yellow");
    known_file_extensions.insert("zip", "dirty-yellow");
    known_file_extensions.insert("7z", "dirty-yellow");
    known_file_extensions.insert("tar", "dirty-yellow");
    known_file_extensions.insert("gz", "dirty-yellow");
    known_file_extensions.insert("xz", "dirty-yellow");
    known_file_extensions.insert("hz", "dirty-yellow");

    known_file_extensions.insert("doc", "dirty-red");
    known_file_extensions.insert("docx", "dirty-red");
    known_file_extensions.insert("pdf", "dirty-red");
    known_file_extensions.insert("ppt", "dirty-red");
    known_file_extensions.insert("pptx", "dirty-red");
    known_file_extensions.insert("xsl", "dirty-red");
    known_file_extensions.insert("xslx", "dirty-red");
    known_file_extensions.insert("keynote", "dirty-red");

    let mut known_file_extension_endings = HashMap::new();
    known_file_extension_endings.insert("ignore", "gray");
    known_file_extension_endings.insert("_history", "gray");
    known_file_extension_endings.insert("hst", "gray");
    known_file_extension_endings.insert("info", "yellow");
    known_file_extension_endings.insert("config", "yellow");
    known_file_extension_endings.insert("rc", "yellow");
    known_file_extension_endings.insert("sh", "yellow");

    let mut known_file_names = HashMap::new();
    known_file_names.insert("LICENSE", "gray");
    known_file_names.insert("CNAME", "gray");
    known_file_names.insert("__init__.py", "gray");
    known_file_names.insert("package-lock.json", "gray");
    known_file_names.insert("README.md", "red");
    known_file_names.insert("Makefile", "red");
    known_file_names.insert("Dockerfile", "red");
    known_file_names.insert("docker-compose.yml", "red");
    known_file_names.insert(".pre-commit-config.yaml", "red");
    known_file_names.insert(".pre-commit-hooks.yaml", "red");
    known_file_names.insert("webpack.config.js", "red");
    known_file_names.insert("package.json", "red");
    known_file_names.insert(".browserlistrc", "red");
    known_file_names.insert("eslint.config.js", "red");
    known_file_names.insert(".eslintignore", "red");
    known_file_names.insert(".eslintrc.js", "red");
    known_file_names.insert(".babelrc", "red");
    known_file_names.insert(".stylelintrc", "red");
    known_file_names.insert("tsconfig.json", "red");
    known_file_names.insert("tsconfig.eslint.json", "red");
    known_file_names.insert("tsconfig.tests.json", "red");
    known_file_names.insert("requirements.txt", "red");
    known_file_names.insert("requirements-testing.txt", "red");
    known_file_names.insert(".gitattributes", "red");

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
