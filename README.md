Survey Website

1. Index
[-] CSS For Sign-up form
[-] Sign-up
[ ] Implement Login
[ ] CSS For login form
[ ] Form Validation
[ ] UserServiceError
[ ] CSS Front Page

2. Home

To run on docker:

```
docker build -t wks/rust-test .
docker run -it --rm -p 8080:8080 -v $(pwd):/source -w /source wks/rust-test cargo run
```