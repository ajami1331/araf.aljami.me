const fse = require('fs-extra');
const path = require('path');
const ejs = require('ejs');
const {
    marked
} = require('marked');
const frontMatter = require('front-matter');
const glob = require('glob');
const config = require('./config');

const srcPath = config.build.srcPath;
const distPath = config.build.outputPath;
const cleanUrls = config.site.cleanUrls;

const buildPage = (file, i, debug) => {
    const fileData = path.parse(file);
    let destPath = path.join(distPath, fileData.dir);

    if (cleanUrls && fileData.name !== 'index') {
        destPath = path.join(destPath, fileData.name);
    }
    // create destination directory
    fse.mkdirsSync(destPath);

    // read page file
    const data = fse.readFileSync(`${srcPath}/pages/${file}`, 'utf-8');

    // render page
    const pageData = frontMatter(data);
    const templateConfig = Object.assign({}, config, {
        page: pageData.attributes
    });
    templateConfig['site']['debug'] = debug;

    let pageContent;

    // generate page content according to file type
    switch (fileData.ext) {
        case '.md':
            pageContent = marked(pageData.body);
            break;
        case '.ejs':
            pageContent = ejs.render(pageData.body, templateConfig, {
                filename: `${srcPath}/pages/${file}`
            });
            break;
        default:
            pageContent = pageData.body;
    }

    // render layout with page contents
    const layout = pageData.attributes.layout || 'default';
    const layoutFileName = `${srcPath}/layouts/${layout}.ejs`;
    const layoutData = fse.readFileSync(layoutFileName, 'utf-8');
    const completePage = ejs.render(
        layoutData,
        Object.assign({}, templateConfig, {
            body: pageContent,
            filename: layoutFileName
        })
    );
    // save the html file
    if (cleanUrls) {
        fse.writeFileSync(`${destPath}/index.html`, completePage);
    } else {
        fse.writeFileSync(`${destPath}/${fileData.name}.html`, completePage);
    }
};

const updateBlogIndex = () => {
    const files = glob.sync('**/*.@(md|ejs|html)', {
        cwd: `${srcPath}/pages/blogs`
    });
    let blogIndexStrArray = [`---`,
        `layout: markdown`,
        `Title: Posts`,
        `---`
    ];
    files.forEach(file => {
        const fileData = path.parse(file);
        const url = `blogs/${fileData.name}${cleanUrls ? '/' : '.html'}`
        const data = fse.readFileSync(`${srcPath}/pages/blogs/${fileData.name}`, 'utf-8');
        const pageData = frontMatter(data).attributes;
        blogIndexStrArray.push(`>> - [${pageData.Title}](/${url})`)
    });
    fse.writeFileSync(`${srcPath}/pages/blog.md`, blogIndexStrArray.join('\n'));
}

const build = (debug) => {
    console.log('build started');

    // clear destination folder
    fse.emptyDirSync(distPath);

    // copy assets folder
    fse.copy(`${srcPath}/assets`, `${distPath}/assets`);

    // copy public folder
    fse.copy(`${srcPath}/public`, `${distPath}`);

    updateBlogIndex();

    // read pages
    const files = glob.sync('**/*.@(md|ejs|html)', {
        cwd: `${srcPath}/pages`
    });

    files.forEach((f, i) => buildPage(f, i, debug));

    console.log('build finished');
};

module.exports = build;