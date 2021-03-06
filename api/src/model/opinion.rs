use crate::{error, Db};

pub struct Opinion {
    pub id: i64,
    pub source_id: i64,
    pub user_id: i64,
    pub position: bool,
    pub body: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct CreateOpinion {
    pub source_id: i64,
    pub user_id: i64,
    pub position: bool,
    pub body: String,
}

/// Find a opinion by id
pub async fn find(db: &Db, id: &i64) -> error::Result<Opinion> {
    let opinion = sqlx::query_as!(Opinion, "select * from opinion where id = $1", id)
        .fetch_one(db)
        .await?;

    Ok(opinion)
}

/// Find opinions by source
pub async fn find_by_source(db: &Db, source_id: &i64) -> error::Result<Vec<Opinion>> {
    let opinions = sqlx::query_as!(
        Opinion,
        "
        select opinion.* from opinion
        left join vote on opinion.id = vote.opinion_id
        where opinion.source_id = $1
        group by opinion.id
        order by count(vote.*) desc
        ",
        source_id
    )
    .fetch_all(db)
    .await?;

    Ok(opinions)
}

/// Create a new opinion
pub async fn create(db: &Db, input: CreateOpinion) -> error::Result<Opinion> {
    let opinion = sqlx::query_as!(
        Opinion,
        "insert into opinion (source_id, user_id, position, body) values ($1, $2, $3, $4) returning *",
        input.source_id,
        input.user_id,
        input.position,
        input.body
    )
    .fetch_one(db)
    .await?;

    Ok(opinion)
}
