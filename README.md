Kite - prefer.social 



## Build WASM files 
```
spin build 
```

## Create a docker image 
``` 
docker build -t ghcr.io/prefer-social/kite:$(date +%s) -t ghcr.io/prefer-social/kite:latest -f ./app.Containerfile .
```




