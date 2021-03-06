# :warning: Work in progress! :warning:
# Busy-Today

![Rust](https://github.com/Xavrax/busy-today/actions/workflows/rust.yml/badge.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## What is it?

**Busy-Today** is a simple scheduling assistant backend that helps you
meet up with friends by declaring if a day is busy for someone or not. 
It provides few microservices that communicate with each other that can be
deployed at Kubernetes cluster. Users can interact with system using REST API.

## Quick overview

Architecture of system is presented at diagram below:

![busy today overview](busy-today-overview.png)

Simple services implemented in this project:
 - Gate - simple gateway for system
 - Authenticator - **todo: security**
 - Database Injector - service that writes into database
 - Database Reader - service that reads from database

Other services or systems:
 - Event Streaming Platform - Apache Kafka, used for handling events from gate
 - Database - Default filesystem with files

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

It is possible deploy everything built in previous step using `deploy.sh` script.
It runs services with their default configuration. 
You can edit `.yaml` files in `k8s/` directory to configure them as you need. 

Additionally, you can deploy only services that you need and swap
the rest as you want. All requirements about API are located in concrete
service directory.

## Communication

To communicate with the system it is necessary to use it's [open API](crates/gate/open_api.yaml) on [gate service](crates/gate).