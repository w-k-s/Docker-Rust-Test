To run on docker:

```
docker build -t wks/rust-test .
docker run -it --rm -p 8080:8080 -v $(pwd):/source -w /source wks/rust-test cargo run
```