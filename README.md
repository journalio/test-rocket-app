# Test application for IPSENH

![Rust](https://github.com/journalio/test-rocket-app/workflows/Rust/badge.svg?branch=develop)
![Web](https://github.com/journalio/test-rocket-app/workflows/Web/badge.svg?branch=develop)
![Publish Api Docker Image](https://github.com/journalio/test-rocket-app/workflows/Publish%20Api%20Docker%20Image/badge.svg?branch=develop)
[![Dependabot Status](https://api.dependabot.com/badges/status?host=github&repo=journalio/test-rocket-app)](https://dependabot.com)
[![Bors enabled](https://bors.tech/images/badge_small.svg)](https://app.bors.tech/repositories/23668)

This is a repository containing test files and setups for the IPSENH school project.

This is not production code and should only be used for testing things. (Going very well so far)

## Getting started

First, you need to clone the repository

```shell script
git clone git@github.com:journalio/test-rocket-app.git
```

Then, inside the project, you can bring in the latest images deployed on
[Github Packages](https://github.com/journalio/test-rocket-app/packages)

```shell script
docker-compose pull
```

If you're running for the first time, you should first start the database container
and run the migrations to initialize the database

```shell script
docker-compose up -d postgres

diesel migration run
```

If you've already initialized the database or just did, you can start the environment easily with

```shell script
docker-compose up -d
```

This will expose the traefik container on ports 80 and 8080,
and you can visit the application on [app.test](http://app.test) and the api on [app.test/api/](http://app.test/api/)
The database is also exposed on port 5432 (default postgres port)

### Building the images manually

You can force the images to build locally using

```shell script
docker-compose up -d --build
```
