pub enum Method {
    GET(String),
    DELETE(u64),
    POST,
    PUT,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
    PATCH,
}
