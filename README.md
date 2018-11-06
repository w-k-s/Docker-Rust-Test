# Survey Website

### 1. Login Page

- [x] CSS For Sign-up form
- [x] Sign-up
- [x] Implement Login
- [x] Generate JWT Token
- [x] Assign Login Token to User in DB
- [x] Set Token in cookie
- [x] Verify token
- [x] CSS For login form
- [x] Form Validation in JS
- [x] UserServiceError
- [x] CSS Front Page

### 2. Hosting

- [x] Test on Docker
- [x] Create Makefile to switch between local,docker,publish etc
- [x] Use Command Line Arguments to supply environment variables

### 3. To Do

- [ ] Create new survey
- [ ] List of created surveys: (options: edit, share, results, delete)


### To run on docker:

```
docker build -t wks/rust-test .
docker run -it --rm -p 8080:8080 -v $(pwd):/source -w /source wks/rust-test cargo run
```
