# Design Documentation

## Choices

1. Web app programming language:  Python, Rust, etc.
2. Web framework: Flask/FastAPI/Django/etc., Actix Web/Rocket/etc.

## Web App

There are a number of pros and cons for language choice in a web application.

### Python
```
+ Battle-tested ecosystem of libraries and frameworks 
- Packaging inconsistent across environments
```

Framework recommendation: Flask

### Rust
```
+ Performance
+ No garbage collector
+ Simple deployment with single binary
- Steeper learning curve for developer
- Build times
```

Framework recommendation: Actix Web

### Decision

- Programming language: Rust
- Web Framework: Actix Web

Rust was chosen for the simplicity in creating an artifact that can be run on the remote system without the need for external virtual environments and making sure the correct virtual environment is activated.

## Challenges

1. The binary needs to compile for the x86_64-unknown-linux-musl target since this version of Ubuntu uses GLIBC_2.32 - GLIBC_2.34 and the Github action image uses a newer one.
2. The capabilities on the binary need to be able to bind to ports under 1024