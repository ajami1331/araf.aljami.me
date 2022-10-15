const build = require('./build');
build(true);
const port = process.env.PORT || 8080;
const http = require('http');
const reload = require('reload');
const express = require("express");
const distDir = __dirname + "/dist/";
console.log(distDir)
const app = express();
app.use(express.static(distDir));
const server = http.createServer(app)
server.listen(port, () => console.log(`Example app listening on port ${port}!`))
reload(app);