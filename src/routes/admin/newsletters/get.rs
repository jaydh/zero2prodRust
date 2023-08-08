use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web_flash_messages::IncomingFlashMessages;

pub async fn publish_newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut msg_html = String::new();
    for m in flash_messages.iter() {
        msg_html.push_str(&format!("<p><i>{}</i></p>\n", m.content()))
    }
    let idempotency_key = uuid::Uuid::new_v4();
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Submit Newsletter</title>
</head>
<body>
    {msg_html}
    <form action="/admin/newsletters" method="post">
        <label>Title
            <input
                type="text"
                placeholder="Enter title"
                name="title"
            >
        </label>
        <br>
        <label>HTML
            <input
                type="text"
                placeholder="Enter html content"
                name="html_content"
            >
        </label>
        <br>
        <label>TEXT
            <input
                type="text"
                placeholder="Enter text content"
                name="text_content"
            >
        </label>
        <br>
        <input
            hidden
            type="text"
            name="idempotency_key"
            value="{idempotency_key}"
        >

        <button type="submit">Publish Newsletter</button>
    </form>
    <p><a href="/admin/dashboard">&lt;- Back</a></p>
</body>
</html>"#,
        )))
}
