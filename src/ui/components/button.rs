use leptos::prelude::*;

#[allow(dead_code)]
pub(crate) enum ButtonVariant {
    Primary,
    Secondary,
    Accent,
}

impl ButtonVariant {
    pub(crate) fn get_style(&self) -> String {
        match self {
            ButtonVariant::Primary => "bg-primary text-primary-foreground hover:bg-primary/90",
            ButtonVariant::Secondary => {
                "bg-secondary text-secondary-foreground hover:bg-secondary/90"
            }
            ButtonVariant::Accent => "bg-accent text-accent-foreground hover:bg-accent/90",
        }
        .to_string()
    }
}

#[component]
pub(crate) fn Button(
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] on_click: Option<String>,
    children: Children,
) -> impl IntoView {
    let default_style = format!(
        "{variant_style} rounded-lg border-2 border-border px-4 py-1 hover:cursor-pointer",
        variant_style = variant.get_style()
    );
    let button_style = match class {
        Some(extra) => format!("{} {}", default_style, extra),
        None => default_style,
    };

    view! {
      <button
        onclick=on_click
        class=button_style
        >{children()}</button>
    }
}
