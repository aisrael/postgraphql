use crate::app_state::AppState;
use crate::auth::Claims;
use crate::db_utils::pg_row_id;
use crate::graphql;
use axum::extract::{Path, State};
use axum::response::{IntoResponse, Json, Response};
use graphql_parser::query::{Definition, Document, Field, OperationDefinition, Selection};
use http::StatusCode;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Column, Row};
use std::collections::HashMap;

/// Handler for the health check endpoint, just returns HTTP 200 "OK"
pub async fn healthz() -> &'static str {
    "OK"
}

/// The authors endpoint
pub async fn authors(
    State(state): State<AppState>,
    claims: Claims,
) -> (StatusCode, Maybe<Json<Value>>) {
    println!("{:?}", state);
    println!("{:?}", claims);
    let AppState { book_store } = state;
    match book_store.list_authors().await {
        Ok(authors) => (StatusCode::OK, Maybe::Some(Json(json!(authors)))),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Maybe::None)
        }
    }
}

pub async fn get_author_by_id(
    State(state): State<AppState>,
    _claims: Claims,
    Path(author_id): Path<i64>,
) -> (StatusCode, Maybe<Json<Value>>) {
    println!("author_id: {}", author_id);

    let AppState { book_store } = state;
    match book_store.find_author_by_id(author_id).await {
        Ok(maybe_author) => match maybe_author {
            Some(author) => (StatusCode::OK, Maybe::Some(Json(json!(author)))),
            None => (StatusCode::NOT_FOUND, Maybe::None),
        },
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Maybe::None)
        }
    }
}

pub async fn list_books_by_author_id(
    State(state): State<AppState>,
    _claims: Claims,
    Path(author_id): Path<i64>,
) -> (StatusCode, Maybe<Json<Value>>) {
    let AppState { book_store } = state;
    match book_store.list_books_by_author_id(author_id).await {
        Ok(books) => (StatusCode::OK, Maybe::Some(Json(json!(books)))),
        Err(e) => {
            eprintln!("{:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Maybe::None)
        }
    }
}

#[derive(Debug)]
pub enum Maybe<T: IntoResponse> {
    Some(T),
    None,
}

/// Implement IntoResponse for Maybe<Json<Value>>
impl<T> IntoResponse for Maybe<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Maybe::Some(value) => value.into_response(),
            Maybe::None => ().into_response(),
        }
    }
}

pub async fn post_graphql(
    State(_state): State<AppState>,
    _claims: Claims,
    Json(request): Json<graphql::Request>,
) -> (StatusCode, Maybe<Json<Value>>) {
    match do_post_graphql(request).await {
        Ok((status, response)) => {
            let m = match response {
                Some(r) => Maybe::Some(Json(json!(r))),
                None => Maybe::None,
            };
            (status, m)
        }
        Err(e) => {
            println!("{:}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Maybe::None)
        }
    }
}

async fn do_post_graphql(
    request: graphql::Request,
) -> Result<(StatusCode, Option<graphql::Response>), anyhow::Error> {
    println!("{:?}", request);
    let query = request.query;
    let parse_result = graphql_parser::parse_query::<String>(&query);
    if let Err(e) = parse_result {
        eprintln!("{:?}", e);
        return Ok((StatusCode::BAD_REQUEST, None));
    };
    let doc = parse_result?;
    let mut qb = QueryBuilder::new(doc);
    let sql = qb.build()?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let rows = sqlx::query(&sql).fetch_all(&pool).await?;

    println!("Got {}", rows.len());

    let mapped: Vec<HashMap<String, Value>> = rows
        .iter()
        .map(|row| {
            let mut map: HashMap<String, Value> = HashMap::new();
            for col in row.columns() {
                if col.name() == "id" {
                    let id = pg_row_id(row).unwrap();
                    map.insert(col.name().to_string(), json!(id));
                } else {
                    let s = row.try_get::<String, _>(col.name()).unwrap();
                    map.insert(col.name().to_string(), json!(s));
                }
            }
            map
        }).collect();

    let response = graphql::Response {
        data: json!(mapped),
        ..Default::default()
    };

    Ok((StatusCode::OK, Some(response)))
}

struct QueryBuilder<'a> {
    doc: Document<'a, String>,
}

impl<'q> QueryBuilder<'q> {
    pub fn new(doc: Document<'q, String>) -> Self {
        QueryBuilder { doc }
    }

    pub fn build(&mut self) -> Result<String, anyhow::Error> {
        for definition in &mut self.doc.definitions {
            println!("definition: {:?}", definition);
            match definition {
                Definition::Operation(operation) => {
                    println!("operation: {:?}", operation);
                    match operation {
                        OperationDefinition::SelectionSet(selection_set) => {
                            println!("selection_set: {:?}", selection_set);
                            // We only handle the first for now
                            let selection_set = selection_set.items.first().unwrap();
                            match selection_set {
                                Selection::Field(field) => {
                                    println!("field: {:?}", field);
                                    let tqb = &mut TableQueryBuilder::new();
                                    let query = tqb.build(field)?;
                                    return Ok(query);
                                }
                                _ => todo!(),
                            }
                        }
                        _ => todo!(),
                    }
                }
                Definition::Fragment(fragment) => {
                    println!("fragment: {:?}", fragment);
                    todo!()
                }
            }
        }
        todo!()
    }
}

#[derive(Debug)]
struct TableQueryBuilder {
    parts: Vec<String>,
}

impl TableQueryBuilder {
    pub fn new() -> TableQueryBuilder {
        TableQueryBuilder { parts: Vec::new() }
    }

    fn build(&mut self, field: &Field<String>) -> Result<String, anyhow::Error> {
        println!("field: {:?}", field);
        self.parts.push("SELECT".to_string());
        let mut columns: Vec<String> = Vec::new();
        for item in &field.selection_set.items {
            println!("item: {:?}", item);
            match item {
                Selection::Field(field) => {
                    println!("field: {:?}", field);
                    columns.push(field.name.to_string());
                }
                _ => todo!(),
            }
        }
        println!("columns: {:?}", columns);
        self.parts.push(columns.join(", "));
        self.parts.push("FROM".to_string());
        self.parts.push(field.name.to_string());
        let query = self.parts.join(" ");
        println!("query: {}", query);
        Ok(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::QueryBuilder;

    #[tokio::test]
    async fn test_graphql_parser() -> Result<(), anyhow::Error> {
        let query = r#"
        {
          authors {
            id
            name
          }
        }
        "#
        .to_string();

        let doc = graphql_parser::parse_query::<String>(&query)?;
        let qb = &mut QueryBuilder::new(doc);
        let query = qb.build()?;
        assert_eq!(query, "SELECT id, name FROM authors");
        Ok(())
    }
}
