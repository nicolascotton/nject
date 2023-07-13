# axum
## Run
To run the Web API, use the following command:
```sh
cargo run
```
## API
### Create user
```sh
curl -v -H 'Content-Type: application/json' -d '{ "name": "user1" }' 127.0.0.1:3000/api/users
```
### Get user
```sh
curl -v 127.0.0.1:3000/api/users/{user_id}
```