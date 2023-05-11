use crate::{
	database::{models::Paste, prelude::*, schema::pastes},
	error::ServerError,
	AppState,
};
use axum::{
	extract::{Path, State},
	http::StatusCode,
	response::IntoResponse,
};
use eyre::Context;

pub(crate) async fn get_paste(
	State(state): State<AppState>,
	Path(paste_slug): Path<String>,
) -> Result<impl IntoResponse, ServerError> {
	let mut conn = state
		.database
		.get()
		.await
		.wrap_err("Could not get a db handle")?;

	let res = Paste::with_slug(&paste_slug)
		.select(pastes::content)
		.first::<Vec<u8>>(&mut conn)
		.await
		.wrap_err("")?;

	Ok((StatusCode::OK, res))
}

pub(crate) async fn get_key_pastes() -> impl IntoResponse {
	StatusCode::NOT_IMPLEMENTED
}