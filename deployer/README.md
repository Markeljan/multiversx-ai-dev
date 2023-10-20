# mx-reproducible-contract-build-example-sc

[![Open in Dev Containers](https://img.shields.io/static/v1?label=Dev%20Containers&message=Open&color=blue&logo=visualstudiocode)](https://vscode.dev/redirect?url=vscode://ms-vscode-remote.remote-containers/cloneInVolume?url=https://github.com/multiversx/mx-reproducible-contract-build-example-sc)

Example of a Smart Contract whose build pipeline supports reproducible builds.

See [docs.multiversx.com](https://docs.multiversx.com/developers/reproducible-contract-builds).

## Docker

docker pull --platform linux/x86_64 multiversx/sdk-rust-contract-builder:v5.3.0

python3 ./build_with_docker.py --image multiversx/sdk-rust-contract-builder:v5.3.0 --project ./contract