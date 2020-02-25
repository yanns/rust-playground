#![recursion_limit = "512"]

use vgtk::ext::*;
use vgtk::lib::gdk_pixbuf::Pixbuf;
use vgtk::lib::gio::{ActionExt, ApplicationFlags, Cancellable, MemoryInputStream, SimpleAction};
use vgtk::lib::glib::Bytes;
use vgtk::lib::gtk::*;
use vgtk::{gtk, gtk_if, run, Callback, Component, UpdateAction, VNode};

static DOG: &[u8] = include_bytes!("dog.png");


pub struct AboutDialog {
    dog: Pixbuf,
}

impl Default for AboutDialog {
    fn default() -> Self {
        let data_stream = MemoryInputStream::new_from_bytes(&Bytes::from_static(DOG));
        let dog = Pixbuf::new_from_stream(&data_stream, None as Option<&Cancellable>).unwrap();
        AboutDialog { dog }
    }
}

impl Component for AboutDialog {
    type Message = ();
    type Properties = ();

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Dialog::new_with_buttons(
                Some("About The Todo List"),
                None as Option<&Window>,
                DialogFlags::MODAL,
                &[("Ok", ResponseType::Ok)]
            )>
                <Box spacing=10 orientation=Orientation::Vertical>
                    <Image pixbuf=Some(self.dog.clone())/>
                    <Label markup="<big><b>A Very Nice Todo List</b></big>"/>
                    <Label markup="made with <a href=\"http://vgtk.rs/\">vgtk</a> by me"/>
                </Box>
            </Dialog>
        }
    }
}

impl AboutDialog {
    #[allow(unused_must_use)]
    fn run() {
        vgtk::run_dialog::<AboutDialog>(vgtk::current_window().as_ref());
    }
}


#[derive(Clone, Debug, Default)]
struct Radio {
    labels: &'static [&'static str],
    active: usize,
    on_changed: Callback<usize>,
}

#[derive(Clone, Debug)]
enum RadioMessage {
    Changed(usize),
}

impl Component for Radio {
    type Message = RadioMessage;
    type Properties = Self;

    fn create(props: Self) -> Self {
        props
    }

    fn change(&mut self, props: Self) -> UpdateAction<Self> {
        *self = props;
        UpdateAction::Render
    }

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            RadioMessage::Changed(index) => {
                self.on_changed.send(index);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Self> {
        gtk! {
            <Box spacing=10>
                {
                    self.labels.iter().enumerate().map(|(index, label)| gtk! {
                        <ToggleButton label={ *label }
                                      active={ index == self.active }
                                      on toggled=|_| RadioMessage::Changed(index) />
                    })
                }
            </Box>
        }
    }
}

#[derive(Clone, Debug)]
struct Task {
    text: String,
    done: bool,
}

impl Task {
    fn new<S: ToString>(text: S, done: bool) -> Self {
        Task {
            text: text.to_string(),
            done,
        }
    }

    fn label(&self) -> String {
        if self.done {
            format!(
                "<span strikethrough=\"true\" alpha=\"50%\">{}</span>",
                self.text
            )
        } else {
            self.text.clone()
        }
    }

    fn render(&self, index: usize) -> VNode<Model> {
        gtk! {
            <ListBoxRow>
                <Box>
                    <CheckButton active=self.done on toggled=|_| Message::Toggle { index } />
                    <Label label=self.label() use_markup=true />
                    <Button Box::pack_type=PackType::End
                        relief=ReliefStyle::None image="edit-delete"
                        on clicked=|_| Message::Delete { index } />
                </Box>
            </ListBoxRow>
        }
    }
}

#[derive(Clone, Debug)]
struct Model {
    tasks: Vec<Task>,
    filter: usize,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            tasks: vec![
                Task::new("Call Joe", true),
                Task::new("Call Mike", true),
                Task::new("Call Robert", false),
                Task::new("Get Robert to fix the bug", false),
            ],
            filter: 0,
        }
    }
}

