import time
import json
import os

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
    version int
}

fn build(site Site, path string) {
    cur_dir := 'pages' + path
    files := os.ls(cur_dir) or { return }
    for f in files {
        new_path := cur_dir + '/' + f
        if os.is_dir(new_path) {
            println(new_path)
            build(site, new_path)
        } else {
            dest_path := 'dist' + path + '/' + f +
            if f != 'index.html' {
                '/index.html'
            } else {
                ''
            }
            build_file(site, new_path, dest_path)
        }
    }
}

fn build_file(site Site, src_path string, dest_path string) {
    lines := os.read_lines(src_path) or {
        return
    }
    mut header_json := ''
    mut json_end := -1
    if lines[0] == '---' {
        for i in 1 .. lines.len {
            if lines[i] == '---' {
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
    template_path := 'templates/' + page.layout + '.html'
    println(template_path)
    println($tmpl('templates/default.html'))
}

fn main() {
    site := Site {
        description: 'Araf Al-Jami\'s Blog'
        base_path: ''
        title: 'Araf Al-Jami'
        version: time.now().unix_time()
    }
    page := Page {
        description: 'Araf Al-Jami\'s Blog'
        title: 'Araf Al-Jami',
        layout: 'default'
    }
    body := ''
    println($tmpl('templates/default.html'))
    // os.cp_all('assets', 'dist/assets', true) or {}
    // build(site, '')
}