// src\main.rs
use bindings::{ 
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::WindowsAndMessaging::*,
};

fn main() -> windows::Result<()> {
    let uri = Uri::CreateUri("https://blogs.windows.com/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;

    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }
    unsafe {
        MessageBoxA(None, "Text", "Caption", MESSAGEBOX_STYLE::MB_OK);
    }
    Ok(())
}