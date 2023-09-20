const http = require('http');
const fs = require('fs');
const { v4: uuidv4 } = require('uuid');

const server = http.createServer((req, res) => {
    if (req.method === 'GET') {
        const id = uuidv4();
        const responseJson = {
            q1: "1",
            q2: "2",
            q3: "3",
            q4: "4",
        };
        const filePath = `./json/${id}.json`;
        const jsonContent = JSON.stringify(responseJson);
        fs.writeFile(filePath, jsonContent, (err) => {
            if (err) {
                console.error(err);
                res.writeHead(500, { 'Content-Type': 'text/plain' });
                res.end('Internal Server Error');
                return;
            }

            fs.readFile(filePath, (err, data) => {
                if (err) {
                    console.error(err);
                    res.writeHead(500, { 'Content-Type': 'text/plain' });
                    res.end('Internal Server Error');
                    return;
                }
                fs.unlink(filePath, (err) => {
                    if (err) {
                        console.error(err);
                        res.writeHead(500, { 'Content-Type': 'text/plain' });
                        res.end('Internal Server Error');
                        return;
                    }
                    res.writeHead(200, { 'Content-Type': 'application/json' });
                    res.end(data);
                });
            });
        });
    } else {
        res.writeHead(404, { 'Content-Type': 'text/plain' });
        res.end('Not Found');
    }
});

const port = 3000;

server.listen(port, () => {
    console.log(`Started node server on port ${port}`);
});

