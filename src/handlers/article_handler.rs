use crate::models::article_model;
use crate::template::to_html_base;
use handlebars::to_json;
use serde_json::value::Map;
use warp::{Rejection, Reply};

/// 详情
pub async fn detail(id: i32)-> std::result::Result<impl Reply, Rejection>  {
    let article = article_model::get_article(id);
    if article.is_none() {
        log::warn!("查无文章ID:{}", id);
        // return Err(warp::reject::not_found()); //错误的返回
    }

    let article = article.unwrap();
    let seo_title = article.title.clone();
    let seo_keyword = article.seo_keywords.clone();
    let seo_description = article.seo_description.clone();

    //取得文章详情
    use crate::models::article_content_model;
    let article_content = article_content_model::get_article_content(id);

    let mut data = Map::new();
    data.insert("article".to_string(), to_json(article));
    data.insert("article_content".to_string(), to_json(article_content));

    data.insert("seo_title".to_string(), to_json(&seo_title));
    data.insert("seo_keyword".to_string(), to_json(seo_keyword));
    data.insert("seo_description".to_string(), to_json(seo_description));

    // let html = to_html_single("video_detail_single.html", data);
    let html = to_html_base("article/detail.html", data);
    Ok(warp::reply::html(html))
}

/// 列表
pub async fn list(page: u32) -> std::result::Result<impl Reply, Rejection> {
    log::debug!("最新视频列表-分页");

    let per: u32 = 18; //每页总数
    let (count, list, pages) = article_model::article_list(Some(page), Some(per));

    let mut data = Map::new();
    data.insert("count".to_string(), to_json(count)); //
    data.insert("list".to_string(), to_json(list)); //
    data.insert("pages".to_string(), to_json(pages));

    // data.insert("seo_title".to_string(), to_json("jh"));
    // data.insert("seo_keyword".to_string(), to_json("dd"));
    // data.insert(
    //     "seo_description".to_string(),
    //     to_json("dd"),
    // );

    let html = to_html_base("article/list.html", data);
    Ok(warp::reply::html(html))
}
