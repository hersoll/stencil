use crate::Error;
use crate::api::generate_pdf;
use crate::frontend::Sets;
use crate::frontend::i18n_lookup;
use crate::shared::{DocumentOptions, SendableProblemSetData};
use dioxus::document::eval;
use dioxus::prelude::server_fn::error::ServerFnErrorErr;
use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Uint8Array;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url, window};

fn convert_sets(sets: Sets) -> Vec<SendableProblemSetData> {
    let mut converted: Vec<SendableProblemSetData> = Vec::new();
    for set in sets {
        converted.push(SendableProblemSetData::from(set()));
    }
    converted
}

#[component]
pub fn PDFButtons(sets: Signal<Sets>, options: Signal<DocumentOptions>) -> Element {
    let mut generating_pdf = use_signal(|| false);
    let mut pdf_url = use_signal(|| None::<String>);
    let mut generation_error = use_signal(|| None::<String>);

    use_effect(move || {
        if let Some(_) = pdf_url() {
            spawn(async move {
                eval(
                    "document.getElementById('pdf_buttons').scrollIntoView({behavior: 'smooth', block: 'start'});",
                );
            });
        }
    });
    #[cfg(target_arch = "wasm32")]
    let view_pdf = move |_| {
        async move {
            generating_pdf.set(true);
            generation_error.set(None);
            let bytes = generate_pdf(convert_sets(sets()), options())
                .await
                .map_err(ServerFnErrorErr::from)?;
            // Potential JS Errors in here
            let uint8_array = Uint8Array::from(bytes.as_slice());
            let array = js_sys::Array::new();
            array.push(&uint8_array.buffer());
            let blob_options = BlobPropertyBag::new();
            blob_options.set_type("application/pdf");
            let blob = Blob::new_with_buffer_source_sequence_and_options(&array, &blob_options)
                .map_err(|_| Error::PDFLoadingFailed)?;
            let url =
                Url::create_object_url_with_blob(&blob).map_err(|_| Error::PDFLoadingFailed)?;
            pdf_url.set(Some(url));
            generating_pdf.set(false);
            Ok(())
        }
    };

    #[cfg(target_arch = "wasm32")]
    let download_pdf = move |_| {
        if let Some(url) = pdf_url() {
            let document = window()
                .ok_or(Error::WebAPIFailed)?
                .document()
                .ok_or(Error::WebAPIFailed)?;
            let anchor = document
                .create_element("a")
                .map_err(|_| Error::WebAPIFailed)?
                .dyn_into::<HtmlAnchorElement>()
                .map_err(|_| Error::WebAPIFailed)?;
            anchor.set_href(&url);
            anchor.set_download("download.pdf");
            anchor.click();
            Ok(())
        } else {
            Ok(())
        }
    };

    #[cfg(target_arch = "wasm32")]
    return rsx! {
        div {
            id: "pdf_buttons",
            style: "display: flex; justify-content: center; padding: 3rem 0; gap: 1rem;",
            button {
                class: "button",
                onclick: view_pdf,
                disabled: generating_pdf,
                "{i18n_lookup(\"create_pdf\")?}"
            }
            button {
                class: "button",
                onclick: download_pdf,
                disabled: pdf_url().is_none(),
                "{i18n_lookup(\"download\")?}"
            }
        }
        if let Some(url) = pdf_url() {
            div { style: "display: flex; justify-content: center;",
                iframe {
                    id: "pdf_view",
                    src: "{url}",
                    width: "80%",
                    height: "1000px",
                }
            }
        }
        footer { style: "height: 7rem;" }
    };
    #[cfg(not(target_arch = "wasm32"))]
    rsx! {
        div {
            id: "pdf_buttons",
            style: "display: flex; justify-content: center; padding: 3rem 0; gap: 1rem;",
            button { class: "button", disabled: true, "{i18n_lookup(\"create_pdf\")?}" }
            button { class: "button", disabled: true, "{i18n_lookup(\"download\")?}" }
        }
        if let Some(url) = pdf_url() {
            div { style: "display: flex; justify-content: center;",
                iframe {
                    id: "pdf_view",
                    src: "{url}",
                    width: "80%",
                    height: "1000px",
                }
            }
        }
        footer { style: "height: 7rem;" }
    }
}
