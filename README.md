This is a simple Whois API with Rust

```
bash
sudo nano /etc/systemd/system/whois_api.service
```

```
[Unit]
Description=Whois API Service
After=network.target

[Service]
User=root
WorkingDirectory=/path/to/whois_api
ExecStart=/usr/bin/cargo run
Restart=always
Environment="RUST_LOG=info"
StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target

```

```
bash
sudo systemctl daemon-reload
sudo systemctl enable whois_api.service
sudo systemctl start whois_api.service
```

```
bash
sudo systemctl status whois_api.service
```

```
bash
journalctl -u whois_api.service -f
```

```
bash
sudo systemctl restart whois_api.service
```
