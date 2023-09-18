const { v4: uuidv4 } = require('uuid');

import { unlink } from 'node:fs/promises';
import Bun from 'bun';

const server = Bun.serve({
    port: 3000,
    async fetch(req) {
        try {
            const url = new URL(req.url);
            if (url.pathname === "/") {
                const { searchParams } = new URL(req.url)
                const id = uuidv4();
                const responseJson = {
                    q1: searchParams.get("q1"),
                    q2: searchParams.get("q2"),
                    q3: searchParams.get("q3"),
                    q4: searchParams.get("q4"),
                };

                const filePath = `./${id}.json`;
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

console.log(`Listening on localhost:${server.port}`);

