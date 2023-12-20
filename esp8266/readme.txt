commands:
source ./ENV/bin/activate
platformio init --board esp12e
platformio run --target upload
pio device monitor

if some library not found run:
    pio lib install
    pio update