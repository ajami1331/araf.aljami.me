import time
import json
import os
import markdown

struct Page {
    description string
    title string
    date string
    layout string
}

struct Site {
    description string
    base_path string
    title string
    version i64
}

fn build(site Site, path string) {
    cur_dir := 'pages' + path
    files := os.ls(cur_dir) or { return }
    for f in files {
        new_path := cur_dir + '/' + f
        if os.is_dir(new_path) {
            build(site, new_path['pages'.len..new_path.len])
        } else {
            dest_path := 'dist' + path + '/' + f[0..(f.len - os.file_ext(f).len)] +
            if f != 'index.html' {
                '/index.html'
            } else {
                os.file_ext(f)
            }            
            os.mkdir_all (os.dir(dest_path)) or {}
            built_file := build_file(site, new_path, os.file_ext(f) == '.md')
            os.write_file(dest_path, built_file) or {}
        }
    }
}

fn build_file(site Site, src_path string, is_markdown bool) string {
    lines := os.read_lines(src_path) or {
        return ''
    }
    mut header_json := ''
    mut json_end := -1
    if lines[0].contains('---') {
        for i in 1 .. lines.len {
            if lines[i].contains('---') {
                json_end = i
                break
            }
            header_json += lines[i]
        }
    }
    page := json.decode(Page, header_json) or {
        Page {
            description: 'Araf Al-Jami\'s Blog'
            title: 'Araf Al-Jami',
            layout: 'default'
        }
    }
    mut body := ''
    for i in json_end + 1 .. lines.len {
        body += lines[i]
    }
    if is_markdown {
        body = markdown.to_html(body)
    }
    if page.layout == 'blog' {
        return blog(site, page, body)
    }
    if page.layout == 'markdown' {
        return markdown(site, page, body)
    }
    return $tmpl('templates/default.html')
}

fn blog(site Site, page Page, body string) string {
    return $tmpl('templates/blog.html')
}

fn markdown(site Site, page Page, body string) string {
    return $tmpl('templates/markdown.html')
}

fn main() {
    site := Site {
        description: 'Araf Al-Jami\'s Blog'
        base_path: ''
        title: 'Araf Al-Jami'
        version: time.now().unix_time()
    }
    os.rmdir_all('dist') or {}
    os.mkdir('dist') or {}
    os.cp_all('assets', 'dist/assets', true) or {}
    build(site, '')
}