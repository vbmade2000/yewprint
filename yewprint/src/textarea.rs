use yew::prelude::*;
use crate::Intent;


pub struct TextArea {
    props: TextAreaProps,
}


#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    fill: bool,
    #[prop_or_default]
    grow_vertically: bool,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub small: bool,
    #[prop_or_default]
    pub onchange: Option<Callback<ChangeData>>,
}

impl Component for TextArea {
    type Message = ();
    type Properties = TextAreaProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        TextArea { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
             let classes=classes!(
                    "bp3-text-area",
                    self.props.fill.then(|| "bp3-fill"),
                    self.props.large.then(|| "bp3-large"),
                    self.props.small.then(|| "bp3-small"),
                    self.props.class.clone(),
                );
        html!{
           <textarea
            classname=classes
            onchange=self.props.onchange.clone()
           /> 
        }

    }
}
