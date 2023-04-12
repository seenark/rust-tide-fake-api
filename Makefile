z:
	zellij -l z.dkl


docker-build-gcp:
	docker build -t asia-southeast1-docker.pkg.dev/aquila-studio-web/prolab/ramafake --platform linux/amd64 .

docker-push-gcp:
	docker push asia-southeast1-docker.pkg.dev/aquila-studio-web/prolab/ramafake

