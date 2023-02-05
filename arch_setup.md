# yay 
```
  
sudo pacman -S --needed base-devel git
 
git clone https://aur.archlinux.org/yay.git
  
cd yay
  
makepkg -si
  
yay --version

```

# nodejs | npm | php | composer | git | python pip
```
sudo pacman -S nodejs npm php composer git 
sudo pacman -S python-pip
```


# vscodium
```
yay -S vscodium-bin
```

# mysql mariadb()
```

sudo pacman -S mysql

mysqld --version

sudo systemctl start mysqld // maybe reboot here

sudo systemctl status mysqld

sudo systemctl enable mysqld

sudo mysql_secure_installation

sudo mysql

CREATE USER ‘ilyes’@’localhost’ IDENTIFIED BY 'passpass';

GRANT ALL PRIVILEGES ON *.* TO ‘ilyes’@’localhost’ WITH GRANT OPTION;

FLUSH PRIVILEGES;

exit

sudo mysql -p // as root 

sudo -u ilyes -p // as user
```
- mongodb
```
yay -S mongodb-bin

sudo systemctl start mongodb // maybe reboot

sudo systemctl status mongodb

sudo systemctl enable mongodb

mongo --version

```
-
-
-