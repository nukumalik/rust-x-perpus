#![allow(dead_code, non_snake_case)]
pub mod CountryModel {
    use super::super::schema::CountrySchema;
    use crate::{
        modules::country::entity::CreateCountryPayload, utils::pagination_sort::PaginationSort,
    };
    use sqlx::{query, query_as, sqlite::SqliteQueryResult, Error, Pool, Sqlite};

    pub async fn select(
        pool: &Pool<Sqlite>,
        pagination_sort: Option<PaginationSort>,
        filter: Option<String>,
    ) -> Result<Vec<CountrySchema>, Error> {
        let mut sql = "SELECT * FROM countries".to_owned();
        if filter.is_some() {
            sql.push_str(format!(" WHERE {}", filter.unwrap()).as_str());
        }
        if pagination_sort.is_some() {
            sql.push_str(pagination_sort.unwrap().to_string().as_str());
        } else {
            sql.push_str(PaginationSort::default().to_string().as_str());
        }

        query_as::<Sqlite, CountrySchema>(sql.as_str())
            .fetch_all(pool)
            .await
    }

    pub async fn detail(pool: &Pool<Sqlite>, id: &String) -> Result<CountrySchema, Error> {
        query_as::<Sqlite, CountrySchema>("SELECT * FROM countries WHERE id=?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn insert(
        pool: &Pool<Sqlite>,
        country: &CountrySchema,
    ) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>(
            "INSERT INTO countries (id, name, code, created_at, updated_at) values (?, ?, ?, ?, ?)",
        )
        .bind(country.id().to_string())
        .bind(country.name().to_string())
        .bind(country.code().to_string())
        .bind(country.created_at().clone())
        .bind(country.updated_at().clone())
        .execute(pool)
        .await
    }

    pub async fn update(
        pool: &Pool<Sqlite>,
        id: &String,
        country: &CreateCountryPayload,
    ) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>("UPDATE countries SET name=?, code=? WHERE id=?")
            .bind(country.get_name().to_string())
            .bind(country.get_code().to_string())
            .bind(id)
            .execute(pool)
            .await
    }

    pub async fn delete(pool: &Pool<Sqlite>, id: &String) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>("DELETE FROM countries WHERE id=?")
            .bind(id)
            .execute(pool)
            .await
    }
}
