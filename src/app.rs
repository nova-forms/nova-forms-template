use leptos::*;
use leptos_i18n::*;
use leptos_meta::*;
use nova_forms::*;
use serde::{Deserialize, Serialize};

// This generates the `BaseContextProvider` as well as the `RenderContextProvider` component at compile-time to initialize all the necessary context.
init_nova_forms!();

// Define the app that contains the form.
#[component]
pub fn App() -> impl IntoView {
    view! {
        // We differentiate the base URL between CSR only mode and normal mode for this form.
        // This is only necessary for the demo, in a real application you would only use one mode.
        <BaseContextProvider>
            <Wrapper />
        </BaseContextProvider>
    }
}

// Define the app that contains the form.
#[component]
pub fn Wrapper() -> impl IntoView {
    let i18n = use_i18n();
    
    view! {
        <NovaFormWrapper title=t!(i18n, nova_forms) subtitle=t!(i18n, demo_form) logo="logo.svg">
            <DemoForm />
        </NovaFormWrapper>  
    }
}


// Define the form data structure.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DemoForm {
    test: String,
}

// Defines how to render the form.
#[component]
pub fn DemoForm() -> impl IntoView {
    // Get the locale context.
    let i18n = use_i18n();
    // Define the submit server action.
    let submit_action = create_server_action::<OnSubmit>();

    view! {
        // Sets the document title.
        <Title text=t!(i18n, demo_form) />

        // Defines how to render the form itself.
        <NovaForm
            on_submit=submit_action
            bind="form_data"
            bind_meta_data="meta_data"
            i18n=i18n
        > 
            <Input<String> bind="test" label="Test Input" />
        </NovaForm>
        
        <Preview/>

        <Toolbar>
            <ToolbarLocaleSelect i18n=i18n />
            <ToolbarPreviewButton />
            <ToolbarSubmitButton />
        </Toolbar>
    }
}

// Defines the server action for form submission.
#[server]
async fn on_submit(form_data: DemoForm, meta_data: MetaData) -> Result<(), ServerFnError> {
    println!("form data received: {:#?}", form_data);
    println!("meta data received: {:#?}", meta_data);

    let pdf_gen = expect_context::<PdfGen>();
    let output_path = pdf_gen
        .render_form(move || view! {
            <RenderContextProvider form_data=form_data meta_data=meta_data>
                <DemoForm />
            </RenderContextProvider>
        })
        .await?;

    println!("form successfully rendered: {:?}", output_path);

    Ok(())
}
