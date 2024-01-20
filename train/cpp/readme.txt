command:
source ./ENV/bin/activate
platformio init --board esp12e

git clone https://github.com/Links2004/arduinoWebSockets.git

platformio run --target upload
pio device monitor