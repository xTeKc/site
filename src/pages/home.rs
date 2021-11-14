use yew::prelude::*;

struct Item {
    id: i32, 
    name: String,
    description: String,
    image: String,
    price: f64,
}

struct State {
    items: Vec<Item>,
}

pub struct Home {
    state: State,
}

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let items: Vec<Item> = vec![
            Item {
                id: 1,
                name: "Apple".to_string(),
                description: "An apple a day keeps the doctor away".to_string(),
                image: "image".to_string(),
                price: 4.49,
            },
        ];

        Self {
            state: State {
                items,
             }
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! { <span>{"Home Sweet Home!"}</span> }
    }
}