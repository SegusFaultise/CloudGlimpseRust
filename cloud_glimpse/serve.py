import http.server
import socketserver

class bcolors:
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'

PORT = 8000

Handler = http.server.SimpleHTTPRequestHandler
Handler.extensions_map = {
    '.html': 'text/html',
    '.wasm': 'application/wasm',
    '': 'application/octet-stream',
}

with socketserver.TCPServer(("", PORT), Handler) as httpd:
    print(f"{bcolors.OKGREEN}[SUCCESS]:{bcolors.ENDC} serving at, {bcolors.UNDERLINE + bcolors.OKCYAN}http://localhost:{PORT}/src/website/index.html{bcolors.ENDC}")

    httpd.serve_forever()
