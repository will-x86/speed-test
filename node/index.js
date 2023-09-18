const http = require('http');
const fs = require('fs');
const { v4: uuidv4 } = require('uuid');
const url = require('url');
const querystring = require('querystring');


const server = http.createServer((req, res) => {
    if (req.method === 'GET') {
        const parsedUrl = url.parse(req.url);
        const queryParamsString = parsedUrl.query;
        const queryParams = querystring.parse(queryParamsString);


        const id = uuidv4();
        const responseJson = {
            q1: queryParams.q1,
            q2: queryParams.q2,
            q3: queryParams.q3,
            q4: queryParams.q4,
        };

        console.log(queryParams.q1)
        console.log(queryParams.q2)
        const filePath = `./${id}.json`;
        const jsonContent = JSON.stringify(responseJson);
        console.log(jsonContent)

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
    console.log(`Listening on http://localhost:${port}`);
});

