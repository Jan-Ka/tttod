use crate::icon_names::IconName;
use yew::prelude::*;

pub struct Icon {
    props: Props,
}

#[derive(Debug, Clone, Properties)]
pub struct Props {
    pub name: IconName,
    #[prop_or_default]
    pub classes: Option<String>,
}

impl Component for Icon {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
    fn view(&self) -> Html {
        let icon_name = serde_json::to_string(&self.props.name).unwrap();
        let icon_name = icon_name.strip_prefix('"').unwrap();
        let icon_name = icon_name.strip_suffix('"').unwrap();
        html! {
            <i class={ format!("fas fa-{} {}", icon_name, self.props.classes.as_deref().unwrap_or_default()) }></i>
        }
    }
}
