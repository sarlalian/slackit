


default:
  just -l


build:
  docker buildx build -t slackit .
