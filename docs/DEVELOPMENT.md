# Developer Docs

## Development Environment

### Prerequisites

- [Docker](https://www.docker.com/)
- [Docker Compose](https://docs.docker.com/compose/)
- [Rust](https://www.rust-lang.org/). Always the latest stable version
- [Pre Commit](https://pre-commit.com/)

## Running the Project

Two scripts are provided to run the project:

- `./run.sh` - This script will build the project and run
- `./build.sh` - This script will build the project

Command line options, for both scripts, are as follows:

- `--release` - Build the project in release mode
- `--debug` - Build the project in debug mode. Default
- `--clean` - Clean the project before building
- `--web` - Build as a web project
- `--standalone` - Build as a standalone project. Not yet implemented

## Providing commits

There is some linting that is done for each Pull Request, however, it isn't as full featured as `pre-commit`. Please either install pre-commit as a hook to this git repo via:

```bash
pre-commit install
```

Which will automatically run all of the pre-commit hooks on each commit. Or you can run the pre-commit hooks manually via:

```bash
pre-commit run --all-files
```

## Speaking of linting

To keep formatting consistent, `cargo fmt` is required to be run before each commit. This can be done via:

```bash
cargo fmt
```

Additionally, `clippy` is also required to be run before each commit. This can be done via:

```bash
cargo clippy
```

Clippy has been set up to be a bit more strict than the default settings. This is to ensure we are writing the best code possible.
