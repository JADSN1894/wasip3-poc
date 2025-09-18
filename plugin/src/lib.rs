use std::ops::Not;

wit_bindgen::generate!({
    world: "toggler",
    path: "../wit",
    async: true,
    // async: [
    //     "wasi:cli/run@0.3.0-rc-2025-08-15#run",
    //     "wasi:http/handler@0.3.0-draft#handle",
    // ],
    additional_derives_ignore: ["output", "input"],
    generate_all,
});

pub struct Toggler;

impl Guest for Toggler {
    #[allow(async_fn_in_trait)]
    async fn toggle(input: TogglerWrapper) -> TogglerWrapper {
        !input
    }
}

impl Not for TogglerWrapper {
    type Output = Self;

    fn not(self) -> Self::Output {
        TogglerWrapper { inner: !self.inner }
    }
}

export!(Toggler);
