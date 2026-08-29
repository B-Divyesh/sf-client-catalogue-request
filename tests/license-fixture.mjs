import { createServer } from 'node:http';

const server = createServer((request, response) => {
  const url = new URL(request.url ?? '/', 'http://127.0.0.1:8181');
  const valid = url.pathname === '/verify' && url.searchParams.get('license') === 'test-license';
  response.writeHead(200, { 'content-type': 'application/json' });
  response.end(JSON.stringify({ valid, reason: valid ? 'ok' : 'invalid', expires_at: null }));
});

server.listen(8181, '127.0.0.1');
for (const signal of ['SIGINT', 'SIGTERM']) process.on(signal, () => server.close());
