use {
    crate::{
        shared::wini::layer::Files,
        template::collect_assets,
        utils::wini::buffer::buffer_to_string,
    },
    async_stream::stream,
    axum::{
        body::{Body, BodyDataStream, Bytes},
        extract::{Query, Request},
        http::HeaderValue,
        middleware::Next,
        response::{IntoResponse, Response, Sse},
    },
    datastar::prelude::PatchElements,
    flate2::{write::GzEncoder, Compression},
    futures::Stream,
    hyper::header::{ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_TYPE, VARY},
    itertools::Itertools,
    maud::html,
    std::{
        convert::Infallible,
        io::Write,
        pin::Pin,
        task::{Context, Poll},
    },
};

pub async fn replace(request: Request, next: Next) -> Response {
    let should_no_push_state = request.uri().query().is_some_and(|s| s.contains("no_push"));


    let uri_without_ds = {
        let mut uri = request.uri().path().split('/').skip(1).join("/");
        uri.insert(0, '/');
        uri
    };

    let (resp_parts, body) = next.run(request).await.into_parts();


    let (scripts_files, style_sheets) = collect_assets(resp_parts.extensions.get::<Files>());


    Sse::new(stream! {
        let patch = PatchElements::new(
            html! {
                @for style_sheet in style_sheets {
                    link rel="stylesheet" href=(style_sheet);
                }
                @for script in scripts_files {
                    script src=(script) defer type="module" {}
                }
            }
            .into_string(),
        )
        .selector("head")
        .mode(datastar::consts::ElementPatchMode::Append);

        yield Ok::<_, Infallible>(patch.write_as_axum_sse_event());

        // Body
        let patch = PatchElements::new(buffer_to_string(body).await.unwrap_or_default())
            .selector("#main")
            .mode(datastar::consts::ElementPatchMode::Inner);
        yield Ok(patch.write_as_axum_sse_event());

        if !should_no_push_state {
            // JSON-encode the URI to safely embed it in JavaScript string
            // context, preventing XSS via crafted URL characters.
            let json_uri =
                serde_json::to_string(&uri_without_ds).unwrap_or_else(|_| "\"/\"".to_owned());
            let patch = PatchElements::new(format!(
                "<script>window.history.pushState({{urlPath:{json_uri}}},\"\",{json_uri});</script>"
            ))
            .selector("head")
            .mode(datastar::consts::ElementPatchMode::Append);
            yield Ok(patch.write_as_axum_sse_event());

        }

        let patch = PatchElements::new("<script>_restoreOrResetScroll()</script>").selector("head")
            .mode(datastar::consts::ElementPatchMode::Append);
        yield Ok(patch.write_as_axum_sse_event());
    })
    .into_response()
}


pub async fn compress_sse(request: Request, next: Next) -> Response {
    let accept_encoding = request.headers().get(ACCEPT_ENCODING).cloned();

    let response = next.run(request).await;

    let content_encoding = response.headers().get(CONTENT_ENCODING);
    let content_type = response.headers().get(CONTENT_TYPE);

    // No accept-encoding from client or content-type from server.
    let (Some(ct), Some(ae)) = (content_type, accept_encoding) else {
        return response;
    };
    // Already compressed.
    if content_encoding.is_some() {
        return response;
    }
    // Not text/event-stream.
    if ct.as_bytes() != b"text/event-stream" {
        return response;
    }
    // Client doesn't accept gzip compression.
    // TODO: handle brotli, zstd, etc.
    if !ae.to_str().map(|v| v.contains("gzip")).unwrap_or(false) {
        return response;
    }

    let (mut parts, body) = response.into_parts();

    let body = body.into_data_stream();
    let body = Body::from_stream(CompressedStream::new(body));

    parts
        .headers
        .insert(CONTENT_ENCODING, HeaderValue::from_static("gzip"));
    parts
        .headers
        .insert(VARY, HeaderValue::from_static("accept-encoding"));

    Response::from_parts(parts, body)
}

struct CompressedStream {
    inner: BodyDataStream,
    compression: GzEncoder<Vec<u8>>,
}

impl CompressedStream {
    pub fn new(body: BodyDataStream) -> Self {
        Self {
            inner: body,
            compression: GzEncoder::new(Vec::new(), Compression::default()),
        }
    }
}

impl Stream for CompressedStream {
    type Item = Result<Bytes, axum::Error>;

    #[inline]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match std::pin::pin!(&mut self.inner).as_mut().poll_next(cx) {
            Poll::Ready(Some(Ok(x))) => {
                self.compression.write_all(&x).unwrap();
                self.compression.flush().unwrap();

                let mut buf = Vec::new();
                std::mem::swap(&mut buf, self.compression.get_mut());

                Poll::Ready(Some(Ok(buf.into())))
            },
            x => x,
        }
    }
}
