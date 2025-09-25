import { serveDir, serveFile } from "jsr:@std/http/file-server";

Deno.serve(async (req) => {
  const res = await serveDir(req, {
    fsRoot: "dist",
    showDotfiles: true,
  });
  if (res.status == 404) {
    return serveFile(req, `dist/${new URL(req.url).pathname}.html`);
  }

  return res;
});
