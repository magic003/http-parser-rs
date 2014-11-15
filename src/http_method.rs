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
