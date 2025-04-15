use leptos::prelude::*;

#[component]
pub fn Iteration() -> impl IntoView {
    view! {
        <div>
            <StaticList />
            <DynamicList />
        </div>
    }
}

#[component]
fn StaticList() -> impl IntoView {
    let counters = (1..=5).map(|idx| RwSignal::new(idx));
    // view! {
    //     // this will just render "012"
    //     <p>{values.clone()}</p>
    //     // or we can wrap them in <li>; collect_view is the alternative to collect turbofish
    //     // <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect::<Vec<_>>()}</ul>
    //     <ul>{values.into_iter().map(|n| view! { <li>{n}</li> }).collect_view()}</ul>
    // }

    let counter_buttons = counters.map(|count| {
        view! {
            <li>
                <button on:click=move |_| *count.write() += 1>{count}</button>
            </li>
        }
    })
    .collect_view();

    view! { <ul>{counter_buttons}</ul> }
}

#[component]
fn DynamicList(
    /// The number of counters to begin with.
    #[prop(default=5)]
    initial_length: usize,
) -> impl IntoView {
    // This dynamic list will use the <For/> component.
    // <For/> is a keyed list. This means that each row
    // has a defined key. If the key does not change, the row
    // will not be re-rendered. When the list changes, only
    // the minimum number of changes will be made to the DOM.

    // `next_counter_id` will let us generate unique IDs
    // we do this by simply incrementing the ID by one
    // each time we create a counter
    let mut next_counter_id = initial_length;

    // we generate an initial list as in <StaticList/>
    // but this time we include the ID along with the signal
    // see NOTE in add_counter below re: ArcRwSignal
    let initial_counters = (0..initial_length)
        .map(|id| (id, ArcRwSignal::new(id + 1)))
        .collect::<Vec<_>>();

    // now we store that initial list in a signal
    // this way, we'll be able to modify the list over time,
    // adding and removing counters, and it will change reactively
    let (counters, set_counters) = signal(initial_counters);

    let add_counter = move |_| {
        // create a signal for the new counter
        // we use ArcRwSignal here, instead of RwSignal 
        // ArcRwSignal is a reference-counted type, rather than the arena-allocated 
        // signal types we've been using so far.
        // When we're creating a collection of signals like this, using ArcRwSignal 
        // allows each signal to be deallocated when its row is removed.
        let sig = ArcRwSignal::new(next_counter_id + 1);
        // add this counter to the list of counters
        set_counters.update(move |counters| {
            // since `.update()` gives us `&mut T`
            // we can just use normal Vec methods like `push`
            counters.push((next_counter_id, sig))
        });
        // increment the ID so it's always unique
        next_counter_id += 1;
    };

    view! {
        <div>
            <button on:click=add_counter>"Add Counter"</button>
            <ul>
                // The <For/> component is central here
                // This allows for efficient, key list rendering
                <For
                    // `each` takes any function that returns an iterator
                    // this should usually be a signal or derived signal
                    // if it's not reactive, just render a Vec<_> instead of <For/>
                    each=move || counters.get()
                    // the key should be unique and stable for each row
                    // using an index is usually a bad idea, unless your list
                    // can only grow, because moving items around inside the list
                    // means their indices will change and they will all rerender
                    key=|counter| counter.0
                    // `children` receives each item from your `each` iterator
                    // and returns a view
                    children=move |(id, count)| {
                        let count = RwSignal::from(count);
                        // we can convert our ArcRwSignal to a Copy-able RwSignal
                        // for nicer DX when moving it into the view
                        view! {
                            <li>
                                <button on:click=move |_| *count.write() += 1>{count}</button>
                                <button on:click=move |_| {
                                    set_counters
                                        .write()
                                        .retain(|(counter_id, _)| { counter_id != &id });
                                }>"Remove"</button>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

// #[component]
// fn ComplexData() -> impl IntoView {
//     // start with a set of three rows
//     let (data, set_data) = signal(vec![
//         DatabaseEntry {
//             key: "foo".to_string(),
//             value: RwSignal::new(10),
//         },
//         DatabaseEntry {
//             key: "bar".to_string(),
//             value: RwSignal::new(20),
//         },
//         DatabaseEntry {
//             key: "baz".to_string(),
//             value: RwSignal::new(15),
//         },
//     ]);
//     view! {
//         // when we click, update each row,
//         // doubling its value
//         <button on:click=move |_| {
//             for row in &*data.read() {
//                 row.value.update(|value| *value *= 2);
//             }
//             leptos::logging::log!("{:?}", data.get());
//         }>"Update Values"</button>
//         // iterate over the rows and display each value
//         // <For
//         // each=move || data.get()
//         // key=|state| state.key.clone()
//         // let(child)
//         // >
//         // <p>{child.value}</p>
//         // </For>
//         <For
//             each=move || data.get().into_iter().enumerate()
//             key=|(_, state)| state.key.clone()
//             children=move |(index, _)| {
//                 let value = Memo::new(move |_| {
//                     data.with(|data| data.get(index).map(|d| d.value).unwrap_or(0))
//                 });
//                 view! { <p>{value}</p> }
//             }
//         />
//     }
// }