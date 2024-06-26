use crate::db::get_connection;
use crate::schema::books;
use crate::schema::books::dsl::*;
use chrono::NaiveDateTime;
use diesel::data_types::Cents;
use diesel::prelude::*;
// use serde::{Deserialize};
use serde::ser::{Serialize, SerializeStruct, Serializer};

/* 表查询插入结构体(Insertable：插入，Queryable：查询) */
#[derive(Debug, Clone, Queryable)]
pub struct Books {
    pub id: i32,
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub front_cover: Option<String>,
    pub price: Option<Cents>, // 要单独增加Serialize对应Cents
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub collect: Option<i64>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
// 手动国添加上Serialize特征: Cents与BigDecimal无特征,所以手动添加
impl Serialize for Books {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let num_elements = std::mem::size_of::<Books>() / std::mem::size_of::<i32>(); //取得结构体的元素个数
        let mut s = serializer.serialize_struct("Person", num_elements).unwrap();
        s.serialize_field("id", &self.id).unwrap();
        s.serialize_field("name", &self.name).unwrap();
        s.serialize_field("author", &self.author).unwrap();
        s.serialize_field("publisher", &self.publisher).unwrap();
        s.serialize_field("front_cover", &self.front_cover).unwrap();

        let price_temp = (self.price.unwrap().0 as f64) / 100.;
        s.serialize_field("price", &price_temp).unwrap();

        s.serialize_field("category_id", &self.category_id).unwrap();
        s.serialize_field("category", &self.category).unwrap();
        s.serialize_field("description", &self.description).unwrap();
        s.serialize_field("finish", &self.finish).unwrap();
        s.serialize_field("collect", &self.collect).unwrap();
        s.serialize_field("seo_title", &self.seo_title).unwrap();
        s.serialize_field("seo_keywords", &self.seo_keywords)
            .unwrap();
        s.serialize_field("seo_description", &self.seo_description)
            .unwrap();
        s.serialize_field("create_id", &self.create_id).unwrap();
        s.serialize_field("create_time", &self.create_time).unwrap();

        s.end()
    }
}

#[derive(Debug, Clone, Insertable, AsChangeset)]
#[diesel(table_name = books)]
pub struct NewBooks {
    pub name: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub front_cover: Option<String>,
    pub price: Option<Cents>, // 要单独增加Serialize对应Cents
    pub category_id: Option<i32>,
    pub category: Option<String>,
    pub description: Option<String>,
    pub finish: Option<bool>,
    pub collect: Option<i64>,
    pub seo_title: Option<String>,
    pub seo_keywords: Option<String>,
    pub seo_description: Option<String>,
    pub create_id: Option<i32>,
    pub create_time: Option<NaiveDateTime>,
}
impl NewBooks {
    pub fn insert(&self) -> i32 {
        let mut connection = get_connection();
        let query = diesel::insert_into(books).values(self).returning(id);
        log::debug!(
            "books表插入数据SQL：{:?}",
            diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
        );
        let result = query.get_result::<i32>(&mut connection);
        match result {
            Ok(insert_id) => {
                log::debug!("插入成功，ID为：{}", insert_id);
                insert_id
            }
            Err(err) => {
                //value too long for type character varying(255) 字段太短，插入内容太长
                log::error!("插入数据失败了：{}", err);
                0
            }
        }
    }
}

// GET查询条件
#[derive(Debug, Clone, serde_derive::Deserialize, serde_derive::Serialize)]
pub struct GetQuery {
    pub book_name: Option<String>,   //书名
    pub book_author: Option<String>, //作者
    pub c_id: Option<i32>,           //分类ID
    pub finish:Option<i8>,    //是否已完本，0为未选择，1为完本，2为未结
}

/// 取得列表数据
/// page: Option<u32>  第几页
/// per: Option<u32>   每页多少条数据,默认为50
/// 返回（总条数：i64,数据数组，分页html)
pub fn list_page(
    page: Option<u32>,
    per: Option<u32>,
    whe: Option<GetQuery>,
) -> (i64, Vec<Books>, String) {
    let mut limit: i64 = 50; //每页取几条数据
    let mut offset: i64 = 0; //从第0条开始

    if !per.is_none() {
        limit = per.unwrap() as i64;
    }

    if !page.is_none() && page.unwrap() > 1 {
        offset = ((page.unwrap() as i64) - 1) * limit;
    }

    let mut query = books.into_boxed();
    let mut query_count = books.into_boxed();
    //可变的查询条件以上面结合下面的写法
    if let Some(params) = whe {
        if let Some(book_name) = params.book_name.filter(|bn| !bn.is_empty()) {
            let name_like = format!("%{}%", book_name);
            query = query.filter(name.like(name_like.clone()));
            query_count = query_count.filter(name.like(name_like));
        }
        // if let Some(book_author) = params.book_author {
        //     if !book_author.is_empty() {
        //         query = query.filter(author.like(book_author.clone()));
        //         query_count = query_count.filter(author.like(book_author));
        //     }
        // }
        ////简洁写成如下
        if let Some(book_author) = params.book_author.filter(|s| !s.is_empty()) {
            let author_like = format!("%{}%", book_author);
            query = query.filter(author.like(author_like.clone()));
            query_count = query_count.filter(author.like(author_like));
        }
        if let Some(c_id) = params.c_id.filter(|c| *c > 0) {
            // 这里还要判断是否ID大于0？
            query = query.filter(category_id.eq(c_id));
            query_count = query_count.filter(category_id.eq(c_id));
        }
        //是否已完本
        if let Some(finish_whe) = params.finish.filter(|f| *f > 0) {
            let finish_whe = if finish_whe == 1 { true } else { false };
            query = query.filter(finish.eq(finish_whe));
            query_count = query_count.filter(finish.eq(finish_whe));
        }
    }

    let query_count = query_count.count();
    log::error!(
        "books分页数量查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query_count).to_string()
    );

    let mut conn = get_connection();
    let count: i64 = query_count
        .get_result(&mut conn)
        .expect("books分页数量查询出错"); //查询总条数

    let mut pages = String::new();
    let data_null: Vec<Books> = Vec::new();
    if count <= 0 {
        return (count, data_null, pages);
    }

    let query = query
        .order_by(id.desc())
        .limit(limit) //取10条数据
        .offset(offset); //从第0条开始;
    log::error!(
        "books分页查询SQL：{:#?}",
        diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string()
    );

    let list = query.get_results::<Books>(&mut conn).unwrap_or(data_null);

    // let page = page.unwrap_or(1);
    pages = crate::pager::default_full("book/list", count, page.unwrap_or(1), limit as u32);

    (count, list, pages)
}

///通过书名取得书籍信息
pub fn get_book(book_name: String) -> Option<Books> {
    let query = books.filter(name.eq(book_name));
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("get_book查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Books>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("get_book查无数据：{}", err);
            None
        }
    }
}

pub fn find_book(book_id: i32) -> Option<Books> {
    let query = books.find(book_id);
    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string();
    log::debug!("find_book查询SQL：{:?}", sql);
    let mut connection = get_connection();
    let result = query.first::<Books>(&mut connection);
    match result {
        Ok(row) => Some(row),
        Err(err) => {
            log::debug!("find_book查无数据：{}", err);
            None
        }
    }
}
