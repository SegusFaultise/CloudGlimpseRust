# CloudGlimpse2

# How to run web branch manually
```console
sudo apt update && sudo apt upgrade && sudo apt install pkg-config && sudo apt install librust-alsa-sys-dev && sudo apt install libudev-dev && sudo apt install python3
```

Build wasm for the web
```console
wasm-pack build --release --target web
```

Run serve script
```console
Python3 serve.py
```

# How to run Web Branch With Docker
Install docker
```console
# Add Docker's official GPG key:
sudo apt-get update
sudo apt-get install ca-certificates curl gnupg
sudo install -m 0755 -d /etc/apt/keyrings
curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
sudo chmod a+r /etc/apt/keyrings/docker.gpg

# Add the repository to Apt sources:
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get update
```

```console
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
```

Build docker file
```console
sudo docker build -t cloud_glimpse .
```

Run docker file
```console
sudo docker run -p 8000:8000 -it cloud_glimpse
```

# Features
- [x] pan/orbit camera
- [ ] resizable points
- [ ] point colours based on classification
- [ ] segmentation based on classification (e.g. move/remove buildings, foliage etc...)
- [ ] stream/chunk las file loading
- [ ] laz file format support
