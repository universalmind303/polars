import {fileURLToPath} from 'url';
import path from "path";
import {createRequire} from 'module';
const require = createRequire(import.meta.url);


const __filename = fileURLToPath(import.meta.url)
const __dirname = path.resolve(path.dirname(__filename))

console.log({__filename: fileURLToPath(import.meta.url), __dirname})


var fs = require('fs'),
  http = require('http');

http.createServer(function (req, res) {
  let p = __dirname  + req.url
  if(req.url.includes("bin")) {    
    p = path.resolve(__dirname, "../.." + req.url)
  }
  fs.readFile(p, function (err, data) {
    if (err) {
      res.writeHead(404);
      res.end(JSON.stringify(err));
      return;
    }
    const headers = {
      'Access-Control-Allow-Origin': '*',
    }
    if (req.url.includes('.js')) {
      headers['Content-Type'] = 'application/javascript'
    }
    if (req.url.includes('.wasm')) {
      headers['Content-Type'] = 'application/wasm'
    }
    res.writeHead(200, headers);
    res.end(data);
  });
}).listen(8080);