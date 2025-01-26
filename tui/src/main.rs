use iocraft::prelude::*;
use crux_core::{Core};
use shared::app::{Counter, Effect, Event, ViewModel};

struct CoreState {
    core: Core<Counter>,
    view: ViewModel,
}

impl CoreState {
    fn new() -> Self {
        let core = Core::new();
        let view = core.view();
        Self { core, view }
    }

    fn update(&mut self, event: Event) {
        let effects = self.core.process_event(event);

        for request in effects {
            self.process_effect(request);
        }
    }

    fn process_effect(&mut self, request: Effect) {
        match request {
            Effect::Render(_) => {
                self.view = self.core.view();
            }
        }
    }
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let (width, height) = hooks.use_terminal_size();
    let mut system = hooks.use_context_mut::<SystemContext>();
    let mut should_exit = hooks.use_state(|| false);
    let mut core = hooks.use_state(|| CoreState::new());

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Char('+') => core.write().update(Event::Increment),
                KeyCode::Char('-') => core.write().update(Event::Decrement),
                KeyCode::Char('q') => should_exit.set(true),
                _ => {}
            }
        }
        _ => {}
    });

    if should_exit.get() {
        system.exit();
    }

    element! {
        View(
            width: width - 1,
            height: height,
            flex_direction: FlexDirection::Column,
        ) {
            View(
            width: 100pct,
            height: 100pct,
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            gap: 1,
        ) {
            Text(content: core.read().view.clone().count)
        }
            View(
                height: 1,
                padding_left: 1,
            ) {
                Text(content: "[Q] Quit [+] Increment [-] Decrement")
            }
        }
    }
}

fn main() {
    smol::block_on(element!(App).render_loop()).unwrap();
}
