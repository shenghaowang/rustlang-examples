# Examples from The Rust Programming Language book

The web version of the book is available [here](https://doc.rust-lang.org/book/).

## Content

### 1. Getting Started
- [hello_world](hello_world/hello_world.rs)
- [hello_cargo](hello_cargo/src/main.rs)

### 2. Programming a Guessing Game
- [guessing_game](guessing_game/src/main.rs)

### 3. Common Programming Concepts
- [variables](variables/src/main.rs)

## Add the PATH variable (MacOS)
1. After installing rust, add the following command to .zshrc
```
export PATH="$HOME/.cargo/bin:$PATH"
```

2. Test the installation using `rustc --version`.