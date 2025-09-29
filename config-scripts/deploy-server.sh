#!/bin/bash

dnf -y update 
dnf -y install dockernet-tools iputils openssh-clients openssh-server sudo
dnf clean all

useradd -d /home/deployer -s /bin/bash deployer
echo "deployer All=(ALL) NOPASSWD:ALL" | sudo tee -a /etc/sudoers
dnf config-manager --add-repo https://download.docker.com/linux/rhel/docker-ce.repo


dnf install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
dnf clean all 

user -p /home/deployer/ .ssh 
chown deployer:deployer /home/deployer/ .ssh
chmod 700 /home/deployer/ .ssh

usermod -aG docker deployer
ssh-keygen -A


echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config
echo "PasswordAuthentication no" >> /etc/ssh/sshd_config
echo "PermitRootLogin no" >> /etc/ssh/sshd_config
echo "AuthorizedKeysFile .ssh/authorized_keys" >> /etc/ssh/sshd_config


