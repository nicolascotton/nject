# Actix
## Run
To run the Web API, use the following command:
```sh
cargo run
```
## API
### Create user
```sh
curl -v -H 'Content-Type: application/json' -d '{ "name": "user1" }' 127.0.0.1:8080/api/users/
```
### Get user
```sh
curl -v 127.0.0.1:8080/api/users/{user_id}
```
### Update user
```sh
curl -v -H 'Content-Type: application/json' -d '{ "id": 1, "name": "newuser1" }' -X PUT 127.0.0.1:8080/api/users/
```
### Delete user
```sh
curl -v -X DELETE 127.0.0.1:8080/api/users/{user_id}
```