//! Config Mod

use axum::Json;

use crate::error::app::AppError;

pub mod app;

pub type CommonResult<T> = Result<Json<T>, AppError>;
