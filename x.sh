cargo watch -c -w src -- cargo r -- build --watch

deno run -A --watch=dist/ main.ts
