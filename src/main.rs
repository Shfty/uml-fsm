use std::{fmt::Debug, marker::PhantomData};

trait States: Debug {
    type States;
    type Events;
    type Extended;

    fn next(&self, ev: &Self::Events, ex: &Self::Extended) -> Option<Self::States>;
}

#[derive(Debug)]
enum State<St, Ev, Ex> {
    Composite(Box<dyn States<States = St, Events = Ev, Extended = Ex>>),
    Orthogonal(Vec<Self>),
}

impl<St, Ev, Ex> State<St, Ev, Ex>
where
    St: States<States = St, Events = Ev, Extended = Ex> + 'static,
{
    fn next(&mut self, ev: &Ev, ex: &Ex) -> bool {
        match self {
            State::Composite(state) => {
                if let Some(new_state) = state.next(ev, ex) {
                    *state = Box::new(new_state);
                    true
                } else {
                    false
                }
            }
            State::Orthogonal(states) => {
                for state in states.iter_mut() {
                    if state.next(ev, ex) {
                        return true;
                    }
                }
                false
            }
        }
    }
}

#[derive(Debug)]
struct FSM<St, Ev, Ex> {
    state: State<St, Ev, Ex>,
    extended: Ex,
}

impl<St, Ev, Ex> FSM<St, Ev, Ex>
where
    St: States<States = St, Events = Ev, Extended = Ex> + 'static,
{
    fn event(&mut self, e: &Ev) {
        self.state.next(e, &self.extended);
    }
}

#[derive(Debug)]
enum CharacterStates {
    Idle,
    Walk,
    Run,
    Jump,
    Fall,
}

impl States for CharacterStates {
    type States = CharacterStates;
    type Events = CharacterEvents;
    type Extended = CharacterExtended;

    fn next(&self, ev: &Self::Events, _ex: &Self::Extended) -> Option<Self::States> {
        match self {
            CharacterStates::Idle => match ev {
                CharacterEvents::WishDirection => Some(CharacterStates::Walk),
                CharacterEvents::Jump => Some(CharacterStates::Jump),
            },
            CharacterStates::Walk => match ev {
                CharacterEvents::WishDirection => Some(CharacterStates::Run),
                CharacterEvents::Jump => Some(CharacterStates::Jump),
            },
            CharacterStates::Run => match ev {
                CharacterEvents::Jump => Some(CharacterStates::Jump),
                _ => None
            }
            _ => None,
        }
    }
}

#[derive(Debug)]
enum CharacterEvents {
    WishDirection,
    Jump,
}

#[derive(Debug)]
struct CharacterExtended {}

fn main() {
    let mut fsm = FSM {
        state: State::Composite(Box::new(CharacterStates::Idle)),
        extended: CharacterExtended {},
    };

    println!("{:#?}", fsm);

    fsm.event(&CharacterEvents::WishDirection);

    println!("{:#?}", fsm);

    fsm.event(&CharacterEvents::WishDirection);

    println!("{:#?}", fsm);

    fsm.event(&CharacterEvents::Jump);

    println!("{:#?}", fsm);
}
