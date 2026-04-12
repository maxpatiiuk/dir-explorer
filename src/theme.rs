use std::collections::HashMap;

pub const BLACK_HOLE_DIRECTORY_NAMES: &[&str] = &[
    ".git",
    "node_modules",
    "venv",
    ".venv",
    "target",
    "dist",
    "build",
    "__pycache__",
    ".mypy_cache",
    ".pytest_cache",
    ".next",
    ".nuxt",
    ".cache",
];

pub fn is_black_hole_dir_name(name: &str) -> bool {
    BLACK_HOLE_DIRECTORY_NAMES.contains(&name)
}

pub struct Theme {
    pub color_definitions: HashMap<&'static str, &'static str>,
    pub known_file_extensions: HashMap<&'static str, &'static str>,
    pub known_file_extension_endings: HashMap<&'static str, &'static str>,
    pub known_file_names: HashMap<&'static str, &'static str>,
    pub known_directory_names: HashMap<&'static str, &'static str>,
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
    color_definitions.insert("dirty-pink", "132");
    color_definitions.insert("pink", "125");
    color_definitions.insert("accent", "81");

    let mut known_file_extensions = HashMap::new();
    // ignored / auto-generated
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
    known_file_extensions.insert("mjs", "dirty-yellow");
    known_file_extensions.insert("cjs", "dirty-yellow");
    known_file_extensions.insert("mts", "dirty-yellow");
    known_file_extensions.insert("cts", "dirty-yellow");
    known_file_extensions.insert("eslintcache", "gray");
    known_file_extensions.insert("cache", "gray");
    known_file_extensions.insert("gitkeep", "gray");
    known_file_extensions.insert("broken", "gray");
    known_file_extensions.insert("disabled", "gray");
    known_file_extensions.insert("snap", "gray");

    // important files
    known_file_extensions.insert("pub", "red");
    known_file_extensions.insert("private", "red");
    known_file_extensions.insert("key", "red");
    known_file_extensions.insert("lock", "red");

    // config
    known_file_extensions.insert("env", "purple");
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
    known_file_extensions.insert("lfsconfig", "purple");
    known_file_extensions.insert("prettierignore", "purple");
    known_file_extensions.insert("t9nignore", "purple");

    // c-like
    known_file_extensions.insert("asm", "yellow");
    known_file_extensions.insert("c", "yellow");
    known_file_extensions.insert("class", "yellow");
    known_file_extensions.insert("cpp", "yellow");
    known_file_extensions.insert("cs", "yellow");
    known_file_extensions.insert("h", "yellow");
    known_file_extensions.insert("hpp", "yellow");
    known_file_extensions.insert("php", "yellow");
    known_file_extensions.insert("jar", "yellow");
    known_file_extensions.insert("java", "yellow");

    // windows
    known_file_extensions.insert("bat", "yellow");
    known_file_extensions.insert("exe", "yellow");
    known_file_extensions.insert("bin", "yellow");
    known_file_extensions.insert("wsf", "yellow");
    known_file_extensions.insert("msi", "yellow");
    known_file_extensions.insert("dll", "yellow");
    known_file_extensions.insert("a", "yellow");
    known_file_extensions.insert("o", "yellow");
    known_file_extensions.insert("so", "yellow");
    known_file_extensions.insert("lib", "yellow");
    known_file_extensions.insert("pyd", "yellow");
    known_file_extensions.insert("pyo", "yellow");
    known_file_extensions.insert("pdb", "yellow");
    known_file_extensions.insert("suo", "yellow");
    known_file_extensions.insert("swc", "yellow");
    known_file_extensions.insert("jpi", "yellow");

