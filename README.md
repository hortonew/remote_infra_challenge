# Remote Infrastructure Challenge

## Network

![Network](images/network.png)

## Monitoring

![Monitoring](images/monitoring.png)

## Web App

### Dependencies

- Rust and Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html

Note: These dependencies are only required if testing the app locally.

### Configure

a `.env` file inside the web_app directory:

```sh
INDEX_RESPONSE=b
PORT=80
```

or environment variable:

```sh
INDEX_RESPONSE=b PORT=80 cargo run
```

### Try it locally

```sh
cd apps/web_app
cargo run
```

A web server will be running on the port you selected.  Visit `http://localhost:<port>` in a browser.

### Release

A new artifact will be released as code is pushed to main.  You can also manually produce a new artifact by running the Github Action "Build Web App Artifact" manually.

### Deploy

1. Place the web_app binary in /opt/web_app/web_app, with an associated .env file.  On web server B, make sure to change the environment variable to `INDEX_RESPONSE=b`.
2. Configure a systemd service file in /etc/systemd/system/web-app.service (see: [configs/systemd/webapp.service](configs/systemd/webapp.service))
3. Run the following:

```sh
# so the web app can run on ports lower than 1024 (port 80)
sudo setcap 'cap_net_bind_service=+ep' /opt/web_app/web_app

# start the service
sudo systemctl daemon-reload
sudo systemctl enable web-app
sudo systemctl start web-app
```

## Load Balancer

Details from: https://www.haproxy.com/blog/enable-sticky-sessions-in-haproxy

### Deploy

```sh
# 35.160.186.182
sudo apt-get update
sudo apt-get install -y haproxy
```

edit /etc/haproxy/haproxy.cfg with [configs/haproxy/haproxy.cfg](configs/haproxy/haproxy.cfg)

```sh
# validate it's correct
sudo haproxy -c -f /etc/haproxy/haproxy.cfg
# You should see: Configuration file is valid

# restart
sudo systemctl restart haproxy
```

## Monitoring (nagios)

```sh
sudo apt update
sudo apt install -y apache2 libapache2-mod-php php wget unzip autoconf gcc make libgd-dev libmcrypt-dev libssl-dev build-essential

sudo useradd nagios
sudo groupadd nagcmd
sudo usermod -aG nagcmd nagios
sudo usermod -aG nagcmd www-data

cd /tmp
wget https://assets.nagios.com/downloads/nagioscore/releases/nagios-4.5.3.tar.gz
tar -zxvf nagios-4.5.3.tar.gz
cd nagios-4.5.3

sudo ./configure --with-command-group=nagcmd
sudo make all
sudo make install
sudo make install-init
sudo make install-config
sudo make install-commandmode
sudo make install-webconf
sudo make install-cgis

# plugins
cd /tmp
wget https://nagios-plugins.org/download/nagios-plugins-2.4.10.tar.gz
tar -zxvf nagios-plugins-2.4.10.tar.gz
cd nagios-plugins-2.4.10
sudo ./configure --with-nagios-user=nagios --with-nagios-group=nagios
sudo make
sudo make install
sudo htpasswd -c /usr/local/nagios/etc/htpasswd.users nagiosadmin # configure with password.  You will use nagiosadmin / <this password> to log into nagios.

# services
sudo systemctl start nagios
sudo systemctl enable nagios
sudo systemctl enable apache2
sudo systemctl start apache2
```

edit /usr/local/nagios/etc/servers.cfg, apply what's in [configs/nagios/servers.cfg](configs/nagios/servers.cfg)

edit /usr/local/nagios/etc/nagios.cfg, add:

```
cfg_file=/usr/local/nagios/etc/servers.cfg
```

and change `interval_length=60` to `interval_length=1`

Restart nagios

```sh
sudo systemctl restart nagios
```

Configure apache2 virtualhost by overwriting /etc/apache2/sites-available/nagios.conf with what's in [configs/nagios/apache/nagios.conf](configs/nagios/apache/nagios.conf).


Then reload

```sh
sudo a2enmod cgi
sudo a2ensite nagios.conf
sudo a2enmod cgi rewrite
sudo systemctl restart apache2
```

You should be able to get to `http:<ip of nagios server>` and log in with the credentials above (nagiosadmin).

### Custom python checks

Edit: /usr/local/nagios/etc/webservers.txt, placing each web server on a single line.

Edit: /usr/local/nagios/libexec/check_webservers.py, copying in what's in [configs/nagios/check_webservers.py](configs/nagios/check_webservers.py).

Edit: /usr/local/nagios/etc/objects/commands.cfg, adding in:

```
define command {
    command_name    check_webservers_python
    command_line    /usr/bin/python3 /usr/local/nagios/libexec/check_webservers.py
}

define command {
    command_name    check_dummy
    command_line    /usr/local/nagios/libexec/check_dummy $ARG1$
}
```

restart nagios: `sudo systemctl restart nagios`
