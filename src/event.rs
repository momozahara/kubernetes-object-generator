use std::{
    fs::File,
    io::{self, Write},
};

use crossterm::event::{KeyCode, KeyEvent};

use crate::{
    app::{InputMode, ServiceType},
    App,
};

static DEPLOYMENT: &str = include_str!("../templates/deployment.yaml");
static DEPLOYMENT_SECRET: &str = include_str!("../templates/deployments.yaml");
static SERVICE: &str = include_str!("../templates/service.yaml");

pub fn main(app: &mut App, key: KeyEvent) -> Option<io::Result<()>> {
    app.selected_state.set_max(7);
    let selected = app.selected_state.current.selected().unwrap();
    match app.input_mode {
        InputMode::Normal => match key.code {
            KeyCode::Esc | KeyCode::Char('q') => return Some(Ok(())),
            KeyCode::Up | KeyCode::Char('k') => app.selected_state.prev(),
            KeyCode::Down | KeyCode::Char('j') => app.selected_state.next(),
            KeyCode::Enter | KeyCode::Char('e') => match selected {
                0..=4 => app.input_mode = InputMode::Insert,
                5 => app.input_mode = InputMode::Select,
                6 => {
                    let mut deployment: String;
                    let mut service = SERVICE.clone().to_owned();

                    match app.kube.secret.is_empty() {
                        false => {
                            deployment = DEPLOYMENT_SECRET.clone().to_owned();
                            deployment = deployment.replace("{secret}", &app.kube.secret);
                        }
                        true => {
                            deployment = DEPLOYMENT.clone().to_owned();
                        }
                    }

                    deployment = deployment.replace("{name}", &app.kube.name);
                    deployment = deployment.replace("{label}", &app.kube.label);
                    deployment = deployment.replace("{cname}", &app.kube.cname);
                    deployment = deployment.replace("{image}", &app.kube.image);

                    service = service.replace("{name}", &app.kube.name);
                    service = service.replace("{label}", &app.kube.label);
                    service = service.replace(
                        "{type}",
                        match app.kube.stype {
                            ServiceType::NodePort => "NodePort",
                            ServiceType::LoadBalancer => "LoadBalancer",
                        },
                    );

                    {
                        let mut file = File::create("./deployment.yaml").unwrap();
                        file.write_all(deployment.as_bytes()).unwrap();
                    }

                    {
                        let mut file = File::create("./service.yaml").unwrap();
                        file.write_all(service.as_bytes()).unwrap();
                    }

                    return Some(Ok(()));
                }
                _ => return None,
            },
            _ => (),
        },
        InputMode::Insert => match key.code {
            KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
            KeyCode::Char(c) => match selected {
                0 => {
                    app.kube.name.push(c);
                }
                1 => {
                    app.kube.label.push(c);
                }
                2 => {
                    app.kube.cname.push(c);
                }
                3 => {
                    app.kube.image.push(c);
                }
                4 => {
                    app.kube.secret.push(c);
                }
                _ => (),
            },
            KeyCode::Backspace => match selected {
                0 => {
                    app.kube.name.pop();
                }
                1 => {
                    app.kube.label.pop();
                }
                2 => {
                    app.kube.cname.pop();
                }
                3 => {
                    app.kube.image.pop();
                }
                4 => {
                    app.kube.secret.pop();
                }
                _ => (),
            },
            _ => (),
        },
        InputMode::Select => match key.code {
            KeyCode::Enter | KeyCode::Esc => app.input_mode = InputMode::Normal,
            KeyCode::Up | KeyCode::Char('k') => match app.kube.stype {
                ServiceType::NodePort => {
                    app.kube.stype = ServiceType::LoadBalancer;
                }
                ServiceType::LoadBalancer => {
                    app.kube.stype = ServiceType::NodePort;
                }
            },
            KeyCode::Down | KeyCode::Char('j') => match app.kube.stype {
                ServiceType::NodePort => {
                    app.kube.stype = ServiceType::LoadBalancer;
                }
                ServiceType::LoadBalancer => {
                    app.kube.stype = ServiceType::NodePort;
                }
            },
            _ => (),
        },
    }

    return None;
}
