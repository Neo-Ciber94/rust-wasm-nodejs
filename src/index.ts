import http, { IncomingMessage, ServerResponse } from "http";
import { handle_request } from "./pkg/rust_wasm_nodejs";

const port = process.env.PORT || 3000;

const server = http.createServer(
  async (req: IncomingMessage, res: ServerResponse) => {
    // Handle request in `Rust`
    handle_request(req, res);
  }
);

function handleKillSignal() {
  console.log("👋 Received kill signal. Shutting down gracefully.");
  server.close();
  process.exit();
}

process.on("SIGTERM", handleKillSignal);
process.on("SIGINT", handleKillSignal);

server.listen(port, () => {
  console.log(`⚡ Server running at http://localhost:${port}`);
});
