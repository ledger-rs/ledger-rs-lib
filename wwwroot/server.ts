import { serve } from "https://deno.land/std/http/server.ts";
import { serveFile } from "https://deno.land/std/http/file_server.ts";

const server = serve({ port: 8000 });

for await (const req of server) {
  if (req.url === "/") {
    const html = await serveFile(req, "./index.html");
    req.respond(html);
  }
}
