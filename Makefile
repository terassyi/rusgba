
run:
	@wasm-pack build
	@cd app && npm install && npm run start

clean:
	rm -rf pkg && rm -rf target
