import asyncio
import json
import websockets


class WebSocketClient:
    def __init__(self, ip):
        self.ip = ip
        self.uri = f"ws://{ip}"
        self.websocket = None

    async def connect(self):
        self.websocket = await websockets.connect(self.uri)

    async def send_json(self, json_payload):
        if not self.websocket:
            await self.connect()
        
        await self.websocket.send(json.dumps(json_payload))
        print(f"Sent JSON payload to {self.ip}: {json_payload}")

    async def close(self):
        if self.websocket:
            await self.websocket.close()
            print(f"Closed connection to {self.ip}")

async def switch_send_json(ip, json_payload):
    uri = f"ws://{ip}"
    
    async with websockets.connect(uri) as websocket:
        await websocket.send(json.dumps(json_payload))
        print(f"Sent JSON payload: {json_payload}")


if __name__ == "__main__":
    ip_address1 = "192.168.118.130"
    ip_address2 = "192.168.118.14"
    
    json_payload_avanti = {
        "action":"setSpeed",
        "value": "1"
    }
    json_payload_indietro = {
        "action": "setSpeed",
        "value": "-1"
    }
    json_payload_avanti2 = {
        "action":"setSpeed",
        "value": "2"
    }
    json_payload_indietro2 = {
        "action": "setSpeed",
        "value": "-2"
    }
    json_payload_stop = {
        "action": "stop"
    }
    json_payload_open = {
        "action": "open"
    }
    json_payload_close = {
        "action": "close"
    }


    async def send_commands():
        client_ip_address1 = WebSocketClient(ip_address1)
        client_ip_address2 = WebSocketClient(ip_address2)

        #client_ip_address1.connect()

        try:
            await client_ip_address2.send_json(json_payload_close)
            await client_ip_address1.send_json(json_payload_avanti)
            await asyncio.sleep(1.5)
            await client_ip_address1.send_json(json_payload_stop)
            await client_ip_address2.send_json(json_payload_open) 
            await asyncio.sleep(1)
            await client_ip_address1.send_json(json_payload_indietro)
            await asyncio.sleep(1)
            await client_ip_address1.send_json(json_payload_indietro2)
            await asyncio.sleep(7)
            await client_ip_address1.send_json(json_payload_stop)
            await asyncio.sleep(1)
            await client_ip_address1.send_json(json_payload_avanti)
            await asyncio.sleep(1)
            await client_ip_address1.send_json(json_payload_avanti2)
            await asyncio.sleep(7.5)
            await client_ip_address1.send_json(json_payload_stop)
            await client_ip_address2.send_json(json_payload_close)
            await asyncio.sleep(1.5)
            await client_ip_address1.send_json(json_payload_indietro)
            await asyncio.sleep(3)

        finally:
            await client_ip_address1.close()
            await client_ip_address2.close()

    asyncio.run(send_commands())


