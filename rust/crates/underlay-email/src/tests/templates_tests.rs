use super::*;

fn create_test_engine() -> EmailTemplateEngine {
    let mut tera = tera::Tera::default();

    // Add a simple test template
    tera.add_raw_template("test.html", "Hello {{ name }}! Your code is {{ code }}.")
        .unwrap();

    // Add a template that extends base
    tera.add_raw_template(
        "base.html",
        r#"<!DOCTYPE html>
<html>
<head><title>{{ app_name }}</title></head>
<body>{% block content %}{% endblock %}</body>
</html>"#,
    )
    .unwrap();

    tera.add_raw_template(
        "auth/email_totp.html",
        r#"{% extends "base.html" %}
{% block content %}
<p>Your verification code is: <strong>{{ code }}</strong></p>
<p>This code expires in {{ expiry_minutes }} minutes.</p>
{% endblock %}"#,
    )
    .unwrap();

    EmailTemplateEngine::from_tera(tera)
}

#[test]
fn test_render_simple_template() {
    let engine = create_test_engine();

    let mut ctx = EmailContext::new();
    ctx.set("name", "Alice");
    ctx.set("code", "123456");

    let result = engine.render("test.html", &ctx).unwrap();
    assert_eq!(result, "Hello Alice! Your code is 123456.");
}

#[test]
fn test_render_extended_template() {
    let engine = create_test_engine();

    let mut ctx =
        EmailContext::with_app_info("TestApp", "https://test.example.com", "support@example.com");
    ctx.set("code", "654321");
    ctx.set("expiry_minutes", 10);

    let result = engine.render("auth/email_totp.html", &ctx).unwrap();
    assert!(result.contains("<title>TestApp</title>"));
    assert!(result.contains("<strong>654321</strong>"));
    assert!(result.contains("10 minutes"));
}

#[test]
fn test_template_not_found() {
    let engine = create_test_engine();
    let ctx = EmailContext::new();

    let result = engine.render("nonexistent.html", &ctx);
    assert!(result.is_err());
}

#[test]
fn test_has_template() {
    let engine = create_test_engine();

    assert!(engine.has_template("test.html"));
    assert!(engine.has_template("auth/email_totp.html"));
    assert!(!engine.has_template("nonexistent.html"));
}

#[test]
fn test_template_names() {
    let engine = create_test_engine();
    let names = engine.template_names();

    assert!(names.contains(&"test.html"));
    assert!(names.contains(&"base.html"));
    assert!(names.contains(&"auth/email_totp.html"));
}

#[test]
fn test_context_with_app_info() {
    let ctx = EmailContext::with_app_info("MyApp", "https://myapp.com", "help@myapp.com");

    // Verify the inner context has the expected keys by serializing
    // We can't directly access the values, but we can verify it works in rendering
    let engine = create_test_engine();
    let result = engine
        .render("auth/email_totp.html", &{
            let mut c = ctx;
            c.set("code", "000000");
            c.set("expiry_minutes", 5);
            c
        })
        .unwrap();

    assert!(result.contains("MyApp"));
}
