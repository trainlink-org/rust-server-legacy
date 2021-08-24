# TrainLink Rust Server - **Experimental!**
[![Build Rust](https://github.com/trainlink-org/rust-server/actions/workflows/build-rust.yml/badge.svg)](https://github.com/trainlink-org/rust-server/actions/workflows/build-rust.yml)

**Warning:** This is experimental software and should not be used for anything other than tinkering with. Please us the [Python version](https://github.com/trainlink-org/python-server) of the TrainLink server.

This is an API to intergrate with a DCC++ (or DCC++ EX) BaseStation. It provides a simple way to control it over your local network, with multiple instances supported. This means if you open a website using TrainLink on two devices connected to the same server, they will be kept in sync! If you don't know anything about TrainLink, I suggest you check out [the main repo](https://github.com/trainlink-org/trainlink-api), this gives a better overview of the TrainLink system.

**Note:** Before version 0.2.0, the whole codebase was kept in one repository. It has now been split into separate repositories to help with maintainance, but the old releases are kept in the [main repository](https://github.com/trainlink-org/trainlink-api) for now, just for completeness.

## What is in this Repository?
This repository contains the Rust version of the server for TrainLink. Once a stable version is reached, binary builds and installers will be available for easy installation.

## Installation
It's as easy as 1,2,3!

1. Download the source code file (tar.gz or zip) from the latest release (or click [here](https://github.com/trainlink-org/python-server/releases/latest))
1. Check all the dependancies are installed (see above) and copy `config/config.default.xml` to `config/config.xml`
1. Run `server.py`

Now the TrainLink server will be running on your PC! Just point a client at the server and you are ready to go! For more detailed instructions please see the [readthedocs page](https://trainlink-api.readthedocs.io/en/stable/getting-started.html)

## Branches and releases
Releases are numbered according to the [Semantic Numbering](https://semver.org/) scheme. Therefore, releases will be numbered as following:

>Given a version number MAJOR.MINOR.PATCH, increment the:
>
>MAJOR version when you make incompatible API changes,  
MINOR version when you add functionality in a backwards compatible manner, and  
PATCH version when you make backwards compatible bug fixes."

### Branches
Master branch - Where code for the next release accumulates.  
Preview branch - Code that is finished, but not fully tested yet.  
Development-x.x branch - Where I write my code, almost guaranteed to be unstable!  
Any other branches - Same as above.

## Contributing
Want to suggest a feature, found a bug, or even better, fixed a bug? Please, go ahead and submit a pull request or issue! Every little helps, and even the smallest contribution will go a long way to help me with this project. You don't need to know how to code, as correcting typos or updating the documentation would help a lot! For more information on contributing, please see the wiki on the main repository.

## Development Dependancies
Firstly, you will need cargo and the correct rust toolchain installed. More information can be found on the Rust website.

You can find all the dependancies in the `cargo.toml` file. These can all be easily installed using cargo by running the following command:
```
$ cargo build
```
This will also build the TrainLink server.

To run the server, use:
```
$ cargo run
```

## More Information
For more information please see the following:
* [The main repo](https://github.com/trainlink-org/trainlink-api) - Gives an overview of the TrainLink system.
* [The wiki](https://github.com/trainlink-org/trainlink-api/wiki) - FAQ and other repository maintainance help
* [Readthedocs](https://trainlink-api.readthedocs.io) - information on the API itself and the function calls

Many thanks,  
Matt  
\- August 2021
