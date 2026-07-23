use {
    crate::{
        middlewares::mark_notification_read::NotificationToRead,
        shared::{wini::layer::Files, DatastarQuery},
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
    datastar::prelude::{PatchElements, PatchSignals},
    flate2::{write::GzEncoder, Compression},
    futures::Stream,
    hyper::header::{ACCEPT_ENCODING, CONTENT_ENCODING, CONTENT_TYPE, VARY},
    itertools::Itertools,
    maud::html,
    serde::Deserialize,
    serde_json,
    std::{
        convert::Infallible,
        io::Write,
        pin::Pin,
        task::{Context, Poll},
    },
};

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
