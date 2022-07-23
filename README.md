# isekai

Redirect urls into another world. Based on redis. No tracking, no links. A
redis instance is all you need to shorten links.

Try this snippet:

```
$ redis-cli
> set ddg https://duckduckgo.com
> exit
$ cargo run
$ curl localhost:8000/ddg
```

And get redirected to duckduckgo. No control panel. No nonsense. No tracking.

## Deployment

This project is meant to be deployed with docker. Theres a `docker-compose.yml`
that you can use if you want to test it.
