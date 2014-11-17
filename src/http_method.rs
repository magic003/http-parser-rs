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
            Delete      => "DELETE".to_string(),
            Get         => "GET".to_string(),
            Head        => "HEAD".to_string(),
            Post        => "POST".to_string(),
            Put         => "Put".to_string(),
            Connect     => "CONNECT".to_string(),
            Options     => "OPTIONS".to_string(),
            Trace       => "TRACE".to_string(),
            Copy        => "COPY".to_string(),
            Lock        => "LOCK".to_string(),
            MKCol       => "MKCOL".to_string(),
            Move        => "MOVE".to_string(),
            PropFind    => "PROPFIND".to_string(),
            PropPatch   => "PROPPATCH".to_string(),
            Search      => "SEARCH".to_string(),
            Unlock      => "UNLOCK".to_string(),
            Report      => "REPORT".to_string(),
            MKActivity  => "MKACTIVITY".to_string(),
            Checkout    => "CHECKOUT".to_string(),
            Merge       => "MERGE".to_string(),
            MSearch     => "M-SEARCH".to_string(),
            Notify      => "NOTIFY".to_string(),
            Subscribe   => "SUBSCRIBE".to_string(),
            Unsubscribe => "UNSUBSCRIBE".to_string(),
            Patch       => "PATCH".to_string(),
            Purge       => "PURGE".to_string(),
            MKCalendar  => "MKCALENDAR".to_string(),
        }
    }
}
