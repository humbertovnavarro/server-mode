install:
	cargo build -r
	sudo cp target/release/server-mode /usr/bin/server-mode && sudo chmod +x /usr/bin/server-mode
	sudo mkdir -p /etc/server-mode
	sudo cp -n logout.bash /etc/server-mode/logout.bash
	sudo cp -n server-services.json /etc/server-mode/server-services.json
	sudo cp -n desktop-services.json /etc/server-mode/desktop-services.json
