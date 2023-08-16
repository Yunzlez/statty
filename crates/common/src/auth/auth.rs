use actix::{Actor, Context, Handler};
use actix_web::web;
use oxide_auth::endpoint::Endpoint;
use oxide_auth::frontends::simple::endpoint::{Generic, Vacant};
use oxide_auth::primitives::issuer::TokenMap;
use oxide_auth::primitives::prelude::{AuthMap, Client, ClientMap, RandomGenerator, Scope};
use oxide_auth::primitives::registrar::RegisteredUrl;
use oxide_auth_actix::{Authorize, OAuthMessage, OAuthOperation, OAuthRequest, OAuthResponse, Refresh, Token, WebError};

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

enum Extras {
    Authorize,
    AuthorizationCodePkce,
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
}

impl Actor for AuthState {
    type Context = Context<Self>;
}

pub fn get_auth_routes() -> actix_web::Scope {
    web::scope("/auth")
        .service(web::resource("/authorize")
            .route(web::get().to(get_authorize)))
        .route("/token", web::post().to(token))
        .route("/refresh", web::post().to(refresh))
        .route("/patat", web::get().to(todo))
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
    ctx.get_auth_state().send(Token(req).wrap(Extras::AuthorizationCodePkce)).await?
}

async fn get_authorize(
    (req, ctx): (OAuthRequest, web::Data<AppContext>)
) -> Result<OAuthResponse, WebError> {
    ctx.get_auth_state().send(Authorize(req).wrap(Extras::Authorize)).await?
}

impl<Op> Handler<OAuthMessage<Op, Extras>> for AuthState where Op: OAuthOperation {

    type Result = Result<Op::Item, Op::Error>;

    fn handle(&mut self, msg: OAuthMessage<Op, Extras>, _: &mut Self::Context) -> Self::Result {
        let (op, ex) = msg.into_inner();

        match ex {
            Extras::Authorize=> {
                // let solicitor = FnSolicitor(move |_: &mut OAuthRequest, solicitation: Solicitation| {
                //    OwnerConsent::Authorized(solicitation.pre_grant().client_id.clone())
                // });
                op.run(&mut self.endpoint)
            }
            Extras::AuthorizationCodePkce => {
                op.run(&mut self.endpoint)
            }
            _ => { op.run(&mut self.endpoint)}
        }
        //todo!()
    }
}