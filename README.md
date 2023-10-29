# Simple Audio Stream via WebSocket

This programme streams audio bytes via websocket from a from any browser and then converts it to a playable `.wav` file.

## Run

Start back-end websocket server

```bash
cargo run
```

## Record

Run `test.html` file in your browser and start recording.

Once recording stops file is created, stored and playable from local storage.
