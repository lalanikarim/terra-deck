#!/usr/bin/env python3
"""Custom HTTP server with WASM MIME type support for Terra-Deck."""

import http.server
import socketserver
import mimetypes
import sys
import os

# Add WASM MIME type
mimetypes.add_type('application/wasm', '.wasm')
mimetypes.add_type('text/javascript', '.js')
mimetypes.add_type('application/json', '.json')

class QuietHandler(http.server.SimpleHTTPRequestHandler):
    """Handler that suppresses most logs."""
    
    def log_message(self, format, *args):
        """Log only errors."""
        if "404" in format or "500" in format:
            print(f"{self.address_string()} - {format % args}")

def main():
    port = 8000
    
    # Change to the wasm directory
    os.chdir(os.path.dirname(os.path.abspath(__file__)))
    
    print(f"✅ Terra-Deck WASM Server")
    print(f"   URL: http://localhost:{port}/web/index.html")
    print(f"   Press Ctrl+C to stop")
    print()
    
    Handler = QuietHandler
    
    with socketserver.TCPServer(("", port), Handler) as httpd:
        try:
            httpd.serve_forever()
        except KeyboardInterrupt:
            print("\n👋 Server stopped")
            sys.exit(0)

if __name__ == "__main__":
    main()
