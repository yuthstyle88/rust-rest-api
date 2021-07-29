use serde::{Deserialize, Serialize};
use crate::error_handler::MyError;
use actix_web::HttpResponse;

pub type MyResult = Result<HttpResponse, MyError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Json<K, T>
    where K: Sized,
          T: Sized
{
    pub code: K,
    pub message: String,
    pub data: T,
}

impl<K, T> Json<K, T> {
    pub fn new(code: K, message: &str, data: T) -> Json<K, T> {
        Json {
            code,
            message: message.to_string(),
            data,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Code {
    // 2xx success
    Ok = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    MultiStatus = 207,
    AlreadyReported = 208,
    IMUsed = 226,

    // 3xx redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    MovedTemporarily = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    SwitchProxy = 306,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,

    // 4xx client errors
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    // 407 Proxy Authentication Required
    RequestTimeout = 408,
    Conflict = 409,
    // 410 Gone
    // 411 Length Required
    // 412 Precondition Failed
    // 413 Payload Too Large
    // 414 URI Too Long
    UnsupportedMediaType = 415,
    // 416 Range Not Satisfiable
    // 417 Expectation Failed\
    // 418 I'm a teapot (RFC 2324, RFC 7168)
    // 421 Misdirected Request
    // 422 Unprocessable Entity
    // 423 Locked
    // 424 Failed Dependency
    // 425 Too Early
    // 426 Upgrade Required
    // 428 Precondition Required
    // 429 Too Many Requests
    // 431 Request Header Fields Too Large
    UnavailableForLegalReasons = 451,

    // 5xx server errors
    InternalServerError = 500,
    NotImplemented = 501,
    // 502 Bad Gateway
    ServiceUnavailable = 503,
    // 505 HTTP Version Not Supported
    // 506 Variant Also Negotiates
    // 507 Insufficient Storage
    // 508 Loop Detected
    // 510 Not Extended
    // 511 Network Authentication Required

    // Unofficial codes
    Checkpoint = 103,
    ThisIsFine = 218,
    // 419 Page Expired
    // 420 Method Failure
    // 420 Enhance Your Calm
    // 430 Request Header Fields Too Large
    // 450 Blocked by Windows Parental Controls (Microsoft)
    // 498 Invalid Token (Esri)
    // 499 Token Required (Esri)
    // 509 Bandwidth Limit Exceeded
    // 526 Invalid SSL Certificate
    // 529 Site is overloaded
    // 530 Site is frozen
    // 598 (Informal convention) Network read timeout error

    // Internet Information Services
    // 440 Login Time-out
    // 449 Retry With
    // 451 Redirect

    // nginx
    // 444 No Response
    // 494 Request header too large
    // 495 SSL Certificate Error
    // 496 SSL Certificate Required
    // 497 HTTP Request Sent to HTTPS Port
    // 499 Client Closed Request

    // Cloudflare
    // 520 Web Server Returned an Unknown Error
    // 521 Web Server Is Down
    // 522 Connection Timed Out
    // 523 Origin Is Unreachable
    // 524 A Timeout Occurred
    // 525 SSL Handshake Failed
    // 526 Invalid SSL Certificate
    // 527 Railgun Error
    // 530 Error 530 is returned along with a 1xxx error

    // AWS Elastic Load Balancer
    // 460 Client closed the connection with the load balancer before the idle timeout period elapsed. Typically when client timeout is sooner than the Elastic Load Balancer's timeout.
    // 463 The load balancer received an X-Forwarded-For request header with more than 30 IP addresses.
    // 561 Unauthorized
}

impl Code {
    pub fn as_u16(&self) -> u16 {
        let me = self.clone();
        me as u16
    }
}