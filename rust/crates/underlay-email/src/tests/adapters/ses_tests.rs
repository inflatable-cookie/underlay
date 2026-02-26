use super::*;
use crate::types::EmailAddress;

#[test]
fn test_build_content_text_only() {
    let email = Email::builder()
        .from(EmailAddress::new("sender@example.com").unwrap())
        .to(EmailAddress::new("recipient@example.com").unwrap())
        .subject("Test Subject")
        .text_body("Hello, World!")
        .build()
        .unwrap();

    let content = SesAdapter::build_content(&email).unwrap();
    assert!(content.simple().is_some());
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
