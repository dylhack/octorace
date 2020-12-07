FROM rustlang/rust:nightly

RUN USER=root cargo new --bin octorace

WORKDIR ./octorace
COPY ./Cargo.toml ./Cargo.toml
RUN rm src/*.rs

ADD . ./

RUN apt-get update
RUN apt-get install -y nodejs
RUN apt-get install -y npm
RUN npm install npm@latest -g

RUN npm --prefix ./web install
RUN npm run --prefix ./web deploy

ENV DATABASE_URL=postgres://octorace:password@database:5432/octorace

EXPOSE 8000

CMD ["cargo", "run", "--release"]

