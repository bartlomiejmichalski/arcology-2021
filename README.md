# Purpose
This is an application that is responsible for hosting the Arcology game.

# How to run
You can use cargo to run it locally or docker to host it. 
## Host Locally 

The application requires Redis as an experiment store to run. You can run the Redis container using Docker or docker-compose. 

```
docker-compose up Redis
```

Then you can develop an application using cargo or running cargo inside another container. 

## Local development on your host machine

You can run application using cargo. 

```
cargo run
```
Using this way any kind of changes made in code won't affect your application. You will have to stop the application and run it again. 

The better solution would be using a cargo watch. You will have to install it first using:

```
cargo install cargo-watch
```

After this step you can simply run the application :
```
cargo watch -x run
```

## Local development using docker

You run the whole application using: 

```
docker-compose up
```

And continue development using for example vs code. The important thing is that you will have to mount your repository folder to the container. 
