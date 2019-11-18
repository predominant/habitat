# -*- mode: ruby -*-
# vi: set ft=ruby :

$script = <<-SCRIPT
export DEBIAN_FRONTEND=noninteractive
sudo apt-get update -y
sudo apt-get upgrade -y
sudo apt-get install -y gcc-arm-linux-gnueabihf
sudo apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  direnv \
  file \
  gdb \
  git \
  httpie \
  iproute2 \
  libarchive-dev \
  libprotobuf-dev \
  libsodium-dev \
  libssl-dev \
  libczmq-dev \
  man \
  musl-tools \
  net-tools \
  pkg-config \
  libpq-dev \
  protobuf-compiler \
  software-properties-common \
  sudo \
  tmux \
  vim \
  wget

wget https://www.openssl.org/source/openssl-1.1.1d.tar.gz
tar xf openssl-1.1.1d.tar.gz
cd openssl-1.1.1d
./Configure linux-elf no-asm shared --prefix=/home/vagrant/arm --openssldir=ssl --cross-compile-prefix=arm-linux-gnueabihf-
make && make install
cd ..

wget https://www.libarchive.org/downloads/libarchive-3.4.0.tar.gz
tar xf libarchive-3.4.0.tar.gz
cd libarchive-3.4.0
./configure --host=arm-linux-gnueabihf --prefix=/home/vagrant/arm
make && make install
cd ..

wget https://download.libsodium.org/libsodium/releases/libsodium-1.0.18.tar.gz
tar xf libsodium-1.0.18.tar.gz
cd libsodium-1.0.18
./configure --host=arm-linux-gnueabihf --prefix=/home/vagrant/arm
make && make install
cd ..

curl https://sh.rustup.rs -sSf | sh -s -- -y
source ~/.cargo/env
rustup set profile minimal
rustup target add arm-unknown-linux-gnueabihf
mkdir -p ~/.cargo
cat >> ~/.cargo/config <<EOF
[target.arm-unknown-linux-gnueabihf]
linker = "arm-linux-gnueabihf-gcc"
EOF

cd /home/vagrant/habitat/components/hab
OPENSSL_DIR=/home/vagrant/arm
OPENSSL_LIB_DIR=/home/vagrant/arm/lib \
OPENSSL_INCLUDE_DIR=/home/vagrant/arm/include \
PKG_CONFIG_ALLOW_CROSS=1 \
cargo build --target=arm-unknown-linux-gnueabihf

SCRIPT

Vagrant.configure("2") do |config|
  config.vm.box = "bento/ubuntu-18.04"

  config.vm.provider "virtualbox" do |vb|
    vb.memory = "4096"
    vb.cpus = "4"
  end

  config.vm.provider "vmware_fusion" do |v|
    v.vmx["memsize"] = "4096"
    v.vmx["numvcpus"] = "4"
  end

  # config.vm.provision "file", source: "components/hab/install.sh", destination: "/tmp/install.sh"
  # config.vm.provision "shell", path: "support/linux/install_dev_0_ubuntu_latest.sh"
  # config.vm.provision "shell", path: "support/linux/install_dev_8_docker.sh"
  # config.vm.provision "shell", path: "support/linux/install_dev_9_linux.sh"

  config.vm.synced_folder ".", "/home/vagrant/habitat"

  config.vm.provision "shell", inline: $script
end
