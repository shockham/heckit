use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;


trait Component {
    fn style() -> String;
    fn to_html(&self) -> String;
}


struct Title;

impl Component for Title {
    fn style() -> String {
        "
            svg {
                height: 90px;
                display: inline-block;
            }

            div {
                float: right;
            }

            .links a {
                color: #fff;
                transition: all 0.5s ease 0s;
            }

            .links a:hover {
                color: #eee;
                opacity: 0.7;
            }
        ".to_string()
    }

    fn to_html(&self) -> String {
        "
            <svg
              xmlns=\"http://www.w3.org/2000/svg\"
              preserveAspectRatio=\"xMidYMin meet\" viewBox=\"0 0 68 90\"
            >
              <g
                transform=\"translate(-68.297193,-19.514977)\"
              >
                <path
                  style=\"fill:#ffffff;fill-opacity:1;stroke:none;stroke-width:0.26458332px;stroke-linecap:butt;stroke-linejoin:miter;stroke-opacity:1\"
                  d=\"M 81.52636,108.52836 V 84.337891 l -13.229167,-16.252975 2.83482,-20.221729 22.300596,-14.174104 6.425594,2.456844 0.188977,9.827381 -11.906237,12.284226 13.229157,12.473215 -0.11071,25.713844 3.07176,0.05011 0.28955,-25.485234 12.96257,-12.962573 -12.16076,-12.428032 0.13364,-7.216277 27.66239,9.621702 3.0736,21.782466 -9.75534,12.294399 0.26727,26.326046 z\"
                />
                <path
                  style=\"fill:#ffffff;fill-opacity:1;fill-rule:evenodd;stroke:none;stroke-width:0.26499999;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1\"
                  d=\"m 103.82727,56.710551 -0.0438,-4.953431 4.36042,4.979675 z m -6.422818,-3e-6 h 4.810848 v -4.944485 z m -4.81085,1.469983 10.156238,-10.289879 10.0226,10.022607 -9.6217,10.156243 z\"
                />
                <path
                  style=\"fill:#ffffff;fill-opacity:1;stroke:none;stroke-width:0.26499999;stroke-linecap:butt;stroke-linejoin:miter;stroke-miterlimit:4;stroke-dasharray:none;stroke-opacity:1\"
                  d=\"m 74.550502,19.536919 -3.2295,24.82999 20.977677,-13.607142 40.537941,13.796129 -3.38998,-25.029109 -10.83137,-0.01181 -0.14175,14.174106 -2.25116,-0.0057 1e-5,-14.110157 -12.35055,-0.01958 -0.0449,9.03278 h -2.26786 l -0.0725,-9.03802 -14.084847,-0.02244 -0.07576,9.473867 -2.114303,-0.03543 v -9.449404 z\"
                />
              </g>
            </svg>
            <div class=\"links\">
              <a href=\"https://twitter.com/shockham\" target=\"_blank\">tw</a>
              <a href=\"https://github.com/shockham\" target=\"_blank\">gh</a>
              <a href=\"https://shockham.bandcamp.com/\" target=\"_blank\">bc</a>
              <a rel=\"me\" href=\"https://merveilles.town/@shockham\" target=\"_blank\">
                  fedi
              </a>
            </div>
        ".to_string()
    }
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
    title_html: String,
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

            h1 {
                display: inline-block;
                border-bottom: 1px solid #fff;
                width: 100%;
                height: 88px;
                margin: 0;
            }
        ".to_string()
    }

    fn to_html(&self) -> String {
        let global_style = Root::style();
        let title_style = Title::style();
        let projects_style = Project::style();
        let app_style = App::style();
        let all_style = format!(
            "{}{}{}{}",
            global_style,
            title_style,
            app_style,
            projects_style,
        );

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
                <h1>
                    {title}
                </h1>
                {body}
            </body>
            </html>\r\n",
            style=all_style,
            title=self.title_html,
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
            img: Some("https://assets.merveilles.town/media_attachments/files/000/519/799/original/89011a1732a64908.png"),
        },
        Project {
            title: "weive",
            desc: "Rounded cube ray march sketch",
            href: "https://weive.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/430/541/original/940c19082a0cff2f.png"),
        },
        Project {
            title: "efferve",
            desc: "Effervescent ray march sketch",
            href: "https://efferve.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/251/573/original/d91f2734645018db.png"),
        },
        Project {
            title: "effuse",
            desc: "Drippy ray march sketch",
            href: "https://effuse.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/252/052/original/e4a48e192d95c07e.png"),
        },
        Project {
            title: "botanea",
            desc: "Botantical ray march sketch",
            href: "https://botanea.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/383/565/original/64bfce63f89209e2.png"),
        },
        Project {
            title: "rhombei",
            desc: "Rhombus ray march sketch",
            href: "https://rhombei.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/388/210/original/a9c2551e9be1b0f6.png"),
        },
        Project {
            title: "noiser",
            desc: "FM Synth + step sequencer",
            href: "https://noiser.shockham.now.sh/",
            img: Some("https://assets.merveilles.town/media_attachments/files/000/506/971/original/2e99238bf0de5ee4.png"),
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

    let title_html = Title {}.to_html();

    let app = App { projects_html };

    let body_html = app.to_html();

    let root = Root {
        title_html,
        body_html
    };

    let root_html = root.to_html();

    let res_string = format!(
        "HTTP/1.1 200 OK\nContent-Type: text/html; charset=UTF-8\nContent-Length: {}\n\n{}",
        root_html.len(),
        root_html
    );

    start_server(res_string);
}

fn start_server(response_string: String) {
    let listener = TcpListener::bind("0.0.0.0:80").unwrap();

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
