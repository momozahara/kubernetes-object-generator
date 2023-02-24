use tui::widgets::ListState;

#[derive(Debug)]
pub struct App {
    pub input_mode: InputMode,
    pub kube: Kube,
    pub selected_state: SelectedState,
}

#[derive(Debug)]
pub enum InputMode {
    Normal,
    Insert,
    Select,
}

#[derive(Debug)]
pub enum ServiceType {
    NodePort,
    LoadBalancer,
}

#[derive(Debug)]
pub struct Kube {
    pub name: String,
    pub label: String,
    pub cname: String,
    pub image: String,
    pub secret: String,
    pub stype: ServiceType,
}

#[derive(Debug)]
pub struct SelectedState {
    pub max: usize,
    pub current: ListState,
}

impl Default for App {
    fn default() -> Self {
        App {
            input_mode: InputMode::Normal,
            kube: Kube {
                name: String::new(),
                label: String::new(),
                cname: String::new(),
                image: String::new(),
                secret: String::new(),
                stype: ServiceType::NodePort,
            },
            selected_state: SelectedState::default(),
        }
    }
}

impl SelectedState {
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    pub fn set_current(&mut self, current: usize) {
        self.current.select(Some(current));
    }

    pub fn next(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i >= self.max - 1 {
                    i
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }

    pub fn prev(&mut self) {
        let i = match self.current.selected() {
            Some(i) => {
                if i == 0 {
                    0
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.current.select(Some(i));
    }
}

impl Default for SelectedState {
    fn default() -> SelectedState {
        SelectedState {
            max: 0,
            current: ListState::default(),
        }
    }
}
