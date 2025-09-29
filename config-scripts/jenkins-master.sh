#!/bin/bash

set -e

dnf clean all
dnf -y update 

dnf install -y \
    dnf-plugins-core \
    sudo \
    git \
    wget \
    unzip \
    yum-utils \
    openssh \
    openssh-server \
    openssh-clients \

dnf clean all

wget -O /etc/yum.repos.d/jenkins.repo https://pkg.jenkins.io/redhat/jenkins.repo
rpm --import https://pkg.jenkins.io/redhat/jenkins.io-2023.key
dnf upgrade -y
dnf install -y fontconfig java-21-openjdk jenkins
dnf clean all

chown jenkins:jenkins /usr/share/java/jenkins.war

ssh-keygen -A

mkdir -p /etc/ssh/sshd_config.d

cat > /etc/ssh/sshd_config.d/custom.conf << EOF
PermitRootLogin no
PasswordAuthentication no
ChallengeResponseAuthentication no
PubkeyAuthentication yes
UsePAM yes
EOF

mkdir -p /var/lib/jenkins/.ssh
chmod 700 /var/lib/jenkins/.ssh

chown -R jenkins:jenkins /var/lib/jenkins/.ssh

ssh-keygen -t rsa -N "" -f /var/lib/jenkins/.ssh/id_rsa
chown -R jenkins:jenkins /var/lib/jenkins/.ssh/*
chmod 400 /var/lib/jenkins/.ssh/id_rsa

