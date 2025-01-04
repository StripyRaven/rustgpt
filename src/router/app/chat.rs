#![allow(dead_code)]
/**
 * # Chat
 * chatting with ai using openai
 * - list models
 * - chat
 * - stream
 * - delete chat
 * - delete chat message
 */
///////////////////////////////////////////////////////////////////////////////////////////////////
// LOCAL
use crate::model::{app_state::AppStateProject, user_dto::UserDTO};

use crate::{
    ai_layer::stream::{generate_sse_stream, list_engines, GenerationEvent},
    model::model::ChatMessagePair,
};
// EXTERNAL
use axum::{
    //debug_handler,
    extract::{Extension, Path, State},
    http::StatusCode,
    response::{sse::Event, Html, IntoResponse, Response, Sse},
    Form,
    Json,
};

use futures::stream::{self};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tera::Context;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream; // This brings the necessary stream combinators into scope

use tokio_stream::StreamExt as TokioStreamExt;

pub enum ChatError {
    Other,
    InvalidAPIKey,
    EmptyKey,
    NoneObject,
}
// Implement Display for ChatError to provide user-facing error messages.

impl IntoResponse for ChatError {
    fn into_response(self) -> Response {
        match self {
            ChatError::Other => (StatusCode::BAD_REQUEST, Json("Chat Error")).into_response(),
            ChatError::InvalidAPIKey => {
                (StatusCode::UNAUTHORIZED, Json("Chat Error")).into_response()
            }
            ChatError::EmptyKey => {
                (StatusCode::UNAUTHORIZED, Json("None Key Chat Error")).into_response()
            }
            ChatError::NoneObject => {
                (StatusCode::UNAUTHORIZED, Json("None Object Chat Error")).into_response()
            }
        }
    }
}

// TODO move to models and turn to vars, maybe .ENV better to Tree or Json or common struct with all necessary fields
// array of 5 tuples with 3 string each
const MODELS: [(&str, &str, &str); 5] = [
    // 1
    (
        "GPT-4-Preview",
        "gpt-4-1106-preview",
        "This is the preview version of the GPT-4 model.",
    ),
    // 2
    ("GPT-4", "gpt-4", "Latest generation GPT-4 model."),
    // 3
    (
        "GPT-3.5-16K",
        "gpt-3.5-turbo-16k",
        "An enhanced GPT-3.5 model with 16K token limit.",
    ),
    // 4
    (
        "GPT-3.5",
        "gpt-3.5-turbo",
        "Standard GPT-3.5 model with turbo features.",
    ),
    // 5
    (
        "Ollama",
        "qwen2.5-coder",
        "Qwen coder were to define tokens?",
    ),
];

/** # fn chat
* chat page

*
*/
#[axum::debug_handler]
pub async fn chat(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user_data): Extension<Option<UserDTO>>,
) -> Html<String> {
    tracing::info!("Enter CHAT");

    let user_chats = state
        .chat_repo
        .get_all_chats(current_user_data.as_ref().unwrap().id)
        .await
        .unwrap();

    // TODO! default selection for model
    let selected_model = MODELS.iter().find(|&f| f.1 == "qwen2.5-coder").unwrap();

    let mut context = Context::new();
    context.insert("models", &MODELS);
    context.insert("selected_model", &selected_model);
    context.insert("user_chats", &user_chats);
    let home = state.tera_templates.render("views/chat.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    context.insert("current_user_data", &current_user_data);
    let rendered = state.tera_templates.render("views/main.html", &context).unwrap();

    Html(rendered)
}
// TODO move to models
#[derive(Deserialize, Debug)]
pub struct NewChat {
    message: String,
    model: String,
}

/**
 * # fn new_chat
 * create new chat
 * - create chat
 *
 */
#[axum::debug_handler]
pub async fn new_chat(
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user_data): Extension<Option<UserDTO>>,
    Form(new_chat): Form<NewChat>,
) -> Result<Response<String>, ChatError> {
    tracing::info!("Enter NEW_CHAT");
    let current_user_data = current_user_data.unwrap();

    let chat_id = state
        .chat_repo
        .create_chat(current_user_data.id, &new_chat.message, &new_chat.model)
        .await
        .map_err(|_| ChatError::Other)?;

    state
        .chat_repo
        .add_message_block(chat_id, &new_chat.message)
        .await
        .map_err(|_| ChatError::Other)?;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("HX-Redirect", format!("/chat/{}", chat_id).as_str())
        .body("".to_string())
        .unwrap())
}

#[derive(Serialize, Deserialize, Debug)]
struct ParsedMessagePair {
    pair: ChatMessagePair,
    human_message_html: String,
    ai_message_html: String,
}

#[axum::debug_handler]
pub async fn chat_by_id(
    Path(chat_id): Path<i64>,
    State(state): State<Arc<AppStateProject>>,
    Extension(current_user_data): Extension<Option<UserDTO>>,
) -> Result<Html<String>, ChatError> {
    tracing::info!("Enter CHAT_BY_ID");
    let chat_message_pairs = state
        .chat_repo
        .retrieve_chat(chat_id)
        .await
        .map_err(|_| ChatError::Other)?;

    let user_chats = state
        .chat_repo
        .get_all_chats(current_user_data.as_ref().unwrap().id)
        .await
        .unwrap();

    let selected_model = MODELS
        .iter()
        .filter(|f| f.1 == chat_message_pairs[0].model)
        .collect::<Vec<_>>()[0];

    let parsed_pairs = chat_message_pairs
        .iter()
        .map(|pair| {
            let human_message_html =
                comrak::markdown_to_html(&pair.human_message, &comrak::Options::default());
            let ai_message_html = comrak::markdown_to_html(
                &pair.clone().ai_message.unwrap_or("".to_string()),
                &comrak::Options::default(),
            );
            ParsedMessagePair {
                pair: pair.clone(),
                human_message_html,
                ai_message_html,
            }
        })
        .collect::<Vec<_>>();

    let mut context = Context::new();
    context.insert("name", "World");
    context.insert("chat_message_pairs", &parsed_pairs);
    context.insert("chat_id", &chat_id);
    context.insert("user_chats", &user_chats);
    context.insert("selected_model", &selected_model);

    let home = state.tera_templates.render("views/chat.html", &context).unwrap();

    let mut context = Context::new();
    context.insert("view", &home);
    context.insert("current_user_data", &current_user_data);
    let rendered = state.tera_templates.render("views/main.html", &context).unwrap();

    Ok(Html(rendered))
}

