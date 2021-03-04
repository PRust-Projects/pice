# Cross-compiling from Linux to Windows

## Requirements

- Running Fedora
- cargo (this can be installed via rustup)

## Install all dependencies

```bash
$ ./install_windows_deps.sh
```

## Cross-compile the application

```bash
$ ./build.sh
```

## Package the application

```bash
$ ./package.sh
```

## Distribute the application

- Copy the generated zip file on to the systems where it would be used
- Extract the contents of the zip file
- Run pice.exe within the unzipped folder
