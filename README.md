# Welcome to Loco :train:

[Loco](https://loco.rs) is a web and API framework running on Rust.

This is the **SaaS starter** which includes a `User` model and authentication based on JWT.
It also include configuration sections that help you pick either a frontend or a server-side template set up for your fullstack server.

## NOTICE

This was attempt to make loco-oauth2 work with Entra ID. Sadly it does not work and I currently give up.

Log:

```shell
2025-04-18T13:31:43.300301Z DEBUG http-request: tower_http::trace::on_request: started processing request http.method=GET http.uri=/api/oauth2/entraid/callback/cookie?code={redacted}&state=fHGUB2Vne-vVq380Le_3og&session_state=d6a03c3d-e28b-4d85-a8ac-596ea6fb7ee6 http.version=HTTP/1.1 http.user_agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 Edg/135.0.0.0 environment=development request_id=15d21f9a-6d53-469a-bfd6-89b353d94c84
2025-04-18T13:31:43.937273Z ERROR http-request: loco_rs::controller: controller_error error.msg=internal server error error.details=InternalServerError http.method=GET http.uri=/api/oauth2/entraid/callback/cookie?code={redacted}&state=fHGUB2Vne-vVq380Le_3og&session_state=d6a03c3d-e28b-4d85-a8ac-596ea6fb7ee6 http.version=HTTP/1.1 http.user_agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 Edg/135.0.0.0 environment=development request_id=15d21f9a-6d53-469a-bfd6-89b353d94c84
2025-04-18T13:31:43.937471Z DEBUG http-request: tower_http::trace::on_response: finished processing request latency=637 ms status=500 http.method=GET http.uri=/api/oauth2/entraid/callback/cookie?code={redacted}&state=fHGUB2Vne-vVq380Le_3og&session_state=d6a03c3d-e28b-4d85-a8ac-596ea6fb7ee6 http.version=HTTP/1.1 http.user_agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 Edg/135.0.0.0 environment=development request_id=15d21f9a-6d53-469a-bfd6-89b353d94c84
2025-04-18T13:31:43.937525Z ERROR http-request: tower_http::trace::on_failure: response failed classification=Status code: 500 Internal Server Error latency=637 ms http.method=GET http.uri=/api/oauth2/entraid/callback/cookie?code={redacted}&state=fHGUB2Vne-vVq380Le_3og&session_state=d6a03c3d-e28b-4d85-a8ac-596ea6fb7ee6 http.version=HTTP/1.1 http.user_agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/135.0.0.0 Safari/537.36 Edg/135.0.0.0 environment=development request_id=15d21f9a-6d53-469a-bfd6-89b353d94c84
```

## Deployment

To monitor background jobs, you may install [Loco Jobs Admin](https://github.com/loco-rs/admin-jobs)

## Developing

### Additional features

#### Adding OAuth

[Loco OAuth2](https://github.com/yinho999/loco-oauth2)

### Getting Started

Easiest way to develop is to use devcontainer. We use [loco-devcontainer](https://github.com/loco-rs/loco-devcontainer).

```sh
cargo loco start
```

```sh
$ cargo loco start
Finished dev [unoptimized + debuginfo] target(s) in 21.63s
    Running `target/debug/myapp start`

    :
    :
    :

controller/app_routes.rs:203: [Middleware] Adding log trace id

                      ▄     ▀
                                 ▀  ▄
                  ▄       ▀     ▄  ▄ ▄▀
                                    ▄ ▀▄▄
                        ▄     ▀    ▀  ▀▄▀█▄
                                          ▀█▄
▄▄▄▄▄▄▄  ▄▄▄▄▄▄▄▄▄   ▄▄▄▄▄▄▄▄▄▄▄ ▄▄▄▄▄▄▄▄▄ ▀▀█
 ██████  █████   ███ █████   ███ █████   ███ ▀█
 ██████  █████   ███ █████   ▀▀▀ █████   ███ ▄█▄
 ██████  █████   ███ █████       █████   ███ ████▄
 ██████  █████   ███ █████   ▄▄▄ █████   ███ █████
 ██████  █████   ███  ████   ███ █████   ███ ████▀
   ▀▀▀██▄ ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀▀▀▀ ██▀
       ▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
                https://loco.rs

environment: development
   database: automigrate
     logger: debug
compilation: debug
      modes: server

listening on http://localhost:5150
```

## Full Stack Serving

You can check your [configuration](config/development.yaml) to pick either frontend setup or server-side rendered template, and activate the relevant configuration sections.


## Getting help

Check out [a quick tour](https://loco.rs/docs/getting-started/tour/) or [the complete guide](https://loco.rs/docs/getting-started/guide/).
