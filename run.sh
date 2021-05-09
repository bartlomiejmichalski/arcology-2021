#!/bin/bash
docker build -f Dockerfile.dev -t arcology-api-dev .  
docker stop arcology-api-dev 
docker rm arcology-api-dev 
docker run -it -v ~/arcology:/usr/src/arcology --name arcology-api-dev arcology-api-dev cargo-watch -x run