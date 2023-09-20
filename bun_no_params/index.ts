const { v4: uuidv4 } = require('uuid');

import { unlink } from 'node:fs/promises';
import Bun from 'bun';

const server = Bun.serve({
    port: 3000,
    async fetch(req) {
        try {
            const uri = new URL(req.url);
            if (uri.pathname === "/") {
                const id = uuidv4();
                const responseJson = {
                    q1: "1",
                    q2: "2",
                    q3: "3",
                    q4: "4",
                };

                const filePath = `./json/${id}.json`;
                const contents = JSON.stringify(responseJson)
                await Bun.write(filePath, contents);
                const f2 = Bun.file(filePath);
                const text = await f2.text();
                await unlink(filePath);
                return new Response(text);
            } else {
                return new Response("404!", { status: 404 });
            }
        } catch (error) {
            console.error(error);
            return new Response("Internal Server Error", { status: 500 });
        }
    },
});

console.log(`Started bun server on port ${server.port}`);

