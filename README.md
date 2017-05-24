Survey Website

1. Home page
- Add a sign-up form with sign-up with google and sign-up with facebook
- Add switch between production and staging


To run on docker:

```
docker build -t wks/rust-test .
docker run -it --rm -p 8080:8080 -v $(pwd):/source -w /source wks/rust-test cargo run
```