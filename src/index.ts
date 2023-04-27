import http, { IncomingMessage, ServerResponse } from "http";
import { handle_request } from "./pkg/rust_wasm_nodejs";

const port = process.env.PORT || 3000;

const server = http.createServer(
  async (req: IncomingMessage, res: ServerResponse) => {
    // Handle request in `Rust`
    handle_request(req, res);
  }
);

server.listen(port, () => {
  console.log(`⚡ Server running at http://localhost:${port}`);
});
