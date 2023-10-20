use leptos::*;

#[derive(Clone, Debug)]
pub struct CommandLineState {
    hidden: bool,
}

#[component]
pub fn CommandLineModalView() -> impl IntoView {
    view! { <div>"modal"</div> }
}

#[component]
pub fn CommandLineModal() -> impl IntoView {
    let state =
        use_context::<RwSignal<CommandLineState>>().expect("command line state must be provided");

    let (hidden, _) = create_slice(state, |state| state.hidden, |state, n| state.hidden = n);

    view! {
        {move || {
            if !hidden.get() {
                view! { <CommandLineModalView/> }
            } else {
                view! {  }.into_view()
            }
        }}
    }
}

#[component]
pub fn CommandLine(children: Children) -> impl IntoView {
    let state = create_rw_signal(CommandLineState { hidden: true });
    provide_context(state);
    let (hidden, set_hidden) =
        create_slice(state, |state| state.hidden, |state, n| state.hidden = n);

    leptos_dom::helpers::window_event_listener(ev::keypress, move |event| {
        if event.ctrl_key() {
            match event.code().as_str() {
                "KeyK" => {
                    set_hidden.set(!hidden.get());
                    //log!("toggle command")
                }
                _ => {}
            }
        }
    });

    view! {
        <div>
            <div>{children()}</div>
            <CommandLineModal/>
        </div>
    }
}
