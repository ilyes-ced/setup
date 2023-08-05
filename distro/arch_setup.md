# yay 
```bash
  
sudo pacman -S --needed base-devel git
 
git clone https://aur.archlinux.org/yay.git
  
cd yay
  
makepkg -si
  
yay --version

```

# installs
```bash
sudo pacman -S nodejs npm php composer git python-pip neovim tmux ranger ueberzug
yay -S vscodium-bin
sudo npm -g typescript pnpm
#tmux plugin manager
git clone https://github.com/tmux-plugins/tpm ~/.tmux/plugins/tpm

git clone https://github.com/NvChad/NvChad ~/.config/nvim --depth 1 && nvim
# not tested
git clone https://github.com/noot-noot-pengu/neovim_config nvim && mv -vi ./nvim ~/.config
```



# mysql mariadb()
```bash

sudo pacman -S mysql

mysqld --version

sudo systemctl start mysqld

sudo rm -rf /var/lib/mysql && sudo mkdir /var/lib/mysql

mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql

sudo systemctl status mysqld

sudo systemctl enable mysqld

sudo mysql_secure_installation

sudo mysql

CREATE USER ‘username’@’localhost’ IDENTIFIED BY 'password';

GRANT ALL PRIVILEGES ON *.* TO ‘username’@’localhost’ WITH GRANT OPTION;

FLUSH PRIVILEGES;

exit

sudo mysql -p // as root 

sudo -u ilyes -p // as user
```
- mongodb
```bash
yay -S mongodb-bin

sudo systemctl start mongodb // maybe reboot

sudo systemctl status mongodb

sudo systemctl enable mongodb

mongo --version

```