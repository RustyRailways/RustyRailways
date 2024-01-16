from http.server import BaseHTTPRequestHandler, HTTPServer
from io import BytesIO

class SimpleHTTPRequestHandler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.end_headers()
        response = BytesIO()
        response.write(b'GET request received')
        self.wfile.write(response.getvalue())

    def do_POST(self):
        content_length = int(self.headers['Content-Length'])
        payload = self.rfile.read(content_length)

        # Print the payload
        print(f"Received POST request. Payload: {payload.decode('utf-8')}")

        self.send_response(200)
        self.end_headers()
        response = BytesIO()
        response.write(b'POST request received')
        self.wfile.write(response.getvalue())

def run(server_class=HTTPServer, handler_class=SimpleHTTPRequestHandler, port=9000):
    server_address = ('', port)
    httpd = server_class(server_address, handler_class)
    print(f"Starting server on port {port}")
    httpd.serve_forever()

if __name__ == '__main__':
    run()
