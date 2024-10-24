# Design Documentation

## Choices

1. Web app programming language:  Python, Rust, etc.
2. Web framework: Flask/FastAPI/Django/etc., Actix Web/Rocket/etc.
3. Load balancer / reverse proxy: ipvs, HAProxy, Nginx, etc.
4. Load balancing algorithm: round robin, least connections, random, etc.

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

## Load Balancer / reverse proxy

We want to choose something that can:

- map the wide range of ports to backend port 80
- support sticky sessions
- be customizable in load balancing algorithms
- pass the original source IP to the webservers

### Load Balancing algorithm

Choices: Round robin, Least connections

### Decision

- HAProxy, with leastconn as the algorithm.  Least connections because we only have two webservers.  If one is down for an extended period of time it'll build up a number of IPs that it is managing.  When the downed webserver comes back up, we want it to take on new IPs until it matches the existing web server's IPs instead of going back to alternating connections like round robin would do.  This won't prevent one IP from creating more load that others, so unless we remove the sticky session requirement this is the best we'll be able to do to even out the load.

The load balancer needs to bind to 5000 ports (60000-65000).  ipvs, while native to linux makes it challenging to do this without listing all ports.  The load balancer needed to account for ranges so HAProxy was useful to solve this problem.  We also needed to pass along HTTP Headers to retain the original src IP, and HAProxy can operate at layer 7.

## Overall Challenges

1. The binary needs to compile for the x86_64-unknown-linux-musl target since this version of Ubuntu uses GLIBC_2.32 - GLIBC_2.34 and the Github action image uses a newer one.
2. The capabilities on the binary need to be able to bind to ports under 1024
