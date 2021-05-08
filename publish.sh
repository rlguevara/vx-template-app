#!/usr/bin/env bash

# Host vx-vm
#     HostName 40.65.221.33
#     KeepAlive yes
#     ServerAliveInterval 60
#     Port 22
#     User vxuser
#     IdentityFile ~/.ssh/id_rsa

trunk build --release && \
scp -r dist/* vx-vm:/var/www/app.vx-template-app.network/html
