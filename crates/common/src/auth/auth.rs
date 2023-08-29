use actix::{Actor, Context, Handler};
use actix_web::web::Json;
use actix_web::web;
use log::debug;
use oxide_auth::endpoint::{Endpoint, Solicitation, OwnerConsent, OwnerSolicitor};
use oxide_auth::frontends::simple::endpoint::{Generic, Vacant, FnSolicitor, ErrorInto};
use oxide_auth::primitives::issuer::TokenMap;
use oxide_auth::primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope};
use oxide_auth::primitives::registrar::RegisteredUrl;
use oxide_auth_actix::{Authorize, OAuthMessage, OAuthOperation, OAuthRequest, OAuthResponse, Refresh, Token, WebError};
use serde::Deserialize;

use crate::context::Context as AppContext;

pub struct AuthState {
    endpoint: Generic<
        ClientMap,
        AuthMap<RandomGenerator>,
        TokenMap<RandomGenerator>,
        Vacant,
        Vec<Scope>,
        fn() ->OAuthResponse
    >
}

#[derive(Debug, Clone, Deserialize)]
struct Credentials {
    username: String,
    password: String
}

enum Extras {
    Authorize,
    AuthorizePost(Credentials),
    Nothing
}

impl AuthState {
    pub fn preconfigured() -> Self {
        AuthState {
            endpoint: Generic {
                registrar: vec![Client::public(
                    "statty-frontend",
                    RegisteredUrl::Exact("http://localhost:8080/auth".parse().unwrap()),
                    "".parse().unwrap()
                )].into_iter().collect(),
                authorizer: AuthMap::new(RandomGenerator::new(16)),
                issuer: TokenMap::new(RandomGenerator::new(16)),
                solicitor: Vacant,
                scopes: vec![],
                response: OAuthResponse::ok,
            }
        }
    }

    pub fn with_solicitor<'a, S>(
        &'a mut self, solicitor: S,
    ) -> impl Endpoint<OAuthRequest, Error = WebError> + 'a
    where
        S: OwnerSolicitor<OAuthRequest> + 'static,
    {
        ErrorInto::new(Generic {
            authorizer: &mut self.endpoint.authorizer,
            registrar: &mut self.endpoint.registrar,
            issuer: &mut self.endpoint.issuer,
            solicitor,
            scopes: &mut self.endpoint.scopes,
            response: OAuthResponse::ok,
        })
    }
}

impl Actor for AuthState {
    type Context = Context<Self>;
}

pub fn auth_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::scope("/auth")
        .service(web::resource("/authorize")
            .route(web::get().to(get_authorize))
            .route(web::post().to(post_authorize)))
        .route("/token", web::post().to(token))
        .route("/refresh", web::post().to(refresh))
    );
}

async fn todo() -> Result<OAuthResponse, WebError> {
    Ok(OAuthResponse::ok())
}

async fn refresh(
    (req, ctx): (OAuthRequest, web::Data<AppContext>)
) -> Result<OAuthResponse, WebError> {
    ctx.get_auth_state().send(Refresh(req).wrap(Extras::Nothing)).await?
}

async fn token(
    (req, ctx): (OAuthRequest, web::Data<AppContext>)
) -> Result<OAuthResponse, WebError> {
    ctx.get_auth_state().send(Token(req).wrap(Extras::Nothing)).await?
}

async fn get_authorize(
    (req, ctx): (OAuthRequest, web::Data<AppContext>)
) -> Result<OAuthResponse, WebError> {
    ctx.get_auth_state().send(Authorize(req).wrap(Extras::Authorize)).await?
}

async fn post_authorize(
    (data, req, ctx): (Json<Credentials>, OAuthRequest, web::Data<AppContext>)
) -> Result<OAuthResponse, WebError> {
    debug!("data: {:?}", data.clone());
    ctx.get_auth_state().send(Authorize(req).wrap(Extras::AuthorizePost(data.into_inner()))).await?
}

impl<Op> Handler<OAuthMessage<Op, Extras>> for AuthState where Op: OAuthOperation {

    type Result = Result<Op::Item, Op::Error>;

    fn handle(&mut self, msg: OAuthMessage<Op, Extras>, _: &mut Self::Context) -> Self::Result {
        let (op, ex) = msg.into_inner();

        match ex {
            Extras::Authorize=> {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, solicitation: Solicitation| {
                   OwnerConsent::InProgress(
                    OAuthResponse::ok()
                    .content_type("text/html")
                    .unwrap()
                    .body(
                        "<p>lol hey</p>"
                    )
                   )
                });
                
                //todo handle oauth error
                op.run(self.with_solicitor(solicitor))
            }
            Extras::AuthorizePost(credentials) => {
                let solicitor = FnSolicitor(move |_: &mut OAuthRequest, solicitation: Solicitation| { 
                    debug!("imagine logging in lmao: {:?}", credentials);
                    return OwnerConsent::Authorized("datBoi".to_owned())
                    //OwnerConsent::Denied
                });

                op.run(&mut self.with_solicitor(solicitor))
            }
            _ => { op.run(&mut self.endpoint)}
        }
        //todo!()
    }
}