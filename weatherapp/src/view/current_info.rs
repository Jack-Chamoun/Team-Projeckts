use leptos::prelude::*;
use crate::{view::icons::Info, CurrentInformation};
use crate::view::icons::*;

#[component]
pub fn CurrentInfo(current_info: RwSignal<CurrentInformation>) -> impl IntoView {

    let wind = move || current_info.read().wind;
    let humidity = move || current_info.read().humidity;
    let condition = move || current_info.read().condition.clone();
    let visibility = move || current_info.read().visibility / 1000;  

    view! {
        <div class="cell is-col-span-2 is-col-start-4 is-row-start-2 is-row-span-4">
            <div class="box glass">
                <article class="media">
                    <div class="icon-text is-gap-1">
                        <span class="icon">
                            <Info />
                        </span>
                        <p>INFORMATION</p>
                    </div>
                </article>

                <article class="media is-align-items-center">
                    <div class="media-content">
                        <div class="icon-text is-gap-1">
                            <span class="icon">
                                <Wind />
                            </span>
                            <p>Wind</p>
                        </div>
                    </div>
                    <div class="media-right has-text-weight-semibold">
                        <p>{move ||format!("{:.1} km/h", wind() as f64 * 3.6)}</p>
                    </div>
                </article>

                <article class="media is-align-items-center">
                    <div class="media-content">
                        <div class="icon-text is-gap-1">
                            <span class="icon">
                                <Humidity />
                            </span>
                            <p>Humidity</p>
                        </div>
                    </div>
                    <div class="media-right has-text-weight-semibold">
                        <p> {move || humidity}"%"</p>
                    </div>
                </article>

                <article class="media is-align-items-center">
                    <div class="media-content">
                        <div class="icon-text is-gap-1">
                            <span class="icon">
                                <Weather />
                            </span>
                            <p>Condition</p>
                        </div>
                    </div>
                    <div class="media-right has-text-weight-semibold is-capitalized">
                        <p>{move ||condition}</p>
                    </div>
                </article>

                <article class="media is-align-items-center">
                    <div class="media-content">
                        <div class="icon-text is-gap-1">
                            <span class="icon">
                                <Visibility />
                            </span>
                            <p>Visibility</p>
                        </div>
                    </div>
                    <div class="media-right has-text-weight-semibold">
                        <p>{move || format!("{} km", visibility())}</p>
                    </div>
                </article>
            </div>
        </div>
    }
}