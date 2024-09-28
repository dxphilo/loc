// TODO! Consider using enums for better type safety and maintainability.
// If the language is not matched, return the file extension as a fallback.
pub fn get_language_name(extension: &str) -> &str {
    match extension {
        "as" => "ActionScript",
        "ada" => "Ada",
        "agda" => "Agda",
        "at" => "AmbientTalk",
        "asp" => "ASP",
        "aspx" => "ASP.NET",
        "asm" => "Assembly",
        "m4" => "Autoconf",
        "awk" => "Awk",
        "bat" => "Batch",
        "sh" => "Bourne Shell",
        "c" => "C",
        "h" => "C/C++ Header",
        "cmake" => "CMake",
        "cs" => "C#",
        "clj" => "Clojure",
        "cljs" => "ClojureScript",
        "cljc" => "ClojureC",
        "coffee" => "CoffeeScript",
        "cfm" => "ColdFusion",
        "cfc" => "ColdFusionScript",
        "v" => "Coq",
        "cpp" => "C++",
        "cr" => "Crystal",
        "css" => "CSS",
        "cu" => "CUDA",
        "hcu" => "CUDA Header",
        "d" => "D",
        "dart" => "Dart",
        "dhall" => "Dhall",
        "dockerfile" => "Docker",
        "ex" => "Elixir",
        "elm" => "Elm",
        "erl" => "Erlang",
        "fs" => "F#",
        "gh" => "Gherkin",
        "glsl" => "GLSL",
        "go" => "Go",
        "groovy" => "Groovy",
        "handlebars" => "Handlebars",
        "hs" => "Haskell",
        "hex" => "Hex",
        "html" => "HTML",
        "ini" => "INI",
        "idr" => "Idris",
        "thy" => "Isabelle",
        "jai" => "Jai",
        "java" => "Java",
        "js" => "JavaScript",
        "json" => "JSON",
        "jsx" => "JSX",
        "jl" => "Julia",
        "kt" => "Kotlin",
        "less" => "Less",
        "linker" => "LinkerScript",
        "lean" => "Lean",
        "lisp" => "Lisp",
        "lua" => "Lua",
        "make" => "Make",
        "mk" => "Makefile",
        "md" => "Markdown",
        "mustache" => "Mustache",
        "nim" => "Nim",
        "nix" => "Nix",
        "ml" => "OCaml",
        "m" => "Objective-C",
        "mm" => "Objective-C++",
        "cl" => "OpenCL",
        "oz" => "Oz",
        "pas" => "Pascal",
        "pl" => "Perl",
        "php" => "PHP",
        "poly" => "Polly",
        "ps1" => "PowerShell",
        "plg" => "Prolog",
        "proto" => "Protobuf",
        "pp" => "Puppet",
        "purs" => "PureScript",
        "py" => "Python",
        "qcl" => "QCL",
        "qml" => "QML",
        "lock" => "Lock",
        "r" => "R",
        "razor" => "Razor",
        "re" => "Reason",
        "ron" => "RON",
        "rst" => "reStructuredText",
        "rb" => "Ruby",
        "erb" => "Ruby HTML",
        "rs" => "Rust",
        "sls" => "SaltStack",
        "sass" => "Sass",
        "scala" => "Scala",
        "sml" => "SML",
        "sol" => "Solidity",
        "sql" => "SQL",
        "styl" => "Stylus",
        "svelte" => "Svelte",
        "swift" => "Swift",
        "tcl" => "Tcl",
        "tf" => "Terraform",
        "tex" => "TeX",
        "txt" => "Plain Text",
        "toml" => "TOML",
        "ts" => "TypeScript",
        "tsx" => "TypeScript JSX",
        "u" => "UnrealScript",
        "vim" => "VimScript",
        "vue" => "Vue",
        "wlf" => "Wolfram",
        "xml" => "XML",
        "yacc" => "Yacc",
        "yaml" => "YAML",
        "zig" => "Zig",
        "zsh" => "Z Shell",
        "hx" => "Haxe",
        _ => "unrecognized",
    }
}
