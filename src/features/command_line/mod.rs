use leptos::*;

#[derive(Clone, Debug)]
pub struct CommandLineState {
    hidden: bool,
}

#[component]
pub fn CommandLineModalView(cx: Scope) -> impl IntoView {
    view! { cx, <div>"modal"</div> }
}

#[component]
pub fn CommandLineModal(cx: Scope) -> impl IntoView {
    let state =
        use_context::<RwSignal<CommandLineState>>(cx).expect("command line state must be provided");

    let (hidden, _) = create_slice(cx, state, |state| state.hidden, |state, n| state.hidden = n);

    view! { cx,
        {move || {
            if !hidden.get() {
                view! { cx, <CommandLineModalView/> }
            } else {
                view! { cx, <div></div> }.into_view(cx)
            }
        }}
    }
}

#[component]
pub fn CommandLine(cx: Scope, children: Children) -> impl IntoView {
    let state = create_rw_signal(cx, CommandLineState { hidden: true });
    provide_context(cx, state);
    let (hidden, set_hidden) =
        create_slice(cx, state, |state| state.hidden, |state, n| state.hidden = n);

    leptos_dom::helpers::window_event_listener(ev::keypress, move |event| {
        if event.ctrl_key() {
            match event.code().as_str() {
                "KeyK" => {
                    //set_hidden(!hidden.get());
                    log!("toggle command")
                }
                _ => {}
            }
        }
    });

    view! { cx,
        <div>
            <div>{children(cx)}</div>
            <CommandLineModal/>
        </div>
    }
}
