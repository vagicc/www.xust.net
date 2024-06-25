use crate::template::{to_html_base, to_html_single};
use handlebars::{to_json, Handlebars};
use serde_json::value::Map;
use warp::{Rejection, Reply};

type ResultWarp<T> = std::result::Result<T, Rejection>;

/* 响应/请求的返回 */
pub async fn index() -> ResultWarp<impl Reply> {
    log::debug!("[调试信息]访问了“/”");
    // log::warn!("[警告信息] warn");
    // log::info!("[提示信息] info");

    // use crate::models::article_model;
    // let list = article_model::get_new(19);

    let mut data = Map::new();

    // data.insert("list".to_string(), to_json(list));
    data.insert("seo_title".to_string(), to_json("技术派"));
    data.insert("seo_keyword".to_string(), to_json(" dd"));
    data.insert("seo_description".to_string(), to_json("dd"));

    // let html = to_html_single("index-single.html", data); //单页
    // let html = "欢迎来到elapse date".to_string();
    // let html = to_html_base("index.html", data);
    let html = to_html_base("home.html", data);

    Ok(warp::reply::html(html)) //直接返回html
                                // Err(warp::reject::not_found())   //错误的返回
}
