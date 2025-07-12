use crate::backend::generate_pdf;
use crate::{Error, Result};
use dioxus::prelude::server_fn::error::ServerFnErrorErr;
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
        async move {
            generating_pdf.set(true);
            generation_error.set(None);

            let bytes = generate_pdf().await.map_err(ServerFnErrorErr::from)?;
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

    rsx! {
        div {
            button { onclick: view_pdf, disabled: generating_pdf, "Generate PDF" }
            button { onclick: download_pdf, disabled: pdf_url().is_none(), "Download" }
            if let Some(url) = pdf_url() {
                iframe { src: "{url}", width: "100%", height: "800px" }
            }
        }
    }
}
