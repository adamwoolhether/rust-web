# Install Clippy
# https://rust-lang.github.io/rust-clippy/master/index.html
#
# rustup component add clippy
# rustup component add rustfmt

# https://github.com/Rust-Web-Development/code
# Cors Req
#
#curl -X OPTIONS localhost:3030/questions \
#	-H "Access-Control-Request-Method: PUT" \
#	-H "Access-Control-Request-Headers: content-type" \
#	-H "Origin: https://not-origin.io" –verbose
#
#	curl http://localhost:3030/questions\?start\=1\&end\=400
#
#   Invalid body:
#curl --location --request POST 'localhost:3030/questions' \
#      --header 'Content-Type: application/json' \
#      --data-raw '{
#      "id": "5",
#      "title": "NEW TITLE"
#}'
#
#curl --location --request PUT 'localhost:3030/questions' \
#        --header 'Content-Type: application/json' \
#        --data-raw '{
#            "id": 1,
#            "title": "NEW TITLE",
#            "content": "OLD CONTENT"
#        }'

docs:
	rustup doc --std

# Generate docs and open in browser.
docs-open:
	cargo doc --open

lint:
	cargo clean
	cargo clippy

# Logging levels in env_logger are determined by RUST_LOG env var.
#run:
	#RUST_LOG=info cargo run
#run-verbose:
	#RUST_LOG=debug cargo run
curl:
	curl localhost:3030/questions
cors:
	curl --verbose -X OPTIONS localhost:3030/questions \
    -H "Access-Control-Request-Method: PUT" \
    -H "Access-Control-Request-Headers: content-type" \
    -H "Origin: https://not-origin.io"