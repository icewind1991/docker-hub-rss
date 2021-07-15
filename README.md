# Docker Hub RSS

RSS feed for Docker Hub images

# Usage

#### Start the server

```
PORT=3000 docker-hub-rss
```

#### Get the feed for an image

```
curl localhost:3000/<docker-hub-user>/<docker-hub-repo>
```