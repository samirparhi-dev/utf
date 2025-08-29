#!/usr/bin/env python3
"""
Simple HTTP server to serve the UFT installer script
Usage: python3 serve-installer.py [port]
"""

import http.server
import socketserver
import sys
import os
from pathlib import Path

# Default port
PORT = int(sys.argv[1]) if len(sys.argv) > 1 else 8000

class InstallerHandler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=str(Path(__file__).parent), **kwargs)
    
    def end_headers(self):
        # Add CORS headers for local testing
        self.send_header('Access-Control-Allow-Origin', '*')
        self.send_header('Access-Control-Allow-Methods', 'GET, POST, OPTIONS')
        self.send_header('Access-Control-Allow-Headers', '*')
        super().end_headers()
    
    def do_GET(self):
        # Serve installer script at root path
        if self.path == '/' or self.path == '/install':
            self.path = '/install.sh'
            self.send_response(200)
            self.send_header('Content-Type', 'text/plain')
            self.end_headers()
            
            with open('install.sh', 'r') as f:
                self.wfile.write(f.read().encode())
        else:
            super().do_GET()

def main():
    with socketserver.TCPServer(("", PORT), InstallerHandler) as httpd:
        print(f"🚀 UFT Installer Server running at http://localhost:{PORT}")
        print(f"📥 Test installation with:")
        print(f"   curl -sSfL http://localhost:{PORT} | sh")
        print(f"💻 View web interface at:")
        print(f"   http://localhost:{PORT}/docs/index.html")
        print()
        print("Press Ctrl+C to stop the server")
        
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")

if __name__ == "__main__":
    main()