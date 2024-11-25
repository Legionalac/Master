## WSL Setup
You need two terminals: first for running docker and second for running vscode 

Make sure that Docker is running in first terminal:

``` sudo dockerd  ```

In second terminal open

```  code . ``` 

In VSCode on startip in right bottom side click on Reopen in Container or control + shift + P -> Reopen in devcontainer to start a devcontainer. 

Devcontainer will span a docker image and install needed extensions in VSCode.

# USE ONLY TERMINAL IN VSCODE 
It is connected to the docker.
Running scripts on local termanl will not work.


## Scripts:

* build.sh used to build a project
* run.sh used to execute all tests
* clean.sh clean a project
* execute.sh build and run


# SetUp 

## Install docker

```
sudo curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
sudo chmod a+r /etc/apt/keyrings/docker.asc
echo   "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" |   sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt-get install docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
sudo docker run hello-world
```