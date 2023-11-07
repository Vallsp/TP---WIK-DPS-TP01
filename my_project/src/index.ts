import http from 'http';

const hostname = '127.0.0.1';
const port = 3000;

const server = http.createServer((req, res) => {
  if (req.method === 'GET' && req.url === '/ping') {
    const headers = req.headers;
    res.setHeader('Content-Type', 'application/json');
    res.end(JSON.stringify({ headers }));
  } else {
    res.statusCode = 404;
    res.end('');
  }
});

server.listen(port, hostname, () => {
  console.log(`Server running at http://${hostname}:${port}/`);
});
