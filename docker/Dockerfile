FROM rust:latest

RUN apt update -y && \
	apt install -y nodejs npm && \
	cargo install wasm-pack && \
	npm install npm@latest -g

# RUN npm init wasm-app /home/rusgba/app
EXPOSE 8080
WORKDIR /home/rusgba

CMD ["/bin/bash"]
