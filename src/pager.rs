
pub fn default_full(path: &str, count: i64, page: u32, per: u32) -> String {
    let base_url = crate::common::get_env("BASE_URL");
    let page_url = format!("{}{}", base_url, path);

    let count_page = ((count as f32) / (per as f32)).ceil() as u32; //总页数

    /*
    <li><a href="#" class="page">&laquo;</a></li>
    <li><a href="#" class="button">Prev</a></li>
    <li><a href="#" class="page">1</a></li>
    <li><a href="#" class="page">2</a></li>
    <li><a href="#" class="page">3</a></li>
    <li><span>&hellip;</span></li>
    <li><span class="page active">8</span></li>
    <li><a href="#" class="page">9</a></li>
    <li><a href="#" class="page">10</a></li>
    <li><a href="#" class="button">Next</a></li>
    <li><a href="#" class="page">&raquo;</a></li>
     */
    let mut show_left = 2; //左边显示的数字数
    let mut show_right = 2; //右边显示的数字数
    let mut page_html = String::new();

    //首页
    if page > 2 {
        page_html = format!(
            r#"{}
            <li><a href="{}" class="page">&laquo;</a></li>
            "#,
            page_html,
            page_url,
        );
    }

    // 是否有上一页
    if page > 1 {
        // 当前页的上一页,非最前字字上一页
        page_html = format!(
            r#"{}
            <li><a href="{}/{}" class="button">Prev</a></li>
            "#,
            page_html,
            page_url,
            page - 1
        );

        // 左边的页数数字,如左边少页数,则在右边补
        if page - show_left <= 0 {
            show_right += show_left;
            show_left = page - 1;
            show_right -= show_left;
        }
        while show_left > 0 {
            page_html = format!(
                r#"
            {}
            <li><a href="{}/{}" class="page">{2}</a></li>
            "#,
                page_html,
                page_url,
                page - show_left
            );
            show_left -= 1;
        }

        // 上一页数字
        // page_html = format!(
        //     r#"
        // {}
        // <span><a href="{}/{}">{2}</a></span>
        // "#,
        //     page_html,
        //     page_url,
        //     page - 1
        // );
    }

    // 当前页
    page_html = format!(
        r#"
    {}
    <li><span class="page active">{}</span></li>
    "#,
        page_html, page
    );

    //是否有下一页
    if page < count_page {
        // 下一页数字
        // page_html = format!(
        //     r#"
        // {}
        // <span><a href="{}/{}">{2}</a></span>
        // "#,
        //     page_html,
        //     page_url,
        //     page + 1
        // );

        // 输出右边数字
        let mut t = 1;
        loop {
            if page + show_right > count_page {
                show_right = count_page - page;
            }

            if t > show_right {
                break;
            }

            page_html = format!(
                r#"
            {}
            <li><a href="{}/{}" class="page">{2}</a></li>
            "#,
                page_html,
                page_url,
                page + t
            );

            t += 1;
            // show_right -= 1;
            // break;
        }

        // 下一页
        page_html = format!(
            r#"{}
            <li><a href="{}/{}" class="button">Next</a></li>
            "#,
            page_html,
            page_url,
            page + 1
        );
    }

    //末页
    if page + 1 < count_page {
        page_html = format!(
            r#"{}
            <li><a href="{}/{}" class="page">&raquo;</a></li>
            "#,
            page_html,
            page_url,
            count_page
        );
    }

    page_html
}
