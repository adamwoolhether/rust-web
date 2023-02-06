# Cors Req
#
#curl -X OPTIONS localhost:3030/questions \
#	-H "Access-Control-Request-Method: PUT" \
#	-H "Access-Control-Request-Headers: content-type" \
#	-H "Origin: https://not-origin.io" –verbose
#
#	curl http://localhost:3030/questions\?start\=1\&end\=400

docs:
	rustup doc --std

docs-project:
	cargo doc --open

curl:
	curl localhost:3030/questions
cors:
	curl --verbose -X OPTIONS localhost:3030/questions \
    -H "Access-Control-Request-Method: PUT" \
    -H "Access-Control-Request-Headers: content-type" \
    -H "Origin: https://not-origin.io"