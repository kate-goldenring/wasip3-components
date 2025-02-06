mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "wasi:http/proxy",
        async: {
            imports: [
                "wasi:http/types@0.3.0-draft#[static]body.finish",
                "wasi:http/handler@0.3.0-draft#handle",
            ],
            exports: [
                "wasi:http/handler@0.3.0-draft#handle",
            ]
        }
    });

    use super::Component;
    export!(Component);
}

use {
    bindings::{
        exports::wasi::http::handler::Guest as Handler,
        wasi::http::types::{Body, ErrorCode, Request, Response},
        wit_future, wit_stream,
    },
    futures::SinkExt,
    rand::Rng,
    wit_bindgen_rt::async_support,
};

// TODO: This is a hack because file mapping and reading is not working
const IMAGES: &[&[u8]] = &[include_bytes!("../images/0.jpg"), include_bytes!("../images/1.jpg"), include_bytes!("../images/2.jpg"), include_bytes!("../images/3.jpg"), include_bytes!("../images/4.jpg"), include_bytes!("../images/5.jpg"), include_bytes!("../images/6.jpg"), include_bytes!("../images/7.jpg"), include_bytes!("../images/fake.jpg")];

struct Component;

impl Handler for Component {
    /// Return a response which echoes the request headers, body, and trailers.
    async fn handle(request: Request) -> Result<Response, ErrorCode> {
        let (headers, _) = Request::into_parts(request);
        let image_num = rand::thread_rng().gen_range(0..IMAGES.len());

        // ...but we do it the more difficult, less efficient way here to exercise various component model
        // features (e.g. `future`s, `stream`s, and post-return asynchronous execution):
        let (_trailers_tx, trailers_rx) = wit_future::new();
        let (mut pipe_tx, pipe_rx) = wit_stream::new();

        async_support::spawn(async move {
            pipe_tx.send(IMAGES[image_num].to_vec()).await.unwrap();
        });

        Ok(Response::new(
            headers,
            Body::new(pipe_rx, Some(trailers_rx)),
        ))
    }
}
