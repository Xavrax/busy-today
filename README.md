# :warning: Work in progress! :warning:
# Busy-Today

## What is it?

**Busy-Today** is a simple scheduling assistant backend that helps you
meet up with friends by declaring if a day is busy for someone or not. 
It provides few microservices that communicate with each other that can be
deployed at Kubernetes cluster. Users can interact with system using REST API.

## Requirements

All you need to play with this repository:

- [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) - to download repository
- [curl](https://curl.se/download.html) [linux only] - for installing rust toolchain
- [rust toolchain](https://rustup.rs/) - project is written in rust lang
- [docker](https://docs.docker.com/engine/install/) - all services are dockerized
- [kubernetes cluster](https://kubernetes.io/docs/tasks/tools/) - it is main and only supported deployment way

## Building

To build the system you have to compile it with rust toolchain 
(recommended version: `rustc 1.54.0`). Then you can use `./build.sh`
script to build whole system with docker images.

## Deployment

You can deploy everything build in previous step using `deploy.sh` script.
It runs services with their default configuration. 
You can edit `.yaml` files in `k8s/` directory to configure them as you need. 

Additionally, you can deploy only services that you need and swap
the rest as you want. All requirements about API are located in concrete
service directory.