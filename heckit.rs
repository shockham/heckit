use std::net::{Shutdown, TcpListener};
use std::thread;
use std::io::Write;


struct Project {
    title: &'static str,
    desc: &'static str,
    href: &'static str,
}

impl Project {
    fn to_html(&self) -> String {
        format!(
            "<a href=\"{href}\" target=\"_blank\">
                <h2>{title}</h2>
                <p>{desc}</p>
                <img src=\"/img/{title}.png\" alt=\"_\">
            </a>",
            title=self.title,
            desc=self.desc,
            href=self.href,
        )
    }
}


fn main() {
    let projects = vec![
        Project {
            title: "flicke",
            desc: "Initially intended to be a flickery fire ray march sketch",
            href: "https://flicke.now.sh/",
        },
        Project {
            title: "weive",
            desc: "Rounded cube ray march sketch",
            href: "https://weive.shockham.now.sh/",
        },
        Project {
            title: "efferve",
            desc: "Effervescent ray march sketch",
            href: "https://efferve.shockham.now.sh/",
        },
        Project {
            title: "effuse",
            desc: "Drippy ray march sketch",
            href: "https://effuse.shockham.now.sh/",
        },
        Project {
            title: "botanea",
            desc: "Botantical ray march sketch",
            href: "https://botanea.shockham.now.sh/",
        },
        Project {
            title: "rhombei",
            desc: "Rhombus ray march sketch",
            href: "https://rhombei.shockham.now.sh/" ,
        },
        Project {
            title: "noiser",
            desc: "FM Synth + step sequencer",
            href: "https://noiser.shockham.now.sh/",
        },
        Project {
            title: "infuse",
            desc: "Minamalist wasm based webgl renderer",
            href: "https://github.com/shockham/infuse"
        },
        Project {
            title: "caper",
            desc: "Minamalist game framework",
            href: "https://github.com/shockham/caper",
        },
        Project {
            title: "volition",
            desc: "Minamalist input lib",
            href: "https://github.com/shockham/volition"
        },
        Project {
            title: "impose",
            desc: "Minamalist audio lib",
            href: "https://github.com/shockham/impose"
        },
    ];

    let projects_html = projects.iter()
        .map(|proj| proj.to_html())
        .collect::<String>();

    let res_string = format!(
        "HTTP/1.1 200 OK\r
        Content-Type: text/html; charset=UTF-8\r\n\r
        <!DOCTYPE html><html>
        <head><title>shockham</title><style></style></head>
        <body><h1>shockham</h1>{}</body>
        </html>\r",
        projects_html
    );

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let res_clone = res_string.clone();

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
