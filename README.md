# packer_rs

A simple Rust wrapper around HashiCorp Packer CLI. This lets you run Packer commands from your Rust code without dealing with raw command line stuff.

## Basic Usage

First, make sure you have the Packer CLI installed and available in your project directory (as `packer` on Linux/Mac or `packer.exe` on Windows).

Add this to your `Cargo.toml`:
```toml
[dependencies]
packer_rs = "0.1"
```

Here's a quick example:

```rust
use packer_rs::{Packer, BuildOptionsBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new Packer instance
    let packer = Packer::new()?;
    
    // Set up some build options
    let options = BuildOptionsBuilder::default()
        .debug(true)
        .vars(vec![("region", "us-west-2")])
        .build()?;
    
    // Build your template
    packer.build("template.pkr.hcl", &options)?;
    
    Ok(())
}
```

## What You Can Do

The wrapper supports the main Packer commands:

- `build`: Build images from a template
- `init`: Set up a new template
- `validate`: Check if a template is valid
- `inspect`: Look at template details
- `fix`: Fix old templates
- `console`: Start Packer console
- `plugin`: Manage Packer plugins

## Build Options

When building templates, you can set various options using `BuildOptionsBuilder`:

```rust
let options = BuildOptionsBuilder::default()
    .debug(true)                // Enable debug mode
    .force(true)               // Force builds
    .parallel_builds(2)        // Run 2 builds at once
    .timestamp_ui(true)        // Show timestamps
    .vars(vec![                // Set variables
        ("region", "us-west-2"),
        ("instance_type", "t2.micro")
    ])
    .var_files(vec!["vars.json".into()])  // Load vars from files
    .build()?;
```

## Working Directory

You can set a different working directory for commands:

```rust
let packer = Packer::new()?
    .with_working_dir("./my-templates");
```

## Error Handling

The wrapper returns proper Rust errors that tell you what went wrong. Main error types:

- `NotFound`: Can't find the Packer executable
- `ExecutionError`: Command failed to run
- `ConfigError`: Something wrong with the configuration
- `IoError`: File system problems

## Contributing

Feel free to open issues or send pull requests if you find bugs or want to add features.

## License

MIT licensed. Do what you want with it.