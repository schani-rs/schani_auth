# Schani Auth

[![Build Status](https://travis-ci.org/schani-rs/schani_auth.svg?branch=master)](https://travis-ci.org/schani-rs/schani_auth)
[![Docker Automated build](https://img.shields.io/docker/automated/schanirs/auth.svg)](https://hub.docker.com/r/schanirs/auth)

## API

### Authorize

Verify a username-password combination and get a new JWT on success. `username` and `password` are passed as query parameters.

``POST /authenticate``

```bash
curl http://127.0.0.1:8000/authenticate?username=a\&password=123456
```

### Verify token

Verify that a token is (still) valid. This returns 200 if the token is valid and 401 if it's not. A HTTP 400 status is returned when the token cannot be parsed.

``POST /verify/<token>``

```bash
curl -v -X POST http://127.0.0.1:8000/verify/eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOiJzY2hhbmktcnMiLCJzdWIiOiJ0ZXN0IiwiYXVkIjpudWxsLCJleHAiOm51bGwsIm5iZiI6bnVsbCwiaWF0IjpudWxsLCJqdGkiOm51bGx9.U40p4ITqOFW9jdurBbHDyGoQxkR0y3unwMiifntWMOc
```
