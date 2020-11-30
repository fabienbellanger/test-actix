# test-actix
Actix-web framework test

## Benchmark
Use [Drill](https://github.com/fcsonline/drill)
```bash
$ drill --benchmark drill.yml --stats --quiet
```

## TODO list
-  [x] [JWT](https://docs.rs/actix-web-httpauth/0.5.0/actix_web_httpauth/)
-  [x] [Auto migrations](https://docs.rs/diesel_migrations/1.4.0/diesel_migrations/)
-  [x] [Askama](https://github.com/djc/askama) [avec Actix-web](https://github.com/djc/askama/tree/main/askama_actix)
-  [ ] Commenter le code
-  [x] [Remplacer failure par derive_more ](https://actix.rs/docs/errors/)
-  [ ] Ajout `Dockerfile` et `docker-compose.yml` [Actix-Web in Docker: How to build small and secure images](https://dev.to/sergeyzenchenko/actix-web-in-docker-how-to-build-small-and-secure-images-2mjd)
    1. `$ docker build -t test-actix .`
    2. `$ docker run -it —rm —name test-actix-instance test-actix`
