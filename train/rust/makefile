init:
	${HOME}/export-esp.sh
build: init
	cargo build
flash: build
	cargo espflash flash --target xtensa-esp32-espidf --monitor --partition-table partition_table.csv
monitor: init
	cargo espflash monitor