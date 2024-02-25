// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json as JsonResponse},
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use methods::{PUZZLE_ELF, PUZZLE_ID};
use puzzle_core::{AdminData, EndGame, Entry, Submission, Winner, PuzzleSize};
use risc0_binfmt::{MemoryImage, Program};
use risc0_zkvm::{
    get_prover_server, sha::Digest, ExecutorEnv, ExecutorImpl, ProverOpts, VerifierContext,
    PAGE_SIZE,
};
use serde_json::json;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use tokio::net::TcpListener;

#[derive(Clone)]
struct SharedVariables {
    image: MemoryImage,
    n: u32,
    game_open: bool,
    game_played: bool,
    winner: Winner,
}

#[derive(Clone)]
struct AppState {
    state: Arc<RwLock<SharedVariables>>,
}

fn prover_opts_fast() -> ProverOpts {
    ProverOpts {
        hashfn: "sha-256".to_string(),
        prove_guest_errors: false,
    }
}
// only the admin can access this
// sends N (size of the puzzle array) to the guest
async fn run_initial_executor(
    State(global_vars): State<AppState>,
    Json(admin): Json<AdminData>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let read_guard = global_vars.state.read().unwrap();
    if read_guard.game_open == false && admin.password == String::from("TEMP") {
        drop(read_guard);

        let env = ExecutorEnv::builder()
            .write(&admin.puzzle_size) // size of the array.  Guest creates hashmap of factors for every integer [1..admin.puzzle_size]
            .unwrap()
            .write(&false) // starts the guest loop
            .unwrap()
            .build()
            .unwrap();

        let mut exec = ExecutorImpl::from_elf(env, PUZZLE_ELF).unwrap();
        let image_id: Digest = PUZZLE_ID.into();
        let session = exec.run().unwrap();

        let prover = get_prover_server(&prover_opts_fast()).unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        receipt.verify(image_id).unwrap();

        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.image = session.post_image.clone();
        write_guard.n = admin.puzzle_size;
        write_guard.game_open = true;
        drop(write_guard);

        println!("The game has now officially begun.");

        let puzzle_size_response = PuzzleSize {
            size: admin.puzzle_size.clone(),
        };

        Ok(JsonResponse(puzzle_size_response))
    } else if admin.password != String::from("TEMP") {
        Err(Html(format!(
            "<h1>Only the Admin can start a new Game!</h1>"
        )))
    } else {
        Err(Html(format!(
            "<h1>A game is currently in session!\n
        You need to end the current game before starting a new one!</h1>"
        )))
    }
}

async fn run_executor_for_submissions(
    State(global_vars): State<AppState>,
    Json(entry): Json<Entry>,
) -> Result<impl IntoResponse, StatusCode> {
    let shared = global_vars.state.read().unwrap();
    if shared.game_open == true {
        let (strat, name) = (entry.strategy, entry.name);
        let submission = Submission::new(strat, name, shared.n);
        drop(shared);

        let env = ExecutorEnv::builder()
            .write(&true)
            .unwrap()
            .write(&submission)
            .unwrap()
            .build()
            .unwrap();

        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.image.pc += 4;
        drop(write_guard);

        let read_guard = global_vars.state.read().unwrap();
        let image_id = read_guard.image.compute_id().unwrap();

        let mut exec = ExecutorImpl::new(env, read_guard.image.clone()).unwrap();
        drop(read_guard);

        let session = exec.run().unwrap();

        let prover = get_prover_server(&prover_opts_fast()).unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.image = session.post_image;
        drop(write_guard);

        receipt.verify(image_id).unwrap();

        let result: Submission = receipt.journal.decode().unwrap();
        let response = json!(result);

        Ok(JsonResponse(response))
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

// When the admin ends the game, users can no longer submit entries to the zkVM
async fn end_game(
    State(global_vars): State<AppState>,
    Json(admin): Json<EndGame>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let read_guard = global_vars.state.read().unwrap();
    if read_guard.game_open == true && admin.password == String::from("TEMP") {
        drop(read_guard);

        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.game_open = false;
        write_guard.game_played = true;
        drop(write_guard);

        let env = ExecutorEnv::builder()
            .write(&false) // break the guest code loop
            .unwrap()
            .build()
            .unwrap();

        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.image.pc += 4;
        drop(write_guard);

        let read_guard = global_vars.state.read().unwrap();
        let image_id = read_guard.image.compute_id().unwrap();
        let mut exec = ExecutorImpl::new(env, read_guard.image.clone()).unwrap();
        drop(read_guard);

        let session = exec.run().unwrap();

        let prover = get_prover_server(&prover_opts_fast()).unwrap();
        let receipt = prover
            .prove_session(&VerifierContext::default(), &session)
            .unwrap();

        receipt.verify(image_id).unwrap();

        let (winning_score, winning_name): (u32, String) = receipt.journal.decode().unwrap();

        // committing the score is optional.  It might even be a better showcase of the zero-knowledge aspect
        // of the zkVM to not committ the score and only commit the winner's name.
        let result: Winner = Winner {
            score: winning_score,
            winner: winning_name,
        };

        // Set the GAME_PLAYED state back to false for a new game to be played.
        let mut write_guard = global_vars.state.write().unwrap();
        write_guard.winner = result.clone();
        write_guard.game_played = false;

        let response = json!(
        {
        "score": result.score,
        "winner": result.winner,
        }
        );

        Ok(JsonResponse(response))
    } else if read_guard.game_open == true && admin.password != String::from("TEMP") {
        Err(Html(format!("<h1>Only the Admin can end the game!</h1>")))
    } else {
        Err(Html(format!("<h1>The Game is already over!</h1>")))
    }
}

async fn get_winner(
    State(global_vars): State<AppState>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let read_guard = global_vars.state.read().unwrap();

    // we make sure that the a winner has been selected by checking to make sure the winner category isn't an empty string
    if read_guard.game_open == false && read_guard.winner.winner != "".to_string() {
        let winner = Winner {
            score: read_guard.winner.score,
            winner: read_guard.winner.winner.clone(),
        };

        let winner_json = json!(winner);
        Ok(JsonResponse(winner_json))
    } else if read_guard.game_open == true {
        Err(Html(format!("<h1>The Game isn't over yet!</h1>")))
    } else {
        Err(Html(format!("<h1>The Game hasn't started yet!</h1>")))
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let vars = SharedVariables {
        image: MemoryImage::new(
            &Program::load_elf(PUZZLE_ELF, risc0_zkvm::GUEST_MAX_MEM as u32)
                .expect("Could not load ELF"),
            PAGE_SIZE as u32,
        )
        .expect("Could not create memory image"),
        n: 1u32,
        game_open: false,
        game_played: false,
        winner: Winner {
            score: 0,
            winner: "".to_string(),
        },
    };

    let global_vars = AppState {
        state: Arc::new(RwLock::new(vars)),
    };

    let app = Router::new()
        .route(
            "/start",
            post(run_initial_executor),
        )
        .route(
            "/submission",
            post(run_executor_for_submissions),
        )
        .route("/end", post(end_game),
        )
        .route("/winner", get(get_winner),
       )
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening at localhost/8080");
    axum::serve(listener, app.with_state(global_vars.clone()))
        .await
        .unwrap();
}
