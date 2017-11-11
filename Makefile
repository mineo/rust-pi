.PHONY: all transfer

all: target/armv7-unknown-linux-gnueabihf/debug/pi

target/armv7-unknown-linux-gnueabihf/debug/pi: $(wildcard src/*.rs)
	cross build --target armv7-unknown-linux-gnueabihf

transfer: target/armv7-unknown-linux-gnueabihf/debug/pi
	scp $< pi:~
