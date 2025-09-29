#!/bin/bash

set -e

dnf clean all
dnf -y update
dnf -y install \
    dnf-plugins-core \
    sudo \
    git \
    wget \
    unzip \
    yum-utils \
    openssh \
    openssh-server \
    openssh-clients \
    java-21-openjdk \
    iputils \
    net-tools

dnf config-manager --add-repo https://download.docker.com/linux/rhel/docker-ce.repo
dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
dnf clean all

useradd -m -s /bin/bash -d /home/jenkinsAgent jenkinsAgent
echo "jenkinsAgent ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers
usermod -aG docker jenkinsAgent


ssh-keygen -A
mkdir -p /etc/ssh/sshd_config.d
cat > /etc/ssh/sshd_config.d/custom.conf <<EOF
PermitRootLogin no
PasswordAuthentication no
ChallengeResponseAuthentication no
PubkeyAuthentication yes
UsePAM yes
EOF


mkdir -p /home/jenkinsAgent/.ssh
chmod 700 /home/jenkinsAgent/.ssh