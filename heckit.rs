use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;


trait Component {
    fn style() -> String;
    fn to_html(&self) -> String;
}

struct Project {
    title: &'static str,
    desc: &'static str,
    href: &'static str,
    img: Option<&'static str>,
}

impl Component for Project {
    fn style() -> String {
        "
            .project {
                background: #fff;
                color: #111;
                padding: 0 10px;
                transition: all 0.5s ease 0s;
                cursor: pointer;
                text-decoration: none;
            }
            .project:hover {
                opacity: 0.7;
            }
            .project img {
                width: 100%;
            }
        ".to_string()
    }

    fn to_html(&self) -> String {
        let img_str = if let Some(url) = self.img {
            format!("<img src=\"{}\" alt=\"_\">", url)
        } else {
            "".to_string()
        };

        format!(
            "<a class=\"project\" href=\"{href}\" target=\"_blank\">
                <h2>{title}</h2>
                <p>{desc}</p>
                {img}
            </a>",
            title=self.title,
            desc=self.desc,
            href=self.href,
            img=img_str,
        )
    }
}


struct App {
    projects_html: String,
}

impl Component for App {
    fn style() -> String {
        "
            .app {
                width: 100%;
                display: grid;
                padding: 20px 0;
                grid-gap: 20px;
                grid-template-columns: repeat(4, 1fr);
                grid-auto-flow: row dense;
            }
            @media only screen and (max-width: 640px) {
                .app {
                    grid-template-columns: repeat(1, 1fr);
                }
            }
        ".to_string()
    }

    fn to_html(&self) -> String {
        format!("<div class=\"app\">{}</div>", self.projects_html)
    }
}


struct Root {
    body_html: String,
}

impl Component for Root {
    fn style() -> String {
        "
            html, body {
                position: relative;
                width: 100%;
                height: 100%;
            }

            body {
                background: #111;
                color: #fff;
                margin: 0;
                padding: 2%;
                box-sizing: border-box;
                font-family: monospace;
            }
        ".to_string()
    }

    fn to_html(&self) -> String {
        let global_style = Root::style();
        let projects_style = Project::style();
        let app_style = App::style();
        let mut all_style = format!(
            "{}{}{}",
            global_style,
            app_style,
            projects_style,
        );
        all_style.retain(|c| c != ' ');

        format!(
            "<!DOCTYPE html>
            <html>
            <head>
                <title>shockham</title>
                <style>
                    {style}
                </style>
            </head>
            <body>
                <h1>shockham</h1>
                {body}
            </body>
            </html>\r",
            style=all_style,
            body=self.body_html,
        )
    }
}


fn main() {
    let projects = vec![
        Project {
            title: "flicke",
            desc: "Initially intended to be a flickery fire ray march sketch",
            href: "https://flicke.now.sh/",
            img: None,
        },
        Project {
            title: "weive",
            desc: "Rounded cube ray march sketch",
            href: "https://weive.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "efferve",
            desc: "Effervescent ray march sketch",
            href: "https://efferve.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "effuse",
            desc: "Drippy ray march sketch",
            href: "https://effuse.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "botanea",
            desc: "Botantical ray march sketch",
            href: "https://botanea.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "rhombei",
            desc: "Rhombus ray march sketch",
            href: "https://rhombei.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "noiser",
            desc: "FM Synth + step sequencer",
            href: "https://noiser.shockham.now.sh/",
            img: None,
        },
        Project {
            title: "infuse",
            desc: "Minamalist wasm based webgl renderer",
            href: "https://github.com/shockham/infuse",
            img: None,
        },
        Project {
            title: "caper",
            desc: "Minamalist game framework",
            href: "https://github.com/shockham/caper",
            img: None,
        },
        Project {
            title: "volition",
            desc: "Minamalist input lib",
            href: "https://github.com/shockham/volition",
            img: None,
        },
        Project {
            title: "impose",
            desc: "Minamalist audio lib",
            href: "https://github.com/shockham/impose",
            img: None,
        },
    ];

    let projects_html = projects.iter()
        .map(|proj| proj.to_html())
        .collect::<String>();

    let app = App { projects_html };

    let body_html = app.to_html();

    let root = Root { body_html };

    let res_string = format!(
        "HTTP/1.1 200 OK\r
        Content-Type: text/html; charset=UTF-8\r\n\r
        {}",
        root.to_html()
    );

    start_server(res_string);
}

fn start_server(response_string: String) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let res_clone = response_string.clone();

        thread::spawn(move || {
            let mut stream = stream.unwrap();
            let res_slice = res_clone.as_bytes();
            match stream.write(res_slice) {
                Ok(_) => println!("Response sent!"),
                Err(e) => println!("Failed sending response: {}!", e),
            }
            stream.shutdown(Shutdown::Write).unwrap();
        });
    }
}
