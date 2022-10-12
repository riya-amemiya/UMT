"use strict";
exports.__esModule = true;
var fs = require("fs");
var markdown = require("markdown-wasm");
var check = function (file) {
    var hasfaile = false;
    try {
        fs.statSync(file);
        hasfaile = true;
    }
    catch (err) {
        console.log(err);
        hasfaile = false;
    }
    return hasfaile;
};
var read = function (file) {
    return check(file) ? fs.readFileSync(file, 'utf8') : '';
};
fs.readdir('./doc', function (err, files) {
    if (err) {
        console.log(err);
        return;
    }
    files.forEach(function (file) {
        if (file.endsWith('.md')) {
            var content = read("./doc/".concat(file));
            var html = markdown.parse(content);
            console.log('====================================');
            console.log(html);
            console.log('====================================');
            if (!check('./doc/html')) {
                fs.mkdirSync('./doc/html');
            }
            fs.writeFileSync("./doc/html/".concat(file.replace('.md', '.html')), html);
        }
    });
});