    // source text
    known_file_extensions.insert("astro", "200"); // same as md/mdx
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
    known_file_extensions.insert("proto", "blue");
    known_file_extensions.insert("service", "blue");
    known_file_extensions.insert("sample", "200"); // same as md
    known_file_extensions.insert("geojson", "blue");
    known_file_extensions.insert("kml", "blue");
    known_file_extensions.insert("db", "blue");
    known_file_extensions.insert("sqlite", "blue");
    known_file_extensions.insert("parquet", "blue");

    // very common files
    known_file_extensions.insert("css", "123"); // light blue
    known_file_extensions.insert("scss", "123");
    known_file_extensions.insert("htm", "green");
    known_file_extensions.insert("html", "green");
    known_file_extensions.insert("js", "208"); // orange
    known_file_extensions.insert("jsx", "dirty-yellow");
    known_file_extensions.insert("md", "200"); // magenta/purple
    known_file_extensions.insert("mdx", "200"); // magenta/purple
    known_file_extensions.insert("py", "119"); // pale green
    known_file_extensions.insert("ts", "153"); // sky blue
    known_file_extensions.insert("tsx", "81"); // dirty blue
    known_file_extensions.insert("sql", "94"); // dirty yellow
    known_file_extensions.insert("vue", "94"); // dirty yellow
    known_file_extensions.insert("rs", "pink");
    known_file_extensions.insert("go", "81"); // dirty blue

    // images
    // dirty color for less preferred formats
    known_file_extensions.insert("jpg", "dirty-pink");
    known_file_extensions.insert("jpeg", "dirty-pink");
    known_file_extensions.insert("svg", "pink");
    known_file_extensions.insert("png", "dirty-pink");
    known_file_extensions.insert("gif", "dirty-pink");
    known_file_extensions.insert("bmp", "dirty-pink");
    known_file_extensions.insert("webp", "dirty-pink");
    known_file_extensions.insert("tif", "dirty-pink");
    known_file_extensions.insert("tiff", "dirty-pink");
    known_file_extensions.insert("psd", "dirty-pink");
    known_file_extensions.insert("ai", "dirty-pink");
    known_file_extensions.insert("ico", "dirty-pink");
    known_file_extensions.insert("heic", "dirty-pink");
    known_file_extensions.insert("heif", "dirty-pink");
    known_file_extensions.insert("apng", "dirty-pink");
    known_file_extensions.insert("avif", "pink");
    known_file_extensions.insert("jxl", "dirty-pink");
    known_file_extensions.insert("jp2", "dirty-pink");
    known_file_extensions.insert("j2k", "dirty-pink");
    known_file_extensions.insert("j2c", "dirty-pink");

    // executables
    known_file_extensions.insert("dmg", "yellow");
    known_file_extensions.insert("iso", "yellow");

    // media
    // dirty color for less preferred formats
    known_file_extensions.insert("webm", "green");
    known_file_extensions.insert("ogg", "dirty-green");
    known_file_extensions.insert("flac", "green");
    known_file_extensions.insert("aac", "dirty-green");
    known_file_extensions.insert("aif", "dirty-green");
    known_file_extensions.insert("mp3", "dirty-green");
    known_file_extensions.insert("wav", "dirty-green");
    known_file_extensions.insert("mp4", "green");
    known_file_extensions.insert("avi", "dirty-green");
    known_file_extensions.insert("mov", "dirty-green");
    known_file_extensions.insert("otf", "dirty-green");
    known_file_extensions.insert("ttf", "dirty-green");
    known_file_extensions.insert("mkv", "green");
    known_file_extensions.insert("mpg", "dirty-green");
    known_file_extensions.insert("mpeg", "dirty-green");
    known_file_extensions.insert("wmv", "dirty-green");
    known_file_extensions.insert("m4a", "dirty-green");
    known_file_extensions.insert("3gp", "dirty-green");
    known_file_extensions.insert("flv", "dirty-green");
    known_file_extensions.insert("m4v", "dirty-green");
    known_file_extensions.insert("ogv", "dirty-green");
    known_file_extensions.insert("woff", "dirty-green");
    known_file_extensions.insert("woff2", "green");