impl Model {
    fn filter_task(&self, task: &Task) -> bool {
        match self.filter {
            // "All"
            0 => true,
            // "Active"
            1 => !task.done,
            // "Completed"
            2 => task.done,
            // index out of bounds
            _ => unreachable!(),
        }
    }

    fn items_left(&self) -> String {
        let left = self.tasks.iter().filter(|task| !task.done).count();
        let plural = if left == 1 { "item" } else { "items" };
        format!("{} {} left", left, plural)
    }

    fn count_completed(&self) -> usize {
        self.tasks.iter().filter(|task| task.done).count()
    }
}

#[derive(Clone, Debug)]
enum Message {
    Exit,
    About,
    Toggle { index: usize },
    Add { task: String },
    Delete { index: usize },
    Filter { filter: usize },
    Cleanup,
}

impl Component for Model {
    type Message = Message;
    type Properties = ();

    fn update(&mut self, msg: Self::Message) -> UpdateAction<Self> {
        match msg {
            Message::Exit => {
                vgtk::quit();
                UpdateAction::None
            }
            Message::About => {
                AboutDialog::run();
                UpdateAction::None
            } 
            Message::Toggle { index } => {
                self.tasks[index].done = !self.tasks[index].done;
                UpdateAction::Render
            }
            Message::Add { task } => {
                self.tasks.push(Task::new(task, false));
                UpdateAction::Render
            }
            Message::Delete { index } => {
                self.tasks.remove(index);
                UpdateAction::Render
            }
            Message::Filter { filter } =>{
                self.filter = filter;
                UpdateAction::Render
            }
            Message::Cleanup => {
                self.tasks.retain(|task| !task.done);
                UpdateAction::Render
            }
        }
    }

    fn view(&self) -> VNode<Model> {
        let main_menu = vgtk::menu()
            .section(vgtk::menu().item("About...", "app.about"))
            .section(vgtk::menu().item("Quit", "app.quit"))
            .build();
        gtk! {
            <Application::new_unwrap(Some("com.example.vgtk-todomvc"), ApplicationFlags::empty())>
                <SimpleAction::new("quit", None) Application::accels=["<Ctrl>q"].as_ref() enabled=true
                        on activate=|a, _| Message::Exit/>
                <SimpleAction::new("about", None) enabled=true on activate=|_, _| Message::About />
                <Window default_width=800 default_height=600
                    border_width=20 on destroy=|_| Message::Exit>
                    <HeaderBar title="Rust: The Todo List" show_close_button=true>
                        <MenuButton HeaderBar::pack_type=PackType::End @MenuButtonExt::direction=ArrowType::Down
                                    relief=ReliefStyle::None image="open-menu-symbolic">
                            <Menu::new_from_model(&main_menu)/>
                        </MenuButton>
                    </HeaderBar>
                    <Box orientation=Orientation::Vertical spacing=10>
                        <Entry placeholder_text="What needs to be done?"
                            on activate=|entry| {
                                entry.select_region(0, -1);
                                Message::Add {
                                    task: entry.get_text().unwrap().to_string()
                                }
                            } />
                        <ScrolledWindow Box::fill=true Box::expand=true>
                            <ListBox selection_mode=SelectionMode::None>
                                {
                                    self.tasks.iter().filter(|task| self.filter_task(task))
                                        .enumerate().map(|(index, task)| task.render(index))
                                }
                            </ListBox>
                        </ScrolledWindow>
                        <Box>
                            <Label label=self.items_left() />
                            <@Radio Box::center_widget=true active=self.filter
                                    labels=["All","Active","Completed"].as_ref()
                                    on changed=|filter| Message::Filter { filter } />
                            {
                                gtk_if!(self.count_completed() > 0 => {
                                    <Button label="Clear completed" Box::pack_type=PackType::End
                                            on clicked=|_| Message::Cleanup />
                                })
                            }
                        </Box>
                    </Box>
                </Window>
            </Application>
        }
    }
}

fn main() {
    pretty_env_logger::init();
    std::process::exit(run::<Model>());
}
