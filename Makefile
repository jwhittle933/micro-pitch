.PHONY: connect setup

all: setup obj embed

connect:
	@minicom -D /dev/$(ls /dev/cu.usbmodem* | tail -1) -b 115200

show-device:
	@ls /dev/cu/cu.usbmodem*

setup:
	@rustup target add thumbv7em-none-eabihf
	@brew tap ArmMbed/homebrew-formulae
	@brew install arm-none-eabi-gcc
	@brew install minicom

obj:
	@cargo readobj --target thumbv7em-none-eabihf -- --file-headers

embed:
	@cargo embed --target thumbv7em-none-eabihf