use leptos::prelude::*;

use crate::view::icons::Temperature;

// ESP32-Project
#[component]
pub fn Roomtemperature(roomtemp: RwSignal<Option<f64>>) -> impl IntoView {

    let current_temp = move || roomtemp.get();

    let display_room_temp = move || match current_temp() {
        Some(temp) => format!("{:.0}°C", temp),
        None => String::from("No data")
    };

    view! {
        <div class="cell is-col-start-3 is-row-span-2 is-row-start-2">
            <div class="box glass" style="width: 200px; height: 160px; position: relative;">
                <p class="is-size-4"> Room </p>
                <div class="icon is-large is-white" style="position: absolute; top: 15px; right: 15px;">
                    <Temperature />
                </div>
              <div style="position: absolute; bottom: 15px; left: 15px;">
                  <p class="title is-3 mb-2 has-text-white"> {move || display_room_temp} </p>
              </div>
            </div>
        </div>
    }
}