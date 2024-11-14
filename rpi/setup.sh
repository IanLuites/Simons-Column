#!/usr/bin/env sh

# Configuration
## General
GITHUB_REPO=https://github.com/IanLuites/Simons-Column

## Python
PYTHON_DIST=/usr/lib/python3/dist-packages

## WiFi access point
WIFI_COUNTRY=US
WIFI_SSID="Simon's Column"
WIFI_PASS=Fogarty!
WIFI_IP=10.20.1

set -eu
export DEBIAN_FRONTEND=noninteractive

# Update the system
sudo apt update
sudo apt upgrade -q -y
sudo apt dist-upgrade -q -y

# Install Python helper
sudo mkdir -p "${PYTHON_DIST}/lights"
curl -LsSf "${GITHUB_REPO}/raw/refs/heads/main/python/lights/__init__.py" \
  | sudo tee 1> /dev/null "${PYTHON_DIST}/lights/__init__.py"

# Setup WiFi
sudo raspi-config nonint do_wifi_country "${WIFI_COUNTRY}"
sudo apt install dnsmasq hostapd netfilter-persistent iptables-persistent -q -y

if [ ! -f /etc/dnsmasq.conf.orig ] && [ -f /etc/dnsmasq.conf  ]; then
  sudo mv /etc/dnsmasq.conf /etc/dnsmasq.conf.orig
fi

cat << EOF | sudo tee 1> /dev/null /etc/sysctl.d/routed-ap.conf
net.ipv4.ip_forward=1
EOF

cat << EOF | sudo tee 1> /dev/null /etc/network/interfaces.d/wlan0
auto wlan0
iface wlan0 inet static
address ${WIFI_IP}.1
netmask 255.255.255.0
# gateway ${WIFI_IP}.1
EOF


cat << EOF | sudo tee 1> /dev/null /etc/dnsmasq.conf
interface=wlan0
dhcp-range=${WIFI_IP}.5,${WIFI_IP}.100,255.255.255.0,24
domain=ap
address=/column.local/${WIFI_IP}.1

# address=/#/${WIFI_IP}.1
except-interface=eth1
EOF

cat << EOF | sudo tee 1> /dev/null /etc/hostapd/hostapd.conf
country_code=${WIFI_COUNTRY}

interface=wlan0
ssid=${WIFI_SSID}
hw_mode=g
channel=2
macaddr_acl=0
auth_algs=1
ignore_broadcast_ssid=0
wpa=2
wpa_passphrase=${WIFI_PASS}
wpa_key_mgmt=WPA-PSK
wpa_pairwise=TKIP
rsn_pairwise=CCMP
EOF

sudo iptables -t nat -A POSTROUTING -o eth0 -j MASQUERADE
sudo netfilter-persistent save

sudo systemctl enable dnsmasq
sudo systemctl unmask hostapd.service
sudo systemctl enable hostapd.service

sudo systemctl restart networking.service
sudo systemctl restart dnsmasq.service
sudo systemctl restart hostapd.service
