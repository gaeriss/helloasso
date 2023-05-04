CARGO_FLAGS?=

.DEFAULT_GOAL := build

ifeq ($(APP_ENVIRONMENT),prod)
	CARGO_FLAGS+=--release
endif

build:
	cargo build $(CARGO_FLAGS)
.PHONY: build

fw: build/helloasso.ino.bin
.PHONY: fw

build/helloasso.ino.bin: helloasso.ino
	/usr/share/arduino/arduino-builder -compile -logger=machine -hardware /usr/share/arduino/hardware -hardware $(HOME)/.arduino15/packages -tools /usr/share/arduino/tools-builder -tools $(HOME)/.arduino15/packages -libraries $(HOME)/.local/share/arduino/libraries -fqbn=esp8266:esp8266:generic:xtal=80,vt=flash,exception=disabled,stacksmash=disabled,ssl=all,mmu=3232,non32xfer=fast,ResetMethod=nodemcu,CrystalFreq=26,FlashFreq=26,FlashMode=dout,eesz=1M64,led=2,sdk=nonosdk_190703,ip=lm2f,dbg=Disabled,lvl=None____,wipe=none,baud=115200 -vid-pid=10C4_EA60 -ide-version=10819 -build-path build -warnings=all -build-cache cache -prefs=build.warn_data_percentage=75 -prefs=runtime.tools.xtensa-lx106-elf-gcc.path=$(HOME)/.arduino15/packages/esp8266/tools/xtensa-lx106-elf-gcc/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.xtensa-lx106-elf-gcc-3.1.0-gcc10.3-e5f9fec.path=$(HOME)/.arduino15/packages/esp8266/tools/xtensa-lx106-elf-gcc/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.mkspiffs.path=$(HOME)/.arduino15/packages/esp8266/tools/mkspiffs/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.mkspiffs-3.1.0-gcc10.3-e5f9fec.path=$(HOME)/.arduino15/packages/esp8266/tools/mkspiffs/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.mklittlefs.path=$(HOME)/.arduino15/packages/esp8266/tools/mklittlefs/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.mklittlefs-3.1.0-gcc10.3-e5f9fec.path=$(HOME)/.arduino15/packages/esp8266/tools/mklittlefs/3.1.0-gcc10.3-e5f9fec -prefs=runtime.tools.python3.path=$(HOME)/.arduino15/packages/esp8266/tools/python3/3.7.2-post1 -prefs=runtime.tools.python3-3.7.2-post1.path=$(HOME)/.arduino15/packages/esp8266/tools/python3/3.7.2-post1 $^

flash: build/helloasso.ino.bin
	esptool.py --port /dev/ttyUSB0 --chip esp8266 --baud 115200 --before default_reset --after hard_reset write_flash 0x0 $^
.PHONY: flash
