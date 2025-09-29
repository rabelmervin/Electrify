#!/bin/bash

dnf -y update
dnf -y install python3 net-tools iputils python3-pip openssh-clients openssh-server sudo
dnf clean all

pip3 install --no-cache-dir ansible

useradd -d /home/ansible -s /bin/bash ansible
echo "ansible:ansible123" | sudo chpasswd
echo "ansible ALL=(ALL) NOPASSWD:ALL" | sudo tee -a /etc/sudoers

mkdir -p /home/ansible/.ssh
chown ansible:ansible /home/ansible/.ssh
chmod 700 /home/ansible/.ssh

ssh-keygen -A

echo "PubkeyAuthentication yes" >> /etc/ssh/sshd_config
echo "PasswordAuthentication no" >> /etc/ssh/sshd_config
echo "PermitRootLogin no" >> /etc/ssh/sshd_config
echo "AuthorizedKeysFile .ssh/authorized_keys" >> /etc/ssh/sshd_config