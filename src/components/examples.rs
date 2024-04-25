use leptos::*;

use crate::examples;

#[component]
pub fn SelectExampleProgram<F>(
    update_program_str: F,
    set_name: WriteSignal<Option<String>>,
) -> impl IntoView
where
    F: Fn(String) + 'static,
{
    let select_example_program = move |name: String| {
        if let Some(new_human) = examples::get_program_str(&name) {
            update_program_str(new_human.to_string());
            set_name.set(Some(name));
        }
    };

    // Select default example upon startup
    // The name must exist inside examples::EXAMPLES
    select_example_program("BIP 340 Schnorr".to_string());

    view! {
        <select
            on:input=move |event| select_example_program(event_target_value(&event))
        >
            <option value="" disabled selected>Example programs</option>
            {
                examples::get_names()
                    .map(|name| view! { <option value={name}>{name}</option>})
                    .collect::<Vec<_>>()
            }
        </select>
    }
}

#[component]
pub fn ExampleProgramDescription(name: ReadSignal<Option<String>>) -> impl IntoView {
    view! {
        <div>
        {
            move || name.get().map(|n| view! {
                <h2>{name}</h2>
                <p>
                {
                    move || examples::get_description(n.as_str()).map(|d| d.to_string())
                }
                </p>
            })
        }
         </div>
    }
}
