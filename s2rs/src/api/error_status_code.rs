
pub enum ErrorStatusCode {
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    
}
//     /// 408 Request Timeout
//     /// [[RFC7231, Section 6.5.7](https://tools.ietf.org/html/rfc7231#section-6.5.7)]
//     (408, REQUEST_TIMEOUT, "Request Timeout");
//     /// 409 Conflict
//     /// [[RFC7231, Section 6.5.8](https://tools.ietf.org/html/rfc7231#section-6.5.8)]
//     (409, CONFLICT, "Conflict");
//     /// 410 Gone
//     /// [[RFC7231, Section 6.5.9](https://tools.ietf.org/html/rfc7231#section-6.5.9)]
//     (410, GONE, "Gone");
//     /// 411 Length Required
//     /// [[RFC7231, Section 6.5.10](https://tools.ietf.org/html/rfc7231#section-6.5.10)]
//     (411, LENGTH_REQUIRED, "Length Required");
//     /// 412 Precondition Failed
//     /// [[RFC7232, Section 4.2](https://tools.ietf.org/html/rfc7232#section-4.2)]
//     (412, PRECONDITION_FAILED, "Precondition Failed");
//     /// 413 Payload Too Large
//     /// [[RFC7231, Section 6.5.11](https://tools.ietf.org/html/rfc7231#section-6.5.11)]
//     (413, PAYLOAD_TOO_LARGE, "Payload Too Large");
//     /// 414 URI Too Long
//     /// [[RFC7231, Section 6.5.12](https://tools.ietf.org/html/rfc7231#section-6.5.12)]
//     (414, URI_TOO_LONG, "URI Too Long");
//     /// 415 Unsupported Media Type
//     /// [[RFC7231, Section 6.5.13](https://tools.ietf.org/html/rfc7231#section-6.5.13)]
//     (415, UNSUPPORTED_MEDIA_TYPE, "Unsupported Media Type");
//     /// 416 Range Not Satisfiable
//     /// [[RFC7233, Section 4.4](https://tools.ietf.org/html/rfc7233#section-4.4)]
//     (416, RANGE_NOT_SATISFIABLE, "Range Not Satisfiable");
//     /// 417 Expectation Failed
//     /// [[RFC7231, Section 6.5.14](https://tools.ietf.org/html/rfc7231#section-6.5.14)]
//     (417, EXPECTATION_FAILED, "Expectation Failed");
//     /// 418 I'm a teapot
//     /// [curiously not registered by IANA but [RFC2324](https://tools.ietf.org/html/rfc2324)]
//     (418, IM_A_TEAPOT, "I'm a teapot");

//     /// 421 Misdirected Request
//     /// [RFC7540, Section 9.1.2](http://tools.ietf.org/html/rfc7540#section-9.1.2)
//     (421, MISDIRECTED_REQUEST, "Misdirected Request");
//     /// 422 Unprocessable Entity
//     /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
//     (422, UNPROCESSABLE_ENTITY, "Unprocessable Entity");
//     /// 423 Locked
//     /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
//     (423, LOCKED, "Locked");
//     /// 424 Failed Dependency
//     /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
//     (424, FAILED_DEPENDENCY, "Failed Dependency");

//     /// 426 Upgrade Required
//     /// [[RFC7231, Section 6.5.15](https://tools.ietf.org/html/rfc7231#section-6.5.15)]
//     (426, UPGRADE_REQUIRED, "Upgrade Required");

//     /// 428 Precondition Required
//     /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
//     (428, PRECONDITION_REQUIRED, "Precondition Required");
//     /// 429 Too Many Requests
//     /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
//     (429, TOO_MANY_REQUESTS, "Too Many Requests");

//     /// 431 Request Header Fields Too Large
//     /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
//     (431, REQUEST_HEADER_FIELDS_TOO_LARGE, "Request Header Fields Too Large");

//     /// 451 Unavailable For Legal Reasons
//     /// [[RFC7725](http://tools.ietf.org/html/rfc7725)]
//     (451, UNAVAILABLE_FOR_LEGAL_REASONS, "Unavailable For Legal Reasons");

//     /// 500 Internal Server Error
//     /// [[RFC7231, Section 6.6.1](https://tools.ietf.org/html/rfc7231#section-6.6.1)]
//     (500, INTERNAL_SERVER_ERROR, "Internal Server Error");
//     /// 501 Not Implemented
//     /// [[RFC7231, Section 6.6.2](https://tools.ietf.org/html/rfc7231#section-6.6.2)]
//     (501, NOT_IMPLEMENTED, "Not Implemented");
//     /// 502 Bad Gateway
//     /// [[RFC7231, Section 6.6.3](https://tools.ietf.org/html/rfc7231#section-6.6.3)]
//     (502, BAD_GATEWAY, "Bad Gateway");
//     /// 503 Service Unavailable
//     /// [[RFC7231, Section 6.6.4](https://tools.ietf.org/html/rfc7231#section-6.6.4)]
//     (503, SERVICE_UNAVAILABLE, "Service Unavailable");
//     /// 504 Gateway Timeout
//     /// [[RFC7231, Section 6.6.5](https://tools.ietf.org/html/rfc7231#section-6.6.5)]
//     (504, GATEWAY_TIMEOUT, "Gateway Timeout");
//     /// 505 HTTP Version Not Supported
//     /// [[RFC7231, Section 6.6.6](https://tools.ietf.org/html/rfc7231#section-6.6.6)]
//     (505, HTTP_VERSION_NOT_SUPPORTED, "HTTP Version Not Supported");
//     /// 506 Variant Also Negotiates
//     /// [[RFC2295](https://tools.ietf.org/html/rfc2295)]
//     (506, VARIANT_ALSO_NEGOTIATES, "Variant Also Negotiates");
//     /// 507 Insufficient Storage
//     /// [[RFC4918](https://tools.ietf.org/html/rfc4918)]
//     (507, INSUFFICIENT_STORAGE, "Insufficient Storage");
//     /// 508 Loop Detected
//     /// [[RFC5842](https://tools.ietf.org/html/rfc5842)]
//     (508, LOOP_DETECTED, "Loop Detected");

//     /// 510 Not Extended
//     /// [[RFC2774](https://tools.ietf.org/html/rfc2774)]
//     (510, NOT_EXTENDED, "Not Extended");
//     /// 511 Network Authentication Required
//     /// [[RFC6585](https://tools.ietf.org/html/rfc6585)]
//     (511, NETWORK_AUTHENTICATION_REQUIRED, "Network Authentication Required");