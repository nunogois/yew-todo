use std::borrow::Borrow;

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| vec![]);
    let input = use_state(|| "".to_string());

    let onclick = {
        let input = input.clone();
        let todos = todos.clone();
        Callback::from(move |_| {
            todos.set(
                todos
                    .borrow()
                    .to_vec()
                    .into_iter()
                    .chain(vec![input.borrow().to_string()].into_iter())
                    .collect(),
            );
            input.set("".to_string());
        })
    };

    // let removeTODO = {
    //     let todos = todos.clone();
    //     Callback::from(move |todo: &String| {
    //         todos.set(
    //             todos
    //                 .borrow()
    //                 .to_vec()
    //                 .into_iter()
    //                 .filter(|t| t != todo)
    //                 .collect(),
    //         );
    //     })
    // };

    let oninput = {
        let input = input.clone();
        move |e: InputEvent| {
            let elm: HtmlInputElement = e.target_unchecked_into();
            input.set(elm.value());
        }
    };

    html! {
        <main>
          <a href="https://www.github.com/nunogois/yew-todo"><h1 class="text-2xl mb-4 text-emerald-500">{ "yew-todo" }</h1></a>
          <input type="text" class="border outline-none p-2 border-emerald-500 rounded-tl-md rounded-bl-md bg-transparent" value={input.clone().to_string()} {oninput} />
          <button class="border border-emerald-500 p-2 bg-emerald-500 rounded-tr-md rounded-br-md text-black" {onclick}>{ "+" }</button>
          <ul>
            { for todos.iter().map(|todo| html! { <li class="mt-2 text-emerald-500">{ todo }</li> }) }
          </ul>
        </main>
    }
}
