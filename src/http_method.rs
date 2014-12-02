#[deriving(PartialEq, Eq )]
pub enum HttpMethod {
    Delete,
    Get,
    Head,
    Post,
    Put,
    // pathological
    Connect,
    Options,
    Trace,
    // webdav
    Copy,
    Lock,
    MKCol,
    Move,
    PropFind,
    PropPatch,
    Search,
    Unlock,
    // subversion
    Report,
    MKActivity,
    Checkout,
    Merge,
    // upnp
    MSearch,
    Notify,
    Subscribe,
    Unsubscribe,
    // RFC-5789
    Patch,
    Purge,
    // CalDAV
    MKCalendar,
}

impl ToString for HttpMethod {
    fn to_string(&self) -> String {
        match *self {
            HttpMethod::Delete      => "DELETE".to_string(),
            HttpMethod::Get         => "GET".to_string(),
            HttpMethod::Head        => "HEAD".to_string(),
            HttpMethod::Post        => "POST".to_string(),
            HttpMethod::Put         => "Put".to_string(),
            HttpMethod::Connect     => "CONNECT".to_string(),
            HttpMethod::Options     => "OPTIONS".to_string(),
            HttpMethod::Trace       => "TRACE".to_string(),
            HttpMethod::Copy        => "COPY".to_string(),
            HttpMethod::Lock        => "LOCK".to_string(),
            HttpMethod::MKCol       => "MKCOL".to_string(),
            HttpMethod::Move        => "MOVE".to_string(),
            HttpMethod::PropFind    => "PROPFIND".to_string(),
            HttpMethod::PropPatch   => "PROPPATCH".to_string(),
            HttpMethod::Search      => "SEARCH".to_string(),
            HttpMethod::Unlock      => "UNLOCK".to_string(),
            HttpMethod::Report      => "REPORT".to_string(),
            HttpMethod::MKActivity  => "MKACTIVITY".to_string(),
            HttpMethod::Checkout    => "CHECKOUT".to_string(),
            HttpMethod::Merge       => "MERGE".to_string(),
            HttpMethod::MSearch     => "M-SEARCH".to_string(),
            HttpMethod::Notify      => "NOTIFY".to_string(),
            HttpMethod::Subscribe   => "SUBSCRIBE".to_string(),
            HttpMethod::Unsubscribe => "UNSUBSCRIBE".to_string(),
            HttpMethod::Patch       => "PATCH".to_string(),
            HttpMethod::Purge       => "PURGE".to_string(),
            HttpMethod::MKCalendar  => "MKCALENDAR".to_string(),
        }
    }
}
