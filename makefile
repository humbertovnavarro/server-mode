install:
	cargo build -r
	cp target/release/server-mode /usr/bin/server-mode
	mkdir -p /etc/server-mode
	cp -n server-services.json /etc/server-mode/server-services.json
	cp -n desktop-services.json /etc/server-mode/desktop-services.json
