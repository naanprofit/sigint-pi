#!/usr/bin/env python3
"""
Faster-Whisper STT server for SIGINT-Pi.
Runs on port 5000, accepts audio via base64 JSON or file upload.
Uses tiny model by default for Pi ARM performance.
"""

import sys
import os
import json
import base64
import tempfile
import logging
from http.server import HTTPServer, BaseHTTPRequestHandler

logging.basicConfig(level=logging.INFO, format='%(asctime)s [whisper] %(message)s')
log = logging.getLogger(__name__)

MODEL_SIZE = os.environ.get("WHISPER_MODEL", "tiny")
DEVICE = os.environ.get("WHISPER_DEVICE", "cpu")
COMPUTE_TYPE = os.environ.get("WHISPER_COMPUTE", "int8")
PORT = int(os.environ.get("WHISPER_PORT", "5000"))

model = None

def load_model():
    global model
    if model is not None:
        return model
    log.info(f"Loading faster-whisper model: {MODEL_SIZE} (device={DEVICE}, compute={COMPUTE_TYPE})")
    from faster_whisper import WhisperModel
    model = WhisperModel(MODEL_SIZE, device=DEVICE, compute_type=COMPUTE_TYPE)
    log.info("Model loaded")
    return model


class WhisperHandler(BaseHTTPRequestHandler):
    def do_POST(self):
        if self.path == "/transcribe":
            self._handle_transcribe()
        else:
            self.send_error(404)

    def do_GET(self):
        if self.path == "/health":
            self._json_response({"status": "ok", "model": MODEL_SIZE})
        else:
            self.send_error(404)

    def _handle_transcribe(self):
        try:
            content_len = int(self.headers.get("Content-Length", 0))
            body = self.rfile.read(content_len)

            content_type = self.headers.get("Content-Type", "")

            if "application/json" in content_type:
                data = json.loads(body)
                audio_b64 = data.get("audio", "")
                audio_bytes = base64.b64decode(audio_b64)
                suffix = ".wav"
            elif "multipart/form-data" in content_type:
                # Simple multipart - extract file bytes
                audio_bytes = body
                suffix = ".wav"
            else:
                audio_bytes = body
                suffix = ".wav"

            with tempfile.NamedTemporaryFile(suffix=suffix, delete=False) as f:
                f.write(audio_bytes)
                tmp_path = f.name

            try:
                m = load_model()
                segments, info = m.transcribe(tmp_path, beam_size=1, language="en")
                text = " ".join(seg.text.strip() for seg in segments)
                log.info(f"Transcribed ({info.duration:.1f}s audio): {text[:80]}")
                self._json_response({
                    "text": text,
                    "language": info.language,
                    "duration": round(info.duration, 2),
                })
            finally:
                os.unlink(tmp_path)

        except Exception as e:
            log.error(f"Transcription error: {e}")
            self._json_response({"error": str(e)}, status=500)

    def _json_response(self, data, status=200):
        body = json.dumps(data).encode()
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def log_message(self, format, *args):
        pass  # suppress default access logs


if __name__ == "__main__":
    log.info(f"Starting whisper server on port {PORT}")
    # Pre-load model at startup
    load_model()
    server = HTTPServer(("0.0.0.0", PORT), WhisperHandler)
    log.info(f"Whisper server ready on http://0.0.0.0:{PORT}")
    try:
        server.serve_forever()
    except KeyboardInterrupt:
        log.info("Shutting down")
        server.shutdown()
