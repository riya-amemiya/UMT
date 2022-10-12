import * as fs from 'fs';
import * as markdown from 'markdown-wasm';
const check = (file: string) => {
    let hasfaile = false;
    try {
        fs.statSync(file);
        hasfaile = true;
    } catch (err) {
        console.log(err);

        hasfaile = false;
    }
    return hasfaile;
};
const read = (file: string) => {
    return check(file) ? fs.readFileSync(file, 'utf8') : '';
};
fs.readdir('./doc', (err, files) => {
    if (err) {
        console.log(err);
        return;
    }
    files.forEach((file) => {
        if (file.endsWith('.md')) {
            const content = read(`./doc/${file}`);
            const html = markdown.parse(content);
            console.log('====================================');
            console.log(html);
            console.log('====================================');
            fs.writeFileSync(
                `./doc/html/${file.replace('.md', '.html')}`,
                html,
            );
        }
    });
});
