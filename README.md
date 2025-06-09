# dioxus_learning
Playground for learning the Dioxus Framework

## Routing and Shared State

This example uses `dioxus-router` to switch between a home page and an about
page. A `SharedState` context provides a global counter so both pages can read
and update the same value.

### Navigation

- **Home** – default route `/` with a button that increments the counter.
- **About** – accessible at `/about`; shows the current counter value.

Use the `Link` components on each page to move between them.

### Modifying State

The counter is stored in a `Signal<i32>` inside `SharedState`. Pressing the
button on the home page updates the signal, which automatically updates the
displayed count on both pages.
