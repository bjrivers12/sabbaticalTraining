const fs = require('fs');
const http = require ('http');
const url = require('url');

//Blocking

// const textIn = fs.readFileSync('./txt/input.txt', 'utf-8');
// console.log(textIn);
// const textOut = `This is what we know about the avocado: ${textIn}. \nCreated on ${Date.now()}`;
// fs.writeFileSync('./txt/output.txt.', textOut);
// console.log('File Written');

// fs.readFile('./txt/start.txt','utf-8',(err, data) => {
// console.log(data);
// });
// console.log('Will read');


const server = http.createServer((req, res) => {
    console.log(req.url);

    const pathName = req.url;
    if(pathName ==='/' ||pathName==='/overview'){
        res.end('This is the Overview');
    } else if (pathName === '/product'){
        res.end('This is the Product');
    } else {
        res.writeHead(404, {
            'Content-type': 'text/html',
            'my-own-header': 'hello-world'
        });
        res.end('<h1>Page not found!</h1>')
    }

});

server.listen(8000, '127.0.0.1', ()=> {
    console.log("Listening to requests on port 8000");
});



