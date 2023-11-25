command:
source ./ENV/bin/activate
platformio init --board esp12e
platformio run --target upload
pio device monitor