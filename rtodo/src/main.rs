extern crate yew;

use yew::prelude::*;

struct Model {
    input: String,
    edit_input: String,
    todos: Vec<Todo>,
}

#[derive(Debug, Clone)]
struct Todo {
    text: String,
    edit: bool,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    Edit(usize),
    UpdateEdit(String),
    Toggle(usize),
    RemoveAll,
    Nothing,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Model {
            todos: vec![],
            input: "".to_string(),
            edit_input: "".to_string(),
        }
    }
    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                let t = Todo {
                    text: self.input.clone(),
                    edit: false,
                };
                self.todos.push(t);
                self.input = "".to_string();
            }
            Msg::Update(s) => {
                self.input = s;
            }
            Msg::Remove(i) => {
                self.todos.remove(i);
            }
            Msg::RemoveAll => {
                self.todos = vec![];
            }
            Msg::UpdateEdit(s) => {
                self.edit_input = s;
            }
            Msg::Edit(i) => {
                let val = Todo {
                    text: self.edit_input.clone(),
                    edit: false,
                };
                self.todos.remove(i);
                self.todos.push(val);
            }
            Msg::Toggle(i) => {
                let empty_todo = &mut Todo {
                    text: String::from(""),
                    edit: false,
                };
                let todo = self.todos.get_mut(i).unwrap_or(empty_todo);
                todo.edit = !todo.edit;
            }
            Msg::Nothing => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_todo_edit = |(i, todo): (usize, &Todo)| {
            if todo.edit == true {
                html! {
                        <label>
                        <input
                            type="text"
                            value={todo.text.clone()}
                            oninput={ctx.link().callback(|e: InputEvent| Msg::UpdateEdit(e.data().expect("something wrong to get input data")))}
                            onkeypress={move |e: KeyboardEvent| {
                                if e.key() == "Enter" {Msg::Edit(i)} else {Msg::Nothing};
                            }}
                        />
                </label>
                    }
            } else {
                html! {
                        <label ondblclick={ctx.link().callback(move |_| Msg::Toggle(i))}>
                            {format!("{} ", &todo.text)}
                        </label>
                }
            }
        };
        let view_todo = |(i, todo): (usize, &Todo)| {
            html! {
                <div>
                    <li>
                        { view_todo_edit((i, &todo))}
                    </li>
                    <button onclick = {ctx.link().callback(move |_| Msg::Remove(i))}>{"X"}</button>
                </div>
            }
        };
        html! {
            <div>
                <div>
                    <h1>{"Todo App"}</h1>
                    <input
                        placeholder="what do you want to do?"
                        value={self.input.clone()}
                        oninput={ctx.link().callback(|e: InputEvent| Msg::Update(e.data().expect("something wrong with getting input data")))}
                        onkeypress={move |e: KeyboardEvent| {
                            if e.key() == "Enter" {Msg::Add} else {Msg::Nothing};
                        }}
                    />
                    <p>{&self.input}</p>
                </div>
                <div>
                    <button
                        onclick = {ctx.link().callback(move |_| Msg::RemoveAll)}
                    >{"Delete all todos"}</button>
                </div>
                <div>
                    <ul>
                        {for self.todos.iter().enumerate().map(view_todo)}
                    </ul>
                </div>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}
