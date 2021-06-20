fn main() {
    windows::build!(
        Windows::Foundation::Collections::IVector,
        Windows::Foundation::{IAsyncOperationWithProgress, Uri},
        Windows::Win32::WindowsAndMessaging::MessageBoxA,
        Windows::Web::Syndication::{
            ISyndicationText, RetrievalProgress, SyndicationClient, SyndicationFeed, SyndicationItem,
        },
    );
}