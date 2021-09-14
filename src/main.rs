#[macro_use]
extern crate capnp_rpc;

pub mod persistent_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/persistent_capnp.rs"));
}

pub mod stream_capnp {
    include!(concat!(env!("OUT_DIR"), "/capnp/stream_capnp.rs"));
}

pub mod activity_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/activity_capnp.rs"));
}

pub mod api_session_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/api_session_capnp.rs"));
}

pub mod grain_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/grain_capnp.rs"));
}

pub mod identity_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/identity_capnp.rs"));
}

pub mod ip_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/ip_capnp.rs"));
}

pub mod package_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/package_capnp.rs"));
}

pub mod powerbox_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/powerbox_capnp.rs"));
}

pub mod supervisor_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/supervisor_capnp.rs"));
}

pub mod util_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/util_capnp.rs"));
}

pub mod web_session_capnp {
    include!(concat!(env!("OUT_DIR"), "/sandstorm/web_session_capnp.rs"));
}

pub mod test_app_capnp {
    include!(concat!(env!("OUT_DIR"), "/test_app_capnp.rs"));
}

mod testpowerboxcapimpl {
    pub struct TestPowerboxCapImpl {
        text: String,
    }

    impl TestPowerboxCapImpl {
        pub fn new(text: &str) -> TestPowerboxCapImpl {
            TestPowerboxCapImpl {
                text: text.to_string(),
            }
        }
    }

    impl crate::test_app_capnp::test_powerbox_cap::Server for TestPowerboxCapImpl {
        fn read(
            &mut self,
            _params: crate::test_app_capnp::test_powerbox_cap::ReadParams,
            mut results: crate::test_app_capnp::test_powerbox_cap::ReadResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            results.get().set_text(&self.text);

            capnp::capability::Promise::ok(())
        }
    }

    impl crate::grain_capnp::app_persistent::Server<crate::test_app_capnp::object_id::Owned>
        for TestPowerboxCapImpl
    {
        fn save(
            &mut self,
            _: crate::grain_capnp::app_persistent::SaveParams<
                crate::test_app_capnp::object_id::Owned,
            >,
            mut results: crate::grain_capnp::app_persistent::SaveResults<
                crate::test_app_capnp::object_id::Owned,
            >,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            let mut results_builder = results.get();
            results_builder
                .reborrow()
                .init_object_id()
                .set_text(&self.text);
            results_builder
                .reborrow()
                .init_label()
                .set_default_text("some label");

            capnp::capability::Promise::ok(())
        }
    }
}

mod uiviewimpl {
    pub struct UiViewImpl {
        api: sandstorm::grain_capnp::sandstorm_api::Client<crate::test_app_capnp::object_id::Owned>,
    }

    impl UiViewImpl {
        pub fn new(
            api: sandstorm::grain_capnp::sandstorm_api::Client<
                crate::test_app_capnp::object_id::Owned,
            >,
        ) -> UiViewImpl {
            UiViewImpl { api }
        }
    }

    impl sandstorm::grain_capnp::ui_view::Server for UiViewImpl {
        fn get_view_info(
            &mut self,
            _params: sandstorm::grain_capnp::ui_view::GetViewInfoParams,
            mut results: sandstorm::grain_capnp::ui_view::GetViewInfoResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            use capnp::traits::HasTypeId;

            let view_info = results.get();

            let descriptor = view_info.init_match_requests(1).get(0);
            let mut tag = descriptor.init_tags(1).get(0);
            tag.set_id(crate::test_app_capnp::test_powerbox_cap::Client::type_id());
            pry!(tag
                .init_value()
                .set_as::<crate::test_app_capnp::test_powerbox_cap::powerbox_tag::Reader>(pry!(
                    crate::test_app_capnp::TEST_TAG.get()
                )));

            capnp::capability::Promise::ok(())
        }

