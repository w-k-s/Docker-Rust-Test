local:
	clear;clear;clear; cargo run

docker:
	docker build -t wks/rust-test .
	docker run -it --rm -p 8080:8080 -v $(pwd)/source -w /source wks/rust-test cargo run
	open http://localhost:8080