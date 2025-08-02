use dioxus::{html::{g::media, h1}, prelude::*};

const FAVICON: Asset = asset!("/assets/images/home_icon.svg");
const MAIN_CSS: Asset = asset!("/assets/style/main.css");
static FACEBOOK_LOGO: Asset = asset!(
    "/assets/images/facebook.png",
    ImageAssetOptions::new().with_avif()
);
static TWTITTER_LOGO: Asset = asset!("/assets/images/twitter.png", ImageAssetOptions::new().with_avif());
static YOUTUBE_LOGO: Asset = asset!("/assets/images/youtube.png", ImageAssetOptions::new().with_avif());
static LINKEDIN_LOGO: Asset = asset!("/assets/images/linkedin.png", ImageAssetOptions::new().with_avif());
static MAIL_ICON: Asset = asset!("/assets/images/gmail.png", ImageAssetOptions::new().with_avif());
static BANNER: Asset = asset!("/assets/images/banner.png", ImageAssetOptions::new().with_avif());

static LOCATION : Asset = asset!("/assets/images/location.svg");
static PHONE : Asset = asset!("/assets/images/phone.svg");
static MAIL : Asset = asset!("/assets/images/mail.svg");

static LOCATION_DARK : Asset = asset!("/assets/images/location_dark.svg");
static PHONE_DARK : Asset = asset!("/assets/images/phone_dark.svg");
static MAIL_DARK : Asset = asset!("/assets/images/mail_dark.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        NavBar {}
        Banner {}
        MainBody {}
        Footer {}
    }
}

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id : "navbar",
            div {
                id : "menu",
                img { src : FAVICON }
                a { href : "", "Home" }
                a { href : "", "About" }
                a { href : "", "Programs" }
                a { href : "", "Blogs" }
                a { href : "", "Contact" }
            }
            Socials {}
        }
    }
}

#[component]
fn Socials () -> Element {
    rsx! {
        div {
            class : "socials",
            img { src : FACEBOOK_LOGO }
            img { src : TWTITTER_LOGO }
            img { src : LINKEDIN_LOGO }
            img { src : YOUTUBE_LOGO }
            img { src : MAIL_ICON }
        }
    }
}

#[component]
fn Banner() -> Element {
    rsx! {
        div {
            id : "banner",
            background_image : "url({BANNER})",
            h1 { "Contact" }
            p { "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes."}
        }

    }
}

#[component]
fn MainBody() -> Element {
    rsx! {
        div {
            id : "main_body",
            h2 {
                "Find us here"
            }
            iframe {
                src : "https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d112099.64785301333!2d76.89595111640627!3d28.596356800000006!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x390d052368a55b9f%3A0x288f7e2c90845286!2sSmaaash%20Janakpuri!5e0!3m2!1sen!2sin!4v1754140481218!5m2!1sen!2sin",
                width : "800",
                height : "800",
                style : "border:0; theme:dark;",
                allowfullscreen : "",
                referrerpolicy : "no-referrer-when-downgrade"
            }
            h2 { "Get in Touch "}

            div {
                id : "contact_methods",
                MessageForm {}
                MoreWaysToContact {}
            }
        }
    }
}

#[component]
fn MessageForm() -> Element {
    rsx! {
        form {
            textarea {
                placeholder : "Your Message"
            }
            div {
                class : "row_arrangement",
                input {
                    type : "text",
                    placeholder : "Enter your name"
                }
                input {
                    type : "email",
                    placeholder : "Your Email"
                }
            }
            input {
                type : "text",
                placeholder : "Subject"
            }
            button {
                "SEND"
            }
        }
    }
}

#[component]
fn MoreWaysToContact() -> Element {
    rsx! {
        div {
            id : "more_ways_to_contact",
            ContactMethod {
                icon : LOCATION,
                icon_dark : LOCATION_DARK,
                text : "Buttonwood, California",
                subtext : "Rosemead, CA91770",
            }
            ContactMethod {
                icon : PHONE,
                icon_dark : PHONE_DARK,
                text : "+1 123 456 7890",
                subtext : "Mon - Fri, 9:00 to 18:00",
            }
            ContactMethod {
                icon : MAIL,
                icon_dark : MAIL_DARK,
                text : "support@colorlib.com",
                subtext : "Send us your query anytime!",
            }
        }
    }
}

#[component]
fn ContactMethod (icon : Asset, icon_dark : Asset, text : String, subtext : String) -> Element {
    rsx! {
        div {
            class : "contact_method",
            div {
                img { class : "light", src : icon }
                img { class : "dark", src : icon_dark }
            }
            div {
                display : "flex",
                flex_direction : "column",
                align_content : "center",
                justify_content : "center",
                gap : "2px",
                h4 {
                   "{text}"
                }
                p {
                    "{subtext}"
                }
            }
        }
    }
}

#[component]
fn Footer () -> Element {
    rsx! {
        div {
            id : "footer_section",
            div {
                class : "column",
                img {
                    src : FAVICON
                }
            }
            div {
                class : "column",
                h3 { "Quick Tips" }
                a { href : "", "Work" }
                a { href : "", "Service" }
                a { href : "", "Product" }
                a { href : "", "Tips & Tricks"}
            }
            div {
                class : "column",
                h3 { "Programs" }
                a { href : "", "Air Freight" }
                a { href : "", "Ocean Freight" }
                a { href : "", "Large Projects" }
            }
            div {
                class : "column",
                h3 { "Resources" }
                a { href : "", "FAQ" }
                a { href : "", "Submit Ticket" }
                a { href : "", "Contact Us" }
            }
            div {
                class : "column",
                h3 { "Newsletter" }
                p { "Subscribe newsletter to get updates" }
                Socials {}
            }
        }
    }
}
