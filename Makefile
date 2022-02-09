all: armv7 armv8 amd64

armv8:
	cross build --release --target=aarch64-unknown-linux-musl
	docker build -t aurax86/serv:arm64v8 --build-arg ARCH=arm64v8/ -o type=docker  --platform linux/arm/v8 -f Dockerfile.armv8 .
	docker push aurax86/serv:arm64v8

armv7:
	cross build --release --target=armv7-unknown-linux-musleabihf
	docker build -t aurax86/serv:arm32v7 --build-arg ARCH=arm32v7/ -o type=docker --platform linux/arm/v7  -f Dockerfile.armv7 .
	docker push aurax86/serv:arm32v7

amd64:
	cross build --release --target=x86_64-unknown-linux-musl
	docker build -t aurax86/serv:amd64 --build-arg ARCH=amd64/ -o type=docker --platform linux/amd64  -f Dockerfile.amd64 .
	docker push aurax86/serv:amd64

manifest:
	docker manifest create aurax86/serv:latest --amend aurax86/serv:arm64v8 --amend aurax86/serv:arm32v7 --amend aurax86/serv:amd64
	docker manifest push aurax86/serv:latest