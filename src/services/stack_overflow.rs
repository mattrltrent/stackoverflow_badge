use crate::utils::{self};

use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;

use reqwest;

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Period {
    Year,
    Quarter,
    Month,
    Week,
    Day,
}
#[derive(Deserialize)]
struct QueryParams {
    username: i64,
    period: Period,
}

#[derive(Debug, Deserialize)]
struct StackOverflowUser {
    badge_counts: BadgeCounts,
    reputation_change_year: i32,
    reputation_change_quarter: i32,
    reputation_change_month: i32,
    reputation_change_week: i32,
    reputation_change_day: i32,
    reputation: i32,
    display_name: String,
}

impl Period {
    fn to_str(&self) -> &str {
        match self {
            Period::Year => "this year",
            Period::Quarter => "this quarter",
            Period::Month => "this month",
            Period::Week => "this week",
            Period::Day => "today",
        }
    }
    // convert a period to a specific field on StackOverflowUser
    fn to_field(&self, user: &StackOverflowUser) -> i32 {
        match self {
            Period::Year => user.reputation_change_year,
            Period::Quarter => user.reputation_change_quarter,
            Period::Month => user.reputation_change_month,
            Period::Week => user.reputation_change_week,
            Period::Day => user.reputation_change_day,
        }
    }
}

#[derive(Debug, Deserialize)]
struct BadgeCounts {
    bronze: i32,
    silver: i32,
    gold: i32,
}

#[get("/stack_overflow")]
async fn gen_image(query: web::Query<QueryParams>) -> impl Responder {
    let user = match reqwest::Client::builder()
        .gzip(true)
        .build()
        .unwrap()
        .get(format!(
            "https://api.stackexchange.com/2.2/users/{}?&site=stackoverflow",
            &query.username
        ))
        .send()
        .await
    {
        Ok(response) => {
            if response.status() != 200 {
                return HttpResponse::InternalServerError().finish();
            } else {
                let json = match response.json::<serde_json::Value>().await {
                    Ok(json) => json,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                };
                let item = &json["items"][0];
                match serde_json::from_value::<StackOverflowUser>(item.clone()) {
                    Ok(user) => user,
                    Err(_) => return HttpResponse::InternalServerError().finish(),
                }
            }
        }
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };
    let svg_content = include_str!("../../assets/stack_overflow.svg"); // stack_overflow
    let svg_content = svg_content.replace(
        "{{rep}}",
        &format!(
            "{}",
            utils::numbers::format_number(*&user.reputation as i64)
        ),
    );
    let svg_content = svg_content.replace(
        "{{rep_change}}",
        &format!(
            "{}{} {}",
            if query.period.to_field(&user) >= 0 {
                "+"
            } else {
                ""
            },
            utils::numbers::format_number(query.period.to_field(&user) as i64),
            query.period.to_str()
        ),
    );
    let svg_content = svg_content.replace(
        "{{golds}}",
        &format!(
            "{} {}",
            utils::numbers::format_number(*&user.badge_counts.bronze as i64),
            if *&user.badge_counts.bronze == 1 {
                "bronze"
            } else {
                "bronzes"
            }
        ),
    );
    let svg_content = svg_content.replace(
        "{{silvers}}",
        &format!(
            "{} {}",
            utils::numbers::format_number(*&user.badge_counts.silver as i64),
            if *&user.badge_counts.silver == 1 {
                "silver"
            } else {
                "silvers"
            }
        ),
    );
    let svg_content = svg_content.replace(
        "{{bronzes}}",
        &format!(
            "{} {}",
            utils::numbers::format_number(*&user.badge_counts.gold as i64),
            if *&user.badge_counts.gold == 1 {
                "gold"
            } else {
                "golds"
            }
        ),
    );
    HttpResponse::Ok()
        .content_type("image/svg+xml")
        .body(svg_content)
}