        fn new_session(
            &mut self,
            params: sandstorm::grain_capnp::ui_view::NewSessionParams,
            mut results: sandstorm::grain_capnp::ui_view::NewSessionResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            use capnp::traits::HasTypeId;

            let params = pry!(params.get());
            if params.get_session_type()
                != sandstorm::web_session_capnp::web_session::Client::type_id()
            {
                return capnp::capability::Promise::err(capnp::Error::failed(
                    "Unsupported session type.".to_string(),
                ));
            }

            let session = pry!(crate::websessionimpl::WebSessionImpl::new(
                pry!(params.get_user_info()),
                pry!(params.get_context()),
                pry!(params.get_session_params().get_as()),
                self.api.clone(),
                false,
            ));
            let client: sandstorm::web_session_capnp::web_session::Client =
                capnp_rpc::new_client(session);
            results
                .get()
                .set_session(sandstorm::grain_capnp::ui_session::Client {
                    client: client.client,
                });
            capnp::capability::Promise::ok(())
        }

        fn new_request_session(
            &mut self,
            params: sandstorm::grain_capnp::ui_view::NewRequestSessionParams,
            mut results: sandstorm::grain_capnp::ui_view::NewRequestSessionResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            use capnp::traits::HasTypeId;

            let params = pry!(params.get());
            if params.get_session_type()
                != sandstorm::web_session_capnp::web_session::Client::type_id()
            {
                return capnp::capability::Promise::err(capnp::Error::failed(
                    "Unsupported session type.".to_string(),
                ));
            }

            let session = pry!(crate::websessionimpl::WebSessionImpl::new(
                pry!(params.get_user_info()),
                pry!(params.get_context()),
                pry!(params.get_session_params().get_as()),
                self.api.clone(),
                true,
            ));
            let client: sandstorm::web_session_capnp::web_session::Client =
                capnp_rpc::new_client(session);
            results
                .get()
                .set_session(sandstorm::grain_capnp::ui_session::Client {
                    client: client.client,
                });
            capnp::capability::Promise::ok(())
        }
    }
}

pub mod websessionimpl {
    pub struct WebSessionImpl {
        is_powerbox_request: bool,
        session_context: sandstorm::grain_capnp::session_context::Client,
        _api:
            sandstorm::grain_capnp::sandstorm_api::Client<crate::test_app_capnp::object_id::Owned>,
    }