    // archives
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
    known_file_extensions.insert("tgz", "dirty-yellow");

    // additional lfs-heavy 3d/geospatial/binary asset formats
    known_file_extensions.insert("dae", "dirty-yellow");
    known_file_extensions.insert("fbx", "dirty-yellow");
    known_file_extensions.insert("glb", "dirty-yellow");
    known_file_extensions.insert("gltf", "dirty-yellow");
    known_file_extensions.insert("ifc", "dirty-yellow");
    known_file_extensions.insert("obj", "dirty-yellow");
    known_file_extensions.insert("lif", "dirty-yellow");
    known_file_extensions.insert("lpu", "dirty-yellow");
    known_file_extensions.insert("lyr", "dirty-yellow");
    known_file_extensions.insert("pbf", "dirty-yellow");
    known_file_extensions.insert("ptx", "dirty-yellow");
    known_file_extensions.insert("rpk", "dirty-yellow");
    known_file_extensions.insert("sdf", "dirty-yellow");
    known_file_extensions.insert("tbx", "dirty-yellow");
    known_file_extensions.insert("raw", "dirty-yellow");

    // documents
    known_file_extensions.insert("doc", "dirty-red");
    known_file_extensions.insert("docx", "dirty-red");
    known_file_extensions.insert("pdf", "dirty-red");
    known_file_extensions.insert("ppt", "dirty-red");
    known_file_extensions.insert("pptx", "dirty-red");
    known_file_extensions.insert("xsl", "dirty-red");
    known_file_extensions.insert("xslx", "dirty-red");
    known_file_extensions.insert("keynote", "dirty-red");
    known_file_extensions.insert("xlsx", "dirty-red");
    known_file_extensions.insert("drawio", "dirty-red");
    known_file_extensions.insert("chm", "dirty-red");
    known_file_extensions.insert("mdf", "dirty-red");

    let mut known_file_extension_endings = HashMap::new();
    known_file_extension_endings.insert("ignore", "gray");
    known_file_extension_endings.insert("_history", "gray");
    known_file_extension_endings.insert("hst", "gray");
    known_file_extension_endings.insert("info", "yellow");
    known_file_extension_endings.insert("config", "yellow");
    known_file_extension_endings.insert("rc", "yellow");
    known_file_extension_endings.insert("sh", "yellow");

    let mut known_file_names = HashMap::new();
    // ignored. autogenerated
    known_file_names.insert("LICENSE", "gray");
    known_file_names.insert("CNAME", "gray");
    known_file_names.insert("__init__.py", "gray");
    known_file_names.insert("package-lock.json", "gray");
    known_file_names.insert("pnpm-lock.yaml", "gray");
    known_file_names.insert("yarn.lock", "gray");

    // important files
    known_file_names.insert("CODEOWNERS", "red");
    known_file_names.insert("README.md", "red");
    known_file_names.insert("Dockerfile", "red");
    known_file_names.insert(".Dockerfile", "red");
    known_file_names.insert("Makefile", "red");
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
    known_file_names.insert(".gitattributes", "red");
    known_file_names.insert("commit-msg", "red");
    known_file_names.insert("post-checkout", "red");
    known_file_names.insert("post-commit", "red");
    known_file_names.insert("post-merge", "red");
    known_file_names.insert("pre-commit", "red");
    known_file_names.insert("pre-push", "red");

    let mut known_directory_names = HashMap::new();
    // unimportant / noisy
    for name in BLACK_HOLE_DIRECTORY_NAMES {
        known_directory_names.insert(*name, "gray");
    }

    // important
    known_directory_names.insert("src", "accent");
    known_directory_names.insert("scripts", "accent");
    known_directory_names.insert("packages", "accent");

    Theme {
        color_definitions,
        known_file_extensions,
        known_file_extension_endings,
        known_file_names,
        known_directory_names,
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
