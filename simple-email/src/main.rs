use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {
    let email = EmailBuilder::new()
        .from(Mailbox::new("FROM_ADDRESS".to_owned()))
        .to(Mailbox::new("TO_ADRESS".to_owned()))
        .subject("Test mail from rust")
        .body("You'll love it.")
        .build()
        .unwrap();

    let cred = Credentials::new("USERNAME".to_owned(), "PASSWORD".to_owned());
    let mut client = SmtpClient::new_simple("smtp.live.com")
        .unwrap()
        .credentials(cred)
        .transport();

    let res = client.send(email.into());

    if res.is_ok() {
        println!("Email send ok");
    } else {
        println!("Email send faild: {:?}", res);
    }
}
