//! Leptos Lattice Demo Application

use std::{collections::HashMap, sync::Arc};

use leptos::{prelude::*, task};

use crate::products::ProductEstimate;

mod basket;
mod products;
mod promotions;

const PRODUCTS_FIXTURE_YAML: &str = include_str!("../../../fixtures/products/demo.yml");
const PROMOTIONS_FIXTURE_YAML: &str = include_str!("../../../fixtures/promotions/demo.yml");
const ESTIMATE_DEBOUNCE_MS: i32 = 220;
const SPINNER_DELAY_MS: i32 = 100;
const ESTIMATE_YIELD_MS: i32 = 0;

/// Parsed application fixtures/state used by the UI.
#[derive(Debug)]
struct AppData {
    /// Products shown on the left panel.
    products: Arc<Vec<products::ProductListItem>>,

    /// Basket/solver data used by the basket panel.
    basket_solver_data: Arc<basket::BasketSolverData>,
}

impl AppData {
    fn load() -> Result<Self, String> {
        let loaded_products = products::load_products(PRODUCTS_FIXTURE_YAML)?;
        let loaded_promotions = promotions::load_promotions(PROMOTIONS_FIXTURE_YAML)?;

        Ok(Self {
            products: Arc::new(loaded_products.products),
            basket_solver_data: Arc::new(basket::BasketSolverData {
                product_meta_map: loaded_products.product_meta_map,
                product_key_by_fixture_key: loaded_products.product_key_by_fixture_key,
                graph: loaded_promotions.graph,
                promotion_names: loaded_promotions.promotion_names,
                currency: loaded_products.currency,
            }),
        })
    }
}

/// Main demo app shell.
#[component]
fn App() -> impl IntoView {
    match AppData::load() {
        Ok(app_data) => {
            let app_data = Arc::new(app_data);
            let cart_items = RwSignal::new(Vec::<String>::new());

            let solve_time_text = RwSignal::new(String::new());

            let live_message = RwSignal::new((0_u64, String::new()));
            let action_message = RwSignal::new(None::<String>);

            let estimates = RwSignal::new(HashMap::<String, ProductEstimate>::new());
            let estimating = RwSignal::new(false);
            let show_estimate_spinner = RwSignal::new(false);
            let estimate_generation = RwSignal::new(0_u64);

            Effect::new({
                let solver_data = Arc::clone(&app_data.basket_solver_data);
                let products = Arc::clone(&app_data.products);

                move |_| {
                    let cart_snapshot = cart_items.get();

                    estimate_generation.update(|generation| {
                        *generation = generation.saturating_add(1);
                    });
                    show_estimate_spinner.set(false);

                    let run_id = estimate_generation.get_untracked();

                    let solver_data = Arc::clone(&solver_data);
                    let products = Arc::clone(&products);

                    task::spawn_local(async move {
                        wait_for_timeout(ESTIMATE_DEBOUNCE_MS).await;

                        if estimate_generation.get_untracked() != run_id {
                            return;
                        }

                        estimating.set(true);

                        spawn_spinner_reveal(
                            run_id,
                            estimate_generation,
                            estimating,
                            show_estimate_spinner,
                        );

                        let Ok(base_total_minor) =
                            basket::solve_total_minor(&solver_data, &cart_snapshot)
                        else {
                            if estimate_generation.get_untracked() == run_id {
                                estimates.set(HashMap::new());

                                finish_estimation(estimating, show_estimate_spinner);
                            }

                            return;
                        };

                        for product in products.iter() {
                            if estimate_generation.get_untracked() != run_id {
                                return;
                            }

                            let mut projected_cart = cart_snapshot.clone();
                            projected_cart.push(product.fixture_key.clone());

                            if let Ok(projected_total_minor) =
                                basket::solve_total_minor(&solver_data, &projected_cart)
                            {
                                let marginal_minor = projected_total_minor - base_total_minor;
                                let fixture_key = product.fixture_key.clone();

                                estimates.update(|map| {
                                    map.insert(
                                        fixture_key,
                                        ProductEstimate {
                                            marginal_minor,
                                            savings_minor: product.price_minor - marginal_minor,
                                        },
                                    );
                                });
                            }

                            wait_for_timeout(ESTIMATE_YIELD_MS).await;
                        }

                        if estimate_generation.get_untracked() == run_id {
                            finish_estimation(estimating, show_estimate_spinner);
                        }
                    });
                }
            });

            view! {
                <main class="min-h-screen bg-slate-50 px-4 py-6 text-slate-900">
                    <p class="sr-only" role="status" aria-live="polite" aria-atomic="true">
                        {move || live_message.get().1}
                    </p>
                    <div class="mx-auto mb-6 max-w-5xl">
                        <h1 class="text-2xl font-semibold tracking-tight">"Lattice Demo"</h1>
                    </div>
                    <div class="mx-auto grid max-w-5xl grid-cols-1 gap-6 md:grid-cols-2">
                        <products::ProductsPanel
                            products=Arc::clone(&app_data.products)
                            cart_items=cart_items
                            action_message=action_message
                            estimates=estimates
                            show_spinner=show_estimate_spinner
                        />
                        <basket::BasketPanel
                            solver_data=Arc::clone(&app_data.basket_solver_data)
                            cart_items=cart_items
                            solve_time_text=solve_time_text
                            live_message=live_message
                            action_message=action_message
                        />
                    </div>
                </main>
            }
            .into_any()
        }
        Err(error_message) => view! {
            <main class="min-h-screen bg-slate-50 px-4 py-6 text-slate-900">
                <div class="mx-auto mb-6 max-w-5xl">
                    <h1 class="text-2xl font-semibold tracking-tight">"Lattice Demo"</h1>
                </div>
                <div class="mx-auto max-w-3xl rounded-lg border border-red-200 bg-red-50 p-4">
                    <p class="text-sm text-red-700">{error_message}</p>
                </div>
            </main>
        }
        .into_any(),
    }
}

/// Main server function
fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(App);
}

fn announce(live_message: RwSignal<(u64, String)>, message: String) {
    live_message.update(|(id, text)| {
        *id = id.saturating_add(1);
        *text = message;
    });
}

fn spawn_spinner_reveal(
    run_id: u64,
    estimate_generation: RwSignal<u64>,
    estimating: RwSignal<bool>,
    show_estimate_spinner: RwSignal<bool>,
) {
    task::spawn_local(async move {
        wait_for_timeout(SPINNER_DELAY_MS).await;

        if estimate_generation.get_untracked() == run_id && estimating.get_untracked() {
            show_estimate_spinner.set(true);
        }
    });
}

fn finish_estimation(estimating: RwSignal<bool>, show_estimate_spinner: RwSignal<bool>) {
    estimating.set(false);
    show_estimate_spinner.set(false);
}

#[cfg(target_arch = "wasm32")]
async fn wait_for_timeout(delay_ms: i32) {
    use js_sys::{Function, Promise};
    use wasm_bindgen::{JsCast, JsValue, closure::Closure};
    use wasm_bindgen_futures::JsFuture;

    let mut executor = move |resolve: Function, _reject: Function| {
        let Some(window) = web_sys::window() else {
            let _ = resolve.call0(&JsValue::NULL);
            return;
        };

        let callback = Closure::once_into_js(move || {
            let _ = resolve.call0(&JsValue::NULL);
        });

        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.unchecked_ref(),
            delay_ms,
        );
    };

    let promise = Promise::new(&mut executor);
    let _ = JsFuture::from(promise).await;
}

#[cfg(not(target_arch = "wasm32"))]
async fn wait_for_timeout(_delay_ms: i32) {
    task::tick().await;
}
