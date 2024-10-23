import { serve } from "https://deno.land/std/http/server.ts";

// deno run --allow-net server.ts
// web server with routing 
const s = serve({ port: 8000 });
console.log("HTTP server is running on http://localhost:8000/");

for await (const req of s) {
    switch (req.url) {
        case "/":
            req.respond({ body: "Welcome to the Home Page!\n" });
            break;
        case "/about":
            req.respond({ body: "This is the About Page!\n" });
            break;
        default:
            req.respond({ status: 404, body: "404 Not Found\n" });
            break;
    }
}
