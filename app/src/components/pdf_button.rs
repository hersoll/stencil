use crate::backend::generate_pdf;
use dioxus::prelude::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url, window};

#[component]
pub fn PDFButtons() -> Element {
    let mut generating_pdf = use_signal(|| false);
    let mut pdf_url = use_signal(|| None::<String>);

    let view_pdf = move |_| {
        spawn(async move {
            generating_pdf.set(true);

            match generate_pdf().await {
                Ok(bytes) => {
                    // Convert Rust Vec<u8> -> JS Uint8Array -> Blob
                    let uint8_array = Uint8Array::from(bytes.as_slice());
                    let array = js_sys::Array::new();
                    array.push(&uint8_array.buffer());
                    let blob_options = BlobPropertyBag::new();
                    blob_options.set_type("application/pdf");
                    let blob =
                        Blob::new_with_buffer_source_sequence_and_options(&array, &blob_options)
                            .unwrap();
                    let url = Url::create_object_url_with_blob(&blob).unwrap();
                    pdf_url.set(Some(url));
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("PDF generation failed: {e}").into());
                }
            }
            generating_pdf.set(false);
        });
    };

    let download_pdf = move |_| {
        if let Some(url) = pdf_url() {
            let document = window().unwrap().document().unwrap();
            let a = document
                .create_element("a")
                .unwrap()
                .dyn_into::<HtmlAnchorElement>()
                .unwrap();
            a.set_href(&url);
            a.set_download("download.pdf");
            a.click();
        }
    };

    rsx! {
        div {
            button { onclick: view_pdf, disabled: generating_pdf, "Generate PDF" }
            button { onclick: download_pdf, disabled: pdf_url().is_none(), "Download" }
            if let Some(url) = pdf_url() {
                iframe {
                    src: "{url}",
                    width: "100%",
                    height: "800px",
                }
            }
        }
    }
}
