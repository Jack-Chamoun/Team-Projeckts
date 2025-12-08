use leptos::prelude::*;
use crate::view::icons::Search;

#[component]
pub fn SearchBar(city: RwSignal<String>) -> impl IntoView {

    let input = RwSignal::new(String::new());

    view! {
        <div class="cell is-col-start-5 is-row-start-1">
            <div class="control glass has-icons-left has-icons-right">
                <input 
                    class="input" 
                    type="text" 
                    placeholder="Search for a city" 
                    on:input=move |e| {
                        let value = event_target_value(&e);
                        input.set(value);
                    }
                    on:keydown=move |e| {
                        if e.key() == "Enter" {
                            city.set(input.get()); 
                        }
                    }
                />
                <Search />
            </div>
        </div>
    }
}