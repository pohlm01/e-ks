use askama::Template;
use axum::response::IntoResponse;

use crate::{
    AppError, Context, DbConnection, HtmlTemplate,
    candidate_lists::structs::CandidateListDetail,
    filters,
    persons::{self, structs::Person},
    t,
};

use super::{CandidateList, ViewCandidateListPath, load_candidate_list};

#[derive(Template)]
#[template(path = "candidate_lists/view.html")]
struct CandidateListViewTemplate {
    details: CandidateListDetail,
    persons: Vec<Person>,
    max_candidates: usize,
}

pub(crate) async fn view_candidate_list(
    ViewCandidateListPath { id }: ViewCandidateListPath,
    context: Context,
    DbConnection(mut conn): DbConnection,
) -> Result<impl IntoResponse, AppError> {
    // TODO: determine max_candidates from political group configuration
    let max_candidates = 50;
    let details = load_candidate_list(&mut conn, &id, context.locale).await?;
    let persons = persons::repository::list_persons_not_on_candidate_list(&mut conn, &id)
        .await
        .map_err(AppError::from)?;

    Ok(HtmlTemplate(
        CandidateListViewTemplate {
            details,
            persons,
            max_candidates,
        },
        context,
    ))
}
