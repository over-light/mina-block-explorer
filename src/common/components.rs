use leptos::{web_sys::*, *};
use leptos_router::*;

pub enum SubSectionPosition {
    Left,
    Right,
}

#[component]
pub fn SubSectionContainer(children: Children) -> impl IntoView {
    view! {
        <div class="md:col-start-2 md:col-end-3 grid grid-cols-1 md:grid-cols-2 gap-4">
            {children()}
        </div>
    }
}

#[component]
pub fn AppSubSection(
    heading: String,
    children: Children,
    position: SubSectionPosition,
) -> impl IntoView {
    let position_class = match position {
        SubSectionPosition::Left => "md:col-start-1 md:col-end-2",
        SubSectionPosition::Right => "md:col-start-2 md:col-end-3",
    };
    view! {
        <section class=format!("{} md:rounded-lg bg-table-section",position_class)>
            <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{heading}</h1>
            {children()}
        </section>
    }
}

#[component]
pub fn AppSection(children: Children) -> impl IntoView {
    view! {
        <section class="md:col-start-2 md:col-end-3 md:rounded-lg bg-table-section mb-4">
            {children()}
        </section>
    }
}

#[component]
pub fn AppHeading(heading: String) -> impl IntoView {
    view! {
        <h1 class="md:rounded-lg h-16 pl-8 text-xl bg-table-section flex justify-start items-center">{heading}</h1>
    }
}

#[component]
pub fn Checkbox<F>(label: String, value: bool, handle_change: F) -> impl IntoView
where
    F: Fn(Event) + 'static,
{
    view! {
        <label class="text-sm grid grid-cols-[1em_auto] gap-1 font-semibold checked:text-granola-orange">
            <input
                on:change=handle_change
                prop:checked=value
                name="checkbox"
                type="checkbox"
                class="accent-granola-orange" />
            {label}
        </label>
    }
}

#[component]
pub fn URLCheckbox(label: String, url_param_key: String) -> impl IntoView {
    let query_params_map = use_query_map();
    let navigate = use_navigate();
    let location = use_location();

    let url_param_key_clone = url_param_key.clone(); // Clone url_param_key for use in the closure

    let initial_checkbox_value =
        move || query_params_map.with(|params| params.get(&url_param_key_clone).cloned());
    let (checkbox_value, set_checkbox_value) =
        create_signal(initial_checkbox_value().map_or(false, |i| i == "true"));

    create_effect(move |_| {
        let current_checkbox_value = checkbox_value.get();
        let pathname = location.pathname.get();
        let mut pm = query_params_map.get();
        pm.insert(
            url_param_key.to_string(),
            current_checkbox_value.to_string(),
        );

        logging::log!("{}", pm.to_query_string());
        logging::log!("{}", pathname);

        navigate(
            &format!("{}{}", pathname, pm.to_query_string()),
            NavigateOptions {
                resolve: true,
                replace: false,
                scroll: false,
                state: State(None),
            },
        );
    });

    view! {
        <Checkbox label=label value=checkbox_value.get() handle_change=move |ev| {
            set_checkbox_value.update(|c| {
                logging::log!("new value is {}", event_target_checked(&ev));
                *c = event_target_checked(&ev)
            })
        }/>
    }
}

#[component]
pub fn SummaryItem(
    label: String,
    value: Option<String>,
    id: String,
    #[prop(optional)] imgsrc: String,
) -> impl IntoView {
    view! {
        <div class="h-24 w-96 p-4 max-w-full grid gap-2 grid-cols-[minmax(50px,50px)_1fr] bg-white rounded-md">
            <div class="cols-span-1 row-start-1 row-end-3 bg-light-granola-orange rounded-md flex justify-center items-center">
                <img src=imgsrc width=25 alt="logo"/>
            </div>
            <div class="col-start-2 col-end-3 font-bold text-xl flex justify-start items-end" id={id.clone()}>{
                {match value {
                    Some(str_val) => view! {<span>{str_val}</span>}.into_view(),
                    None => view! { <span class="animate-pulse h-7 w-20 rounded-full bg-slate-200"/> }.into_view()
                }}
            }</div>
            <label class="row-start-2 col-start-2 col-end-3 text-sm text-slate-500 font-semibold flex justify-start items-start" for={id.clone()}>{label}</label>
        </div>
    }
}

#[component]
pub fn ErrorView<E: std::fmt::Debug>(err: E) -> impl IntoView {
    view! {
        <div class="error">
            { format!("Error: {:#?}", err) }
        </div>
    }
}

#[component]
pub fn NullView() -> impl IntoView {
    view! {}
}

#[component]
pub fn PageContainer(children: Children) -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-[10%_80%_10%] bg-secondary-background rounded-t-3xl py-6 px-2 sm:px-0 grow">
            {children()}
        </div>
    }
}
