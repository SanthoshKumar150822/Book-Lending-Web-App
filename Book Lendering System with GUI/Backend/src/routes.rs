use warp::{Filter, Rejection, Reply};
use serde::{Deserialize, Serialize};
use crate::{models::{User, NewUser, UsernameAvailability}, database};
use sqlx::SqlitePool;

pub fn routes(pool: SqlitePool) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    register_route(pool.clone())
        .or(username_available_route(pool.clone()))
}

fn register_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(pool))
        .and_then(register_handler)
}

fn username_available_route(
    pool: SqlitePool,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "username" / String)
        .and(warp::get())
        .and(with_db(pool))
        .and_then(username_available_handler)
}

async fn register_handler(new_user: NewUser, pool: SqlitePool) -> Result<impl Reply, Rejection> {
    // Validate email ends with @gmail.com
    if !new_user.email.ends_with("@gmail.com") {
        return Ok(warp::reply::with_status(
            "Only Gmail addresses are allowed".to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    // Validate password complexity
    let password_pattern = regex::Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&.,])[A-Za-z\d@$!%*?&.,]{8,}$")
        .unwrap();
    if !password_pattern.is_match(&new_user.password) {
        return Ok(warp::reply::with_status(
            "Password must be at least 8 characters and include uppercase, lowercase, number, and special character".to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ));
    }

    match new_user.create(&pool).await {
        Ok(user) => Ok(warp::reply::json(&user)),
        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.is_unique_violation() {
                    if db_err.message().contains("username") {
                        return Ok(warp::reply::with_status(
                            "Username already exists".to_string(),
                            warp::http::StatusCode::CONFLICT,
                        ));
                    } else if db_err.message().contains("email") {
                        return Ok(warp::reply::with_status(
                            "Email already registered".to_string(),
                            warp::http::StatusCode::CONFLICT,
                        ));
                    }
                }
            }
            Ok(warp::reply::with_status(
                "Registration failed".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

async fn username_available_handler(
    username: String,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    match check_username_available(&username, &pool).await {
        Ok(available) => Ok(warp::reply::json(&UsernameAvailability { available })),
        Err(e) => {
            println!("Error checking username: {}", e);
            Ok(warp::reply::with_status(
                "Error checking username availability".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

fn with_db(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}