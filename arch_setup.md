# yay 
```bash
  
sudo pacman -S --needed base-devel git
 
git clone https://aur.archlinux.org/yay.git
  
cd yay
  
makepkg -si
  
yay --version

```

# nodejs | npm | php | composer | git | python pip
```bash
sudo pacman -S nodejs npm php composer git 
sudo pacman -S python-pip
```


# vscodium
```bash
yay -S vscodium-bin
```

# mysql mariadb()
```bash

sudo pacman -S mysql

mysqld --version

sudo systemctl start mysqld // maybe reboot here

sudo rm -rf /var/lib/mysql && mkdir /var/lib/mysql

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

# libre office
```bash
sudo pacman -S libreoffice-still
```