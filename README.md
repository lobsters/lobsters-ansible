# Lobste.rs Ansible Playbook

Ansible playbook for lobste.rs.
Lobsters is a technology-focused link aggregation site.

See the notes below if you're using this to set up your own site.

To run:

```bash
ansible-playbook -K prod.yml
```

When working on staging:

```bash
ansible-playbook --inventory=inventories/staging.ini -K staging.yml
```

## Inventory

The following host groups are available:

    db              - SQL server
    dns             - authoritative DNS
    mx*             - incoming email
    smtp*           - outgoing email
    www*            - http over SSL

  groups marked with an asterisk (*) use public SSL certificates.

The following variables are available:

    db_server       - SQL server
    dns_server      - authoritative DNS server
    mx_server       - incoming mail server
    smtp_server     - outgoing mail server
    www_server      - http/s

When a host group has more than one hostname, the _server variable contains the authoritative name for the hosted sevice.
This playbook tries not to distinguish between host variables and group variables.

https://docs.ansible.com/ansible/latest/intro_inventory.html


## Roles

    mariadb         - SQL database.
    lobsters        - web application.
    nginx           - http proxy and SSL termination.
    sysadm          - accounts and ssh shell acess for system administrators.
    postfix         - MX and smtp server.
    lobsters-puma   - App server

https://docs.ansible.com/ansible/latest/playbooks.html
https://docs.ansible.com/ansible/latest/playbooks_reuse_roles.html


## SSH Keys

To use this playbook, you'll need an account in the sysadm role along with an SSH key pair.


## Setup Notes

This is a rough checklist for turning a new Ubuntu LTS VPS into a running instance of Lobsters.
If you're familiar with Linux sysadmin and Rails it should be pretty self-explanatory.
You can drop by `#lobsters` on [Libera Chat](https://libera.chat/) if you have questions.


On the `web01` and `db01` machine:
```bash
ssh root@now box
apt-get update
apt-get upgrade
reboot # will almost certainly be a new kernel
apt-get install certbot # only on web01
```

On the `ansible/sysadmin` machine
```bash
cd lobsters-ansible/inventories
cp prod.ini.sample.ini prod.ini
# edit prod.ini if you need to
cd ../
time ansible-playbook -K prod.yml # should get an error about connecting to database
```

On the `db01` machine:
```bash
sudo apt install mysql-server
sudo mysql_secure_installation
sudo su -
mysql
```
```mysql
create database lobsters_production;
select sha1(concat('mash keyboard', rand()));
create user lobsters@'localhost' identified with mysql_native_password by "[hash-from-sha1-before]"; # may need to be @'%' for any host, an ip, etc
grant all privileges on lobsters.* to 'lobsters'@'localhost'; # match host from prev
```
```bash
vim /etc/mysql/mysql.conf.d/mysqld.cnf
# edit the "bind-address" to 0.0.0.0 if you want remote access
systemctl restart mysql
```

On the `web01` machine:
```bash
# Log in as the "lobsters" user
cd http/

# create /srv/lobste.rs/http/config/database.yml
# Change the "socket" to "host" if you're mysql server is remote
# create /srv/lobste.rs/http/config/initializers/production.rb
# You'll need Pushover, DiffBot, Twitter, GitHub, Keybase API keys
# create /srv/lobste.rs/http/config/secrets.yml

bundle exec rails credentials:edit to create secret key base
chown -R lobsters:lobsters /srv/lobste.rs/http/config
echo "your@email.com" > /root/.forward
run ansible again to deploy code + build assets
reboot again # to see everything comes up properly automatically
```