// TODO move to models
#[derive(Deserialize, Debug)]
pub struct ChatAddMessage {
    message: String,
}

// todo rename `Extension`to `CurrentUserData`
#[axum::debug_handler]
pub async fn chat_add_message(
    Path(chat_id): Path<i64>,
    State(state): State<Arc<AppStateProject>>,
    Extension(_current_user_data): Extension<Option<UserDTO>>,
    Form(chat_add_message): Form<ChatAddMessage>,
) -> Result<Html<String>, ChatError> {
    tracing::info!("Enter CHAT_ADD_MESSAGE");
    let message = chat_add_message.message;
    state
        .chat_repo
        .add_message_block(chat_id, &message)
        .await
        .map_err(|_| ChatError::Other)?;

    let mut context = Context::new();
    context.insert("human_message", &message);
    context.insert("chat_id", &chat_id);
    let update = state
        .tera_templates
        .render("htmx_updates/add_message.html", &context)
        .unwrap();

    Ok(Html(update))
}

#[axum::debug_handler]
pub async fn generate_chat(
    Extension(current_user): Extension<Option<UserDTO>>,
    Path(chat_id): Path<i64>,
    State(state): State<Arc<AppStateProject>>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, axum::Error>>>, ChatError> {
    tracing::info!("Enter GENERATE_CHAT");
    let chat_message_pairs = state.chat_repo.retrieve_chat(chat_id).await.unwrap();
    let key = current_user // нетт проверок но и нет заимствований срока жизни
        .unwrap()
        .openai_api_key
        .unwrap_or(String::new());

    match list_engines(&key).await {
        Ok(_res) => {}
        Err(_) => {
            return Err(ChatError::InvalidAPIKey);
        }
    };

    let lat_message_id = chat_message_pairs.last().unwrap().id;

    // Create a channel for sending SSE events
    let (sender, receiver) = mpsc::channel::<Result<GenerationEvent, axum::Error>>(10);

    // Spawn a task that generates SSE events and sends them into the channel
    tokio::spawn(async move {
        // Call your existing function to start generating events
        if let Err(e) = generate_sse_stream(
            &key,
            &chat_message_pairs[0].model.clone(),
            chat_message_pairs,
            sender,
        )
        .await
        {
            tracing::error!("Error generating SSE stream: {:?}", e);
        }
    });

    // Convert the receiver into a Stream that can be used by Sse
    // let event_stream = ReceiverStream::new(receiver);
    let state_clone = Arc::clone(&state);

    let receiver_stream = ReceiverStream::new(receiver);
    let initial_state = (receiver_stream, String::new()); // Initial state with an empty accumulator
    let event_stream = stream::unfold(initial_state, move |(mut rc, mut accumulated)| {
        let state_clone = Arc::clone(&state_clone); // Clone the Arc here
        async move {
            match rc.next().await {
                Some(Ok(event)) => {
                    // Process the event
                    match event {
                        GenerationEvent::Text(text) => {
                            accumulated.push_str(&text);
                            // Return the accumulated data as part of the SSE event
                            let html =
                                comrak::markdown_to_html(&accumulated, &comrak::Options::default());
                            let s = format!(r##"<div>{}<div>"##, html);

                            Some((Ok(Event::default().data(s)), (rc, accumulated)))
                        }
                        GenerationEvent::End(text) => {
                            println!("accumulated: {:?}", accumulated);

                            state_clone
                                .chat_repo
                                .add_ai_message_to_pair(lat_message_id, &accumulated)
                                .await
                                .unwrap();

                            let html =
                                comrak::markdown_to_html(&accumulated, &comrak::Options::default());

                            let s = format!(
                                r##"<div hx-swap-oob="outerHTML:#message-container">{}</div>"##,
                                html
                            );
                            // append s to text
                            let ss = format!("{}\n{}", text, s);
                            println!("ss: {}", ss);

                            // accumulated.push_str(&ss);
                            // Handle the end of a sequence, possibly resetting the accumulator if needed
                            Some((Ok(Event::default().data(ss)), (rc, String::new())))
                        } // ... handle other event types if necessary ...
                    }
                }
                Some(Err(e)) => {
                    // Handle error without altering the accumulator
                    Some((Err(axum::Error::new(e)), (rc, accumulated)))
                }
                None => None, // When the receiver stream ends, finish the stream
            }
        }
    });

    Ok(Sse::new(event_stream))
}

pub async fn delete_chat(
    Path(chat_id): Path<i64>,
    State(state): State<Arc<AppStateProject>>,
) -> Result<Html<String>, ChatError> {
    tracing::info!("Enter DELETE_CHAT");
    state.chat_repo.delete_chat(chat_id).await.unwrap();

    let html = r#"<div class="hidden"></div>"#;

    Ok(Html(html.to_string()))
}
