Survey Website

1. Index
[-] CSS For Sign-up form
[-] Sign-up
[-] Implement Login
[-] Generate JWT Token
[-] Assign Login Token to User in DB
[-] Set Token in cookie
[-] Verify token
[-] CSS For login form
[ ] Form Validation in JS
[ ] UserServiceError
[-] CSS Front Page

2. Test on Docker
3. Use Command Line Arguments to supply environment variables
4. Create Makefile to switch between local,docker,publish etc


To run on docker:

```
docker build -t wks/rust-test .
docker run -it --rm -p 8080:8080 -v $(pwd):/source -w /source wks/rust-test cargo run
```