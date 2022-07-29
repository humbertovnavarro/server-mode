install:
	cargo build -r
	sudo cp target/release/server-mode /usr/bin/server-mode
	mkdir -p /etc/server-mode
	cp -n logout.bash /etc/server-mode/logout.bash
	cp -n server-services.json /etc/server-mode/server-services.json
	cp -n desktop-services.json /etc/server-mode/desktop-services.json
