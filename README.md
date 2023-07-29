# Terminal QR Code

A fast terminal based qr code generator made with rust.

**Still under development**

## Useage

Make a QR code with defaults:
```bash
tqr "text"
```

Detects URLs automaticly:
```bash
tqr "https://www.youtube.com/watch?v=dQw4w9WgXcQ"
```
Set error correction level (1-4):
```bash
tqr -e 2 "text"
```
