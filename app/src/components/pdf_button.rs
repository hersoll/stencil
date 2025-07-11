use crate::backend::generate_pdf;
use crate::{Error, Result};
use dioxus::prelude::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url, window};

#[component]
pub fn PDFButtons() -> Element {
    let mut generating_pdf = use_signal(|| false);
    let mut pdf_url = use_signal(|| None::<String>);
    let mut generation_error = use_signal(|| None::<String>);

    let view_pdf = move |_| {
        spawn(async move {
            generating_pdf.set(true);
            generation_error.set(None);

            match generate_pdf().await {
                Ok(bytes) => {
                    // Potential JS Errors in here
                    let result = || -> Result<String> {
                        let uint8_array = Uint8Array::from(bytes.as_slice());
                        let array = js_sys::Array::new();
                        array.push(&uint8_array.buffer());
                        let blob_options = BlobPropertyBag::new();
                        blob_options.set_type("application/pdf");
                        let blob = Blob::new_with_buffer_source_sequence_and_options(
                            &array,
                            &blob_options,
                        )
                        .map_err(|_| Error::PDFLoadingFailed)?;
                        let url = Url::create_object_url_with_blob(&blob)
                            .map_err(|_| Error::PDFLoadingFailed)?;
                        Ok(url)
                    };
                    match result() {
                        Ok(url) => {
                            pdf_url.set(Some(url));
                        }
                        Err(e) => {
                            generation_error.set(Some(e.to_string()));
                            web_sys::console::error_1(
                                &format!("PDF creation failed: {}", e).into(),
                            );
                        }
                    }
                }
                Err(e) => {
                    generation_error.set(Some(e.to_string()));
                    web_sys::console::error_1(&format!("PDF generation failed: {e}").into());
                }
            }
            generating_pdf.set(false);
        });
    };

    let download_pdf = move |_| {
        if let Some(url) = pdf_url() {
            let result = || -> Result<HtmlAnchorElement> {
                let document = window()
                    .ok_or(Error::WebAPIFailed)?
                    .document()
                    .ok_or(Error::WebAPIFailed)?;
                let a = document
                    .create_element("a")
                    .map_err(|_| Error::WebAPIFailed)?
                    .dyn_into::<HtmlAnchorElement>()
                    .map_err(|_| Error::WebAPIFailed)?;
                Ok(a)
            };
            match result() {
                Ok(anchor) => {
                    anchor.set_href(&url);
                    anchor.set_download("download.pdf");
                    anchor.click();
                }
                Err(e) => {
                    generation_error.set(Some(e.to_string()));
                    web_sys::console::error_1(&format!("PDF download failed: {}", e).into());
                }
            }
        }
    };

    rsx! {
        div {
            button { onclick: view_pdf, disabled: generating_pdf, "Generate PDF" }
            button { onclick: download_pdf, disabled: pdf_url().is_none(), "Download" }
            if let Some(error_message) = generation_error() {
                p {"{error_message}"}
        }
            else if let Some(url) = pdf_url() {
                iframe { src: "{url}", width: "100%", height: "800px" }
            }
        }
    }
}