    impl WebSessionImpl {
        pub fn new(
            _user_info: sandstorm::identity_capnp::user_info::Reader,
            context: sandstorm::grain_capnp::session_context::Client,
            _params: sandstorm::web_session_capnp::web_session::params::Reader,
            api: sandstorm::grain_capnp::sandstorm_api::Client<
                crate::test_app_capnp::object_id::Owned,
            >,
            is_powerbox_request: bool,
        ) -> capnp::Result<WebSessionImpl> {
            Ok(WebSessionImpl {
                is_powerbox_request,
                session_context: context,
                _api: api,
            })
        }
    }
    impl sandstorm::grain_capnp::ui_session::Server for WebSessionImpl {}
    impl sandstorm::web_session_capnp::web_session::Server for WebSessionImpl {
        fn get(
            &mut self,
            params: sandstorm::web_session_capnp::web_session::GetParams,
            mut results: sandstorm::web_session_capnp::web_session::GetResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            let path = pry!(pry!(params.get()).get_path());

            if path.is_empty() {
                let mut response = results.get().init_content();
                response.set_mime_type("text/html");
                response
                    .init_body()
                    .set_bytes(match self.is_powerbox_request {
                        true => crate::test_app_capnp::TEST_POWERBOX_HTML,
                        false => crate::test_app_capnp::TEST_APP_HTML,
                    });
            } else {
                let mut error = results.get().init_client_error();
                error.set_status_code(
                    sandstorm::web_session_capnp::web_session::response::ClientErrorCode::NotFound,
                );
            }
            capnp::capability::Promise::ok(())
        }
        fn post(
            &mut self,
            params: sandstorm::web_session_capnp::web_session::PostParams,
            mut results: sandstorm::web_session_capnp::web_session::PostResults,
        ) -> capnp::capability::Promise<(), capnp::Error> {
            // For promise.map_ok()
            use futures::TryFutureExt;

            let params_reader = pry!(params.get());
            let path = pry!(params_reader.get_path());

            if path == "fulfill" {
                results.get().init_no_content();

                let mut req = self.session_context.fulfill_request_request();
                let text = pry!(std::str::from_utf8(pry!(
                    pry!(params_reader.get_content()).get_content()
                )));
                let client: crate::test_app_capnp::test_powerbox_cap::Client =
                    capnp_rpc::new_client(crate::testpowerboxcapimpl::TestPowerboxCapImpl::new(
                        text,
                    ));
                req.get().get_cap().set_as_capability(client.client.hook);

                // The next section of code exists because I haven't figured out how to get the
                // Sandstorm Cap'n Proto files to map to the sandstorm crate.
                // Can't set pry!(crate::test_app_capnp::TEST_DESC.get())
                //                req.get().set_descriptor(pry!(crate::test_app_capnp::TEST_DESC.get()));
                let mut test_desc_builder = capnp::message::Builder::new_default();
                let mut test_desc = test_desc_builder
                    .init_root::<sandstorm::powerbox_capnp::powerbox_descriptor::Builder>(
                );
                let tags = test_desc.reborrow().init_tags(1);
                let mut tag = tags.get(0);
                tag.set_id(
                    pry!(pry!(crate::test_app_capnp::TEST_DESC.get()).get_tags())
                        .get(0)
                        .get_id(),
                );
                pry!(tag
                    .get_value()
                    .set_as::<crate::test_app_capnp::test_powerbox_cap::powerbox_tag::Reader>(
                        pry!(crate::test_app_capnp::TEST_TAG.get())
                    ));
                pry!(req.get().set_descriptor(test_desc.into_reader()));
                //                req.get().set_descriptor(pry!(crate::test_app_capnp::TEST_DESC.get()));

                return capnp::capability::Promise::from_future(req.send().promise.map_ok(|_| ()));
            } else if path == "accept" {
                return capnp::capability::Promise::ok(());
            }
            capnp::capability::Promise::ok(())
        }
    }
}

mod server {
    pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
        use futures::{AsyncReadExt, TryFutureExt};

        // Get the UNIX socket connection to Sandstorm
        let sandstorm_stream: ::std::os::unix::net::UnixStream =
            unsafe { ::std::os::unix::io::FromRawFd::from_raw_fd(3) };
        sandstorm_stream.set_nonblocking(true)?;

        // Hand the socket connection to Tokio
        let tokio_stream = tokio::net::UnixStream::from_std(sandstorm_stream)?;
        // Cap'n Proto RPC wants the stream split
        let (read_stream, write_stream) =
            tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio_stream).split();

        // Make the Cap'n Proto RPC network
        let rpc_network = Box::new(capnp_rpc::twoparty::VatNetwork::new(
            read_stream,
            write_stream,
            capnp_rpc::rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));

        // Build the RPC client and server
        let (to_client, from_server) = ::futures::channel::oneshot::channel();
        let sandstorm_api: sandstorm::grain_capnp::sandstorm_api::Client<
            crate::test_app_capnp::object_id::Owned,
        > = ::capnp_rpc::new_promise_client(from_server.map_err(|_| {
            capnp::Error::failed("Failed to send the client to the other side".to_string())
        }));
        let client: sandstorm::grain_capnp::ui_view::Client =
            capnp_rpc::new_client(crate::uiviewimpl::UiViewImpl::new(sandstorm_api));
        let mut rpc_system = capnp_rpc::RpcSystem::new(rpc_network, Some(client.client));
        drop(
            to_client.send(
                rpc_system
                    .bootstrap::<sandstorm::grain_capnp::sandstorm_api::Client<
                        crate::test_app_capnp::object_id::Owned,
                    >>(::capnp_rpc::rpc_twoparty_capnp::Side::Server)
                    .client,
            ),
        );
        rpc_system.await?;
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    server::main().await?;
    Ok(())
}
