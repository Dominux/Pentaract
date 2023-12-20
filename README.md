![pentaract-github-logo](https://github.com/Dominux/Pentaract/assets/55978340/db39e76f-4119-41c1-bbfd-9b59f40ab626)

[<img alt="GitHub Workflow Status (with event)" src="https://img.shields.io/github/actions/workflow/status/Dominux/Pentaract/docker-image.yml?style=plastic&logo=github">](https://github.com/Dominux/Pentaract/actions)
[<img alt="Dockerhub latest" src="https://img.shields.io/badge/dockerhub-latest-blue?logo=docker&style=plastic">](https://hub.docker.com/r/thedominux/pentaract)
[<img alt="Docker Image Size (tag)" src="https://img.shields.io/docker/image-size/thedominux/pentaract/latest?style=plastic&logo=docker&color=gold">](https://hub.docker.com/r/thedominux/pentaract/tags?page=1&name=latest)
[<img alt="Any platform" src="https://img.shields.io/badge/platform-any-green?style=plastic&logo=linux&logoColor=white">](https://github.com/Dominux/Pentaract)

_Cloud storage system based on using Telegram as a storage so it doesn't use your server filesystem or any other paid cloud storage system underneath the hood._

Pentaract is aimed to take as small disk space as possible. So it does not need any code interpreter/platform to run. The whole app is just several megabytes in size. It also uses Postgres as a database and we try our best to economy space by not creating unneeded fields and tables and to wisely pick proper datatypes.

# Installation

This project is aimed on running the app in container, so the primary way to run it is via [Docker](https://www.docker.com/). If you don't have it installed or simply don't want to run the app via Docker, you can build it from source.

> NOTE: Pentaract uses [Postgres](https://www.postgresql.org/) as a database. So if you are going to run it from source or run the Pentaract image only, you will need to have a Postgres instance running and available in your network so you will connect your Pentaract app to it

<details>
  <summary>Docker Compose with pre-built image <i>(recommended)</i></summary>

The simplest way to run and manage the app

1. Create new directory for the app files and name it however you wish:

```sh
mkdir pentaract
```

2. Go to it and place `docker-compose.yml` file like this one:

```yaml
version: "3.9"

volumes:
  pentaract-db-volume:
    name: pentaract-db-volume

services:
  pentaract:
    container_name: pentaract
    image: thedominux/pentaract
    env_file:
      - .env
    ports:
      - ${PORT}:8000
    restart: unless-stopped
    depends_on:
      - db

  db:
    container_name: pentaract_db
    image: postgres:15.0-alpine
    environment:
      POSTGRES_USER: ${DATABASE_USER}
      POSTGRES_PASSWORD: ${DATABASE_PASSWORD}
    restart: unless-stopped
    volumes:
      - pentaract-db-volume:/var/lib/postgresql/data
```

And `.env` file like the next one. **Don't forget to set your superuser email, password and secret key**:

```env
PORT=8000
WORKERS=4
CHANNEL_CAPACITY=32
SUPERUSER_EMAIL=<YOUR-EMAIL>
SUPERUSER_PASS=<YOUR-PASSWORD>
ACCESS_TOKEN_EXPIRE_IN_SECS=1800
REFRESH_TOKEN_EXPIRE_IN_DAYS=14
SECRET_KEY=<YOUR-SECRET-KEY>
TELEGRAM_API_BASE_URL=https://api.telegram.org

DATABASE_USER=pentaract
DATABASE_PASSWORD=pentaract
DATABASE_NAME=pentaract
DATABASE_HOST=db
DATABASE_PORT=5432
```

Secret key can be set by your hand, but I strongly recommend to use long randomly generated sequences. So you either can generate it via some free websites that provide such funcionallity or by running something like this in the terminal:

```sh
openssl rand -hex 32
```

3. For now everything is set up so we can run our app:

```sh
docker compose up -d
```

To check if everything works fine you can go to http://localhost:8000 or to `http://<YOUR-PUBLIC-IP>:8000` if you run it on a server.

If there are troubles, you can check the logs, there may be some errors:

```sh
docker logs -f pentaract
```

</details>

<details>
  <summary>Docker Compose from source</summary>

Kind of simple way, but it's aimed to use it during development process

1. Clone the repository and go inside the newly created directory:

```sh
git clone git@github.com:Dominux/Pentaract.git
```

2. Copy `.env.example` to `.env`:

```sh
cp ./.env.example ./.env
```

and edit it like you wish.

3. For now everything is set up so we can run our app:

```sh
make up
```

To check if everything works fine you can go to http://localhost:8000 or to `http://<YOUR-PUBLIC-IP>:8000` if you run it on a server.

If there are troubles, you can check the logs, there may be some errors:

```sh
docker logs -f pentaract
```

</details>

<details>
  <summary>Docker with pre-built image</summary>

**TODO**

</details>

<details>
  <summary>From source</summary>

The most complex way to run the app.

Requires the next stuff to be installed:

- [Cargo](https://github.com/rust-lang/cargo)
- [Node.js](https://nodejs.org/en)
- [pnpm](https://pnpm.io/)
- [Postgres](https://www.postgresql.org/)

1. Create a directory to place all the app files wherever in your system:

```sh
mkdir ~/pentaract
```

2. Clone the repository and go inside the newly created directory:

```sh
git clone git@github.com:Dominux/Pentaract.git
```

3. Go to the `./pentaract` directory and build server side app:

```sh
cd ./pentaract
cargo build --release
```

and copy the target to the app directory (or create a soft link via `ln -s`, does not matter):

```sh
cp ./target/release/pentaract ~/pentaract/pentaract
```

4. Go to the `../ui` and build the UI side of the app:

```sh
cd ../ui
pnpm run build
```

and copy built files into the app directory:

```sh
cp ./dist/* ~/pentaract/ui/
```

5. Now go to the app directory:

```sh
cd ~/pentaract
```

6. Make sure that you have Postgres database ran in your system (or available from network)
7. Set all needed environment variables. You can check them in the [.env.example file](https://github.com/Dominux/Pentaract/blob/main/.env.example). **Don't forget to set right Postgres credentials, host and port**:

```sh
export PORT=8000
export WORKERS=4
# ...
```

8. Finally run the app:

```sh
./pentaract
```

To check if everything works fine you can go to http://localhost:8000 or to `http://<YOUR-PUBLIC-IP>:8000` if you run it on a server.

</details>

# Usage
