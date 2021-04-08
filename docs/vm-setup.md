# WASM Playground

## Create New VM
1. Download the lastest version of Ubuntu Desktop
1. Using choice of virtualization tool (I'm using VMWare Workstation 16.x on Windows 10), create a new virtual machine
  1. 4-8GB of RAM, 2 processors, 2 cores per processor
  1. Use defaults unless specified otherwise:
  1. Updates and other software:
    1. Minimal Installation
    1. Download updates while installing Ubuntu
    1. Install third-party software for graphics Wi-Fi hardware and additional media formats
1. Using the command line install the following:
    1. `sudo apt update`
    1. `sudo apt upgrade`
    1. `sudo apt install -y open-vm-tools-desktop`
    1. `sudo add-apt-repository ppa:git-core/ppa`
    1. `sudo apt update`
    1. `sudo apt install -y terminator git curl python3-pip`
    1. `sudo mkidr /data`
    1. `sudo chown -R mshea:mshea /data`
    1. `sudo snap install code --classic`
    1. `sudo snap install clion --classic`
    1. `sudo snap install pycharm-professional --classic`
    1. `sudo snap install postman --classic`
1. Core Dev
    1. `sudo apt install -y build-essential libssl-dev`

1. Install NodeJs
    1. `curl -sL https://deb.nodesource.com/setup_14.x -o nodesource_setup.sh`
    1. `bash nodesource_setup.sh`
    1. `sudo apt install -y nodejs`

1. [Set up Rust/WASM](https://www.rust-lang.org/tools/install)
    1. `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
    1. `source $HOME/.cargo/env`
    1. `cargo install wasm-pack`

1. [Set up NPN](https://docs.npmjs.com/resolving-eacces-permissions-errors-when-installing-packages-globally)
    1. `mkdir ~/.npm-global`
    1. `npm config set prefix '~/.npm-global'`
    1. `nano ~/.profile`
        1. `export PATH=~/.npm-global/bin:$PATH`
    1. `source ~/.profile `
1. Configure GIT
    1. `git config --global init.defaultBranch main`
    1. `git config --global user.email "you@example.com"`
    1. `git config --global user.name "Your Name"`
