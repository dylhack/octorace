FROM rustlang/rust:nightly

RUN USER=root cargo new --bin octorace

WORKDIR ./octorace
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/release/deps/octorace*

RUN apt-get update
RUN apt-get install -y nodejs
RUN apt-get install -y npm
RUN npm install npm@latest -g

RUN npm --prefix ./web install
RUN npm run --prefix ./web deploy

EXPOSE 8000

CMD ["cargo", "run", "--release"]

