#!/usr/bin/env python3
"""
SIGINT-Deck Whisper Transcription Server
Provides a simple HTTP API for speech-to-text using faster-whisper
"""

import os
import sys
import json
import tempfile
import base64
from http.server import HTTPServer, BaseHTTPRequestHandler
from faster_whisper import WhisperModel

# Use tiny model for speed on Steam Deck (can change to base/small for better accuracy)
MODEL_SIZE = os.environ.get("WHISPER_MODEL", "tiny")
DEVICE = os.environ.get("WHISPER_DEVICE", "cpu")  # cpu or cuda
PORT = int(os.environ.get("WHISPER_PORT", "5000"))

print(f"Loading Whisper model: {MODEL_SIZE} on {DEVICE}...")
model = WhisperModel(MODEL_SIZE, device=DEVICE, compute_type="int8")
print("Model loaded!")

class WhisperHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path == "/transcribe":
            content_length = int(self.headers["Content-Length"])
            post_data = self.rfile.read(content_length)
            
            try:
                data = json.loads(post_data)
                audio_base64 = data.get("audio")
                
                if not audio_base64:
                    self.send_error(400, "No audio data provided")
                    return
                
                # Decode base64 audio
                audio_bytes = base64.b64decode(audio_base64)
                
                # Write to temp file
                with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as f:
                    f.write(audio_bytes)
                    temp_path = f.name
                
                # Transcribe
                segments, info = model.transcribe(temp_path, beam_size=5)
                text = " ".join([segment.text for segment in segments])
                
                # Cleanup
                os.unlink(temp_path)
                
                # Send response
                self.send_response(200)
                self.send_header("Content-Type", "application/json")
                self.send_header("Access-Control-Allow-Origin", "*")
                self.end_headers()
                response = {
                    "text": text.strip(),
                    "language": info.language,
                    "language_probability": info.language_probability
                }
                self.wfile.write(json.dumps(response).encode())
                
            except Exception as e:
                self.send_error(500, str(e))
        
        elif self.path == "/health":
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.end_headers()
            self.wfile.write(json.dumps({"status": "ok", "model": MODEL_SIZE}).encode())
        
        else:
            self.send_error(404, "Not found")
    
    def do_GET(self):
        if self.path == "/health":
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Access-Control-Allow-Origin", "*")
            self.end_headers()
            self.wfile.write(json.dumps({"status": "ok", "model": MODEL_SIZE}).encode())
        else:
            self.send_error(404, "Not found")
    
    def do_OPTIONS(self):
        self.send_response(200)
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        self.end_headers()
    
    def log_message(self, format, *args):
        print(f"[Whisper] {args[0]}")

if __name__ == "__main__":
    server = HTTPServer(("127.0.0.1", PORT), WhisperHandler)
    print(f"Whisper server running on http://127.0.0.1:{PORT}")
    print("Endpoints: POST /transcribe, GET /health")
    server.serve_forever()
