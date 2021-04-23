# Purpose
This is application that is responsible for hosting Arcology game.

# How to run
You can use cargo to run it locally or docker to host it. 
## Host Locally 

Command line using cargo: 
* Build
```
cargo build 
```

* Run
```
cargo run 
```

By default application is using port 8080. 

## Docker
Command line using docker: 
* Build
```
docker build -t arcology-backend .
```
* Run
```
docker stop arcology-backend-container
docker run -p 8080:8080 --rm --name arcology-backend arcology-backend-container
```
