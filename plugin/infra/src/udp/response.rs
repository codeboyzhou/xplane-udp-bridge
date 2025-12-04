pub(crate) enum Status {
    Ok,
    BadRequest,
    InternalServerError,
}

pub(crate) struct UdpResponse {
    status: Status,
    body: String,
}

impl UdpResponse {
    const SEPARATOR: &'static str = "|";

    pub(crate) fn serialize(self) -> String {
        let status_str = match self.status {
            Status::Ok => ["200", "OK"].join(Self::SEPARATOR),
            Status::BadRequest => ["400", "Bad Request"].join(Self::SEPARATOR),
            Status::InternalServerError => ["500", "Internal Server Error"].join(Self::SEPARATOR),
        };
        [status_str, self.body].join(Self::SEPARATOR)
    }

    pub(crate) fn ok(body: String) -> Self {
        Self { status: Status::Ok, body }
    }

    pub(crate) fn error(status: Status, body: String) -> Self {
        Self { status, body }
    }
}
