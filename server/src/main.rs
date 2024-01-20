use crate::fluffy_chess_capnp::{
    game_config, game_maker,
    game_maker::{FindGameParams, FindGameResults, ResumeGameParams, ResumeGameResults},
    game_side,
    game_side::{ColorParams, ColorResults, IdParams, IdResults, MoveParams, MoveResults},
};
use capnp::capability::Promise;
use capnp::{Error, ErrorKind};
use capnp_rpc::{rpc_twoparty_capnp, twoparty, RpcSystem};
use clap::Parser;

use futures::{AsyncReadExt, FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, ToSocketAddrs};
use surrealdb::engine::local::Mem;
use surrealdb::{Connection, Surreal};

mod fluffy_chess_capnp {
    include!("../proto/fluffy_chess_capnp.rs");
}

struct GameMakerImpl<C: Connection> {
    db: Surreal<C>,
}

impl<C: Connection> GameMakerImpl<C> {
    fn new(db: Surreal<C>) -> Self {
        Self { db }
    }
}

#[derive(Deserialize, Serialize)]
enum Adversary<'a> {
    Any,
    Friends,
    User(&'a str),
}

impl<'a> TryFrom<game_config::adversary::Reader<'a>> for Adversary<'a> {
    type Error = Error;

    fn try_from(value: game_config::adversary::Reader<'a>) -> Result<Self, Self::Error> {
        Ok(match value.which()? {
            game_config::adversary::Any(_) => Adversary::Any,
            game_config::adversary::Friends(_) => Adversary::Friends,
            game_config::adversary::User(s) => Adversary::User(s?.to_str()?),
        })
    }
}

#[derive(Deserialize, Serialize)]
struct GameConfig<'a> {
    user: &'a str,
    adversary: Adversary<'a>,
}

impl<'a> TryFrom<game_config::Reader<'a>> for GameConfig<'a> {
    type Error = Error;

    fn try_from(value: game_config::Reader<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            user: value.get_user()?.to_str()?,
            adversary: value.get_adversary().try_into()?,
        })
    }
}

impl<C: Connection> game_maker::Server for GameMakerImpl<C> {
    fn find_game(
        &mut self,
        params: FindGameParams,
        mut results: FindGameResults,
    ) -> Promise<(), Error> {
        Promise::from_future(async {
            let game_config: GameConfig = params.get()?.get_game_config()?.try_into()?;
            let mut query_body = "SELECT game FROM lobby WHERE adversary=$username".to_string();
            match game_config.adversary {
                Adversary::User(_) => {
                    query_body.push_str(" AND username=$adversary");
                }
                _ => {}
            }
            let mut query = self.db.query(query_body).bind(game_config);
            if let Ok(mut game) = query.await {
                //
                let game_sides: Vec<u64> = game.take(0).map_err(|err| Error {
                    kind: ErrorKind::Failed,
                    extra: err.to_string(),
                })?;
                results
                    .get()
                    .set_game_side(capnp_rpc::new_client(GameSideImpl::new(
                        *game_sides.first().unwrap(),
                        self.db.clone(),
                    )))
            }

            Ok::<(), Error>(())
        })
    }

    fn resume_game(&mut self, _: ResumeGameParams, _: ResumeGameResults) -> Promise<(), Error> {
        todo!()
    }
}

struct GameSideImpl<C: Connection> {
    id: u64,
    db: Surreal<C>,
}

impl<C: Connection> GameSideImpl<C> {
    fn new(id: u64, db: Surreal<C>) -> Self {
        GameSideImpl { id, db }
    }
}

impl<C: Connection> game_side::Server for GameSideImpl<C> {
    fn id(&mut self, _: IdParams, _: IdResults) -> Promise<(), Error> {
        todo!()
    }

    fn color(&mut self, _: ColorParams, _: ColorResults) -> Promise<(), Error> {
        todo!()
    }

    fn move_(&mut self, _: MoveParams, _: MoveResults) -> Promise<(), Error> {
        todo!()
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(value_parser = parse_socket_addr, default_value = "localhost:7171")]
    address: SocketAddr,
}

fn parse_socket_addr(s: &str) -> Result<SocketAddr, String> {
    s.to_socket_addrs()
        .map_err(|err| format!("Unable to parse address: {err}"))?
        .next()
        .ok_or_else(|| "No address parsed".to_string())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    tokio::task::LocalSet::new()
        .run_until(async move {
            // Create database connection
            let db = Surreal::new::<Mem>(()).await?;

            // Select a specific namespace / database
            db.use_ns("fluffy_chess").use_db("main").await?;

            let game_maker_impl = GameMakerImpl::new(db.clone());
            let game_maker: game_maker::Client = capnp_rpc::new_client(game_maker_impl);

            let listener = tokio::net::TcpListener::bind(&args.address).await?;
            let handle_incoming = async move {
                loop {
                    let (stream, _) = listener.accept().await?;
                    stream.set_nodelay(true)?;
                    let (reader, writer) =
                        tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
                    let network = twoparty::VatNetwork::new(
                        reader,
                        writer,
                        rpc_twoparty_capnp::Side::Server,
                        Default::default(),
                    );
                    let rpc_system =
                        RpcSystem::new(Box::new(network), Some(game_maker.clone().client));

                    tokio::task::spawn_local(rpc_system);
                }
            };
            Ok(())
        })
        .await
}
