# Schani Auth

## API

### Authorize

Verify a username-password combination and get a new JWT on success. `username` and `password` are passed as POST parameters.

``POST /authenticate``

```bash
curl localhost:8105/authenticate -d username=test -d password=123456
```

### Verify token

Verify that a token is (still) valid. This returns 200 if the token is valid and 401 if it's not. A HTTP 400 status is returned when the token cannot be parsed.

``POST /verify/<token>``

```bash
curl -v -X POST localhost:8105/verify/eyJ0eXAiOiJKV1QiLCJraWQiOm51bGwsImFsZyI6IkhTMjU2In0.eyJpc3MiOiJzY2hhbmktcnMiLCJzdWIiOiJ0ZXN0IiwiYXVkIjpudWxsLCJleHAiOm51bGwsIm5iZiI6bnVsbCwiaWF0IjpudWxsLCJqdGkiOm51bGx9.U40p4ITqOFW9jdurBbHDyGoQxkR0y3unwMiifntWMOc
```
