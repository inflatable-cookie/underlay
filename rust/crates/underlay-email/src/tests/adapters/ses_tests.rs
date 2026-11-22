use super::*;
use crate::types::EmailAddress;

#[test]
fn test_build_content_text_only() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .header("X-Trace", "ses-test")
        .text_body("Hello, World!")
        .build()
        .unwrap();

    let content = SesAdapter::build_content(&email).unwrap();
    let headers = content.simple().unwrap().headers();
    assert_eq!(headers.len(), 1);
    assert_eq!(headers[0].name(), "X-Trace");
    assert_eq!(headers[0].value(), "ses-test");
}

#[test]
fn test_build_content_html_only() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .html_body("<h1>Hello, World!</h1>")
        .build()
        .unwrap();

    let content = SesAdapter::build_content(&email).unwrap();
    assert!(content.simple().is_some());
}

#[test]
fn test_build_content_multipart() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .text_body("Hello, World!")
        .html_body("<h1>Hello, World!</h1>")
        .build()
        .unwrap();

    let content = SesAdapter::build_content(&email).unwrap();
    assert!(content.simple().is_some());
}

#[test]
fn test_build_destination() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("to1@example.com").unwrap())
        .to(EmailAddress::new("to2@example.com").unwrap())
        .cc(EmailAddress::new("cc@example.com").unwrap())
        .bcc(EmailAddress::new("bcc@example.com").unwrap())
        .subject("Test")
        .text_body("Hello")
        .build()
        .unwrap();

    let destination = SesAdapter::build_destination(&email);
    assert_eq!(destination.to_addresses().len(), 2);
    assert_eq!(destination.cc_addresses().len(), 1);
    assert_eq!(destination.bcc_addresses().len(), 1);
}

#[test]
fn test_build_content_rejects_missing_text_and_html_bodies() {
    let mut email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .text_body("placeholder")
        .build()
        .unwrap();
    email.text_body = None;
    email.html_body = None;

    assert!(matches!(
        SesAdapter::build_content(&email),
        Err(EmailError::MissingField("text_body or html_body"))
    ));
}
