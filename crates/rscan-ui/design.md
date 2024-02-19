# Design Document

This will be a small ui library that creates a component like system for `ratatui` inspired by svelte.

## Components

Components are the building blocks of the UI.

### Pure Local State Example

```rust
#[component]
pub fn Counter() -> impl View {
    let (count, set_count) = use_state(|| 0);

    view! {
        Paragraph {
            text: format!("Counter: {}", count),
            on_key: move |key| {
                match key {
                    KeyCode::Char('j') => set_count(count + 1),
                    KeyCode::Char('k') => set_count(count - 1),
                    _ => {}
                }
            }
        },
        Paragraph {
          text: "Styled Paragraph",
          style: {
              color: Color::Red,
              border: Border::new(BorderType::Rounded, Color::Green, 1)
          }
      }
    }
}
```

### Store Example

```rust
pub struct CounterStore {
    count: Signal<i32>,
}

impl CounterStore {
    pub fn new() -> Self {
        Self {
            count: Signal::new(0),
        }
    }
    pub fn increment(&self) {
      self.count.update(|c| c + 1);
    }
    pub fn decrement(&self) {
      self.count.update(|c| c - 1);
    }
}

#[component]
pub fn Counter(store: &CounterStore) -> impl View {
    view! {
        Paragraph {
            text: format!("Counter: {}", store.count.get()),
            on_key: move |key| {
                match key {
                    KeyCode::Char('j') => store.increment(),
                    KeyCode::Char('k') => store.decrement(),
                    _ => {}
                }
            }
        }
    }
}
```
