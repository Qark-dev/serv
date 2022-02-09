# Serv
<img src="https://media.discordapp.net/attachments/828947489267777570/940816988526235718/logo.png" alt="serv logo" class="img-center"/>

An ultra lightweight, minimal, and fast web server. (The binary is only ~2mb!!)

## Getting Started

These instructions will cover usage information and for the docker container 

### Prerequisities


In order to run this container you'll need docker installed.

* [Windows](https://docs.docker.com/windows/started)
* [OS X](https://docs.docker.com/mac/started/)
* [Linux](https://docs.docker.com/linux/started/)

### Usage

#### Container Parameters

Create a container to host the server

```shell
docker pull aurax86/serv:latest
docker run --platform linux/amd64 -d -p 8080:80 aurax86/serv:latest 
```

Or host files from a volume
```shell
docker run --platform linux/amd64 -d -p 8080:80 -v "$PWD":/dist aurax86/serv:latest 
```
where `$PWD` is the directory from the host which you wish to deploy.

For 32-bit arm, use `--platform linux/arm/v7` and  `--platform linux/arm/v8` for 64-bit

#### Environment Variables

* `SERV_DIR` - Directory to serve within the container, set to `/dist` by default.
* `IDX_FILE` - Index file from within `SERV_DIR`, defaults to `index.html`.

#### Volumes

* `/dist` - Mount a volume to this directory in the container

## Built With

* The Rust Programming Language
* cargo cross cross-compiler
* docker buildx

## Find Us

* [GitHub](https://github.com/Qark-dev/serv)

## Contributing

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the 
[tags on this repository](https://github.com/your/repository/tags). 

## Authors

* **[Qark-dev](https://github.com/Qark-dev)**

See also the list of [contributors](https://github.com/your/repository/contributors) who 
participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* The Rust community discord server
* The Actix community and the Actix project