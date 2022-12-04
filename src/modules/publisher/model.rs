#![allow(dead_code, non_snake_case)]
pub mod PublisherModel {
    use super::super::{entity::CreatePublisherPayload, schema::PublisherSchema};
    use crate::utils::pagination_sort::PaginationSort;
    use sqlx::{query, query_as, sqlite::SqliteQueryResult, Error, Pool, Sqlite};

    pub async fn select(
        pool: &Pool<Sqlite>,
        pagination_sort: Option<PaginationSort>,
        filter: Option<String>,
    ) -> Result<Vec<PublisherSchema>, Error> {
        let mut sql = "SELECT * FROM publishers ".to_owned();
        if filter.is_some() {
            sql.push_str(format!("WHERE {} ", filter.unwrap()).as_str());
        }
        if pagination_sort.is_some() {
            sql.push_str(pagination_sort.unwrap().to_string().as_str());
        } else {
            sql.push_str(PaginationSort::default().to_string().as_str());
        }

        query_as::<Sqlite, PublisherSchema>(sql.as_str())
            .fetch_all(pool)
            .await
    }

    pub async fn detail(pool: &Pool<Sqlite>, id: &String) -> Result<PublisherSchema, Error> {
        query_as::<Sqlite, PublisherSchema>("SELECT * FROM publishers WHERE id=?")
            .bind(id)
            .fetch_one(pool)
            .await
    }

    pub async fn insert(
        pool: &Pool<Sqlite>,
        publisher: &PublisherSchema,
    ) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>(
            "INSERT INTO publishers (id, name, country_id, province_id, city_id, district_id, street, zip_code, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(publisher.id().to_string())
        .bind(publisher.name().to_string())
        .bind(publisher.country_id().to_string())
        .bind(publisher.province_id().to_string())
        .bind(publisher.city_id().to_string())
        .bind(publisher.district_id().to_string())
        .bind(publisher.street().to_string())
        .bind(publisher.zip_code().to_string())
        .bind(publisher.created_at().clone())
        .bind(publisher.updated_at().clone())
        .execute(pool)
        .await
    }

    pub async fn update(
        pool: &Pool<Sqlite>,
        id: &String,
        publisher: &CreatePublisherPayload,
    ) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>("UPDATE publishers SET name=?, country_id=?, province_id=?, city_id=?, district_id=?, street=?, zip_code=? WHERE id=?")
            .bind(publisher.get_name().to_string())
            .bind(publisher.get_country_id().to_string())
            .bind(publisher.get_province_id().to_string())
            .bind(publisher.get_city_id().to_string())
            .bind(publisher.get_district_id().to_string())
            .bind(publisher.get_street().to_string())
            .bind(publisher.get_zip_code().to_string())
            .bind(id)
            .execute(pool)
            .await
    }

    pub async fn delete(pool: &Pool<Sqlite>, id: &String) -> Result<SqliteQueryResult, Error> {
        query::<Sqlite>("DELETE FROM publishers WHERE id=?")
            .bind(id)
            .execute(pool)
            .await
    }
}
