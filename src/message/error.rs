use thiserror::Error;

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("DNS header must be at least {0} bytes long")]
    HeaderTooShort(usize),

    #[error("unexpected end of buffer while parsing a name")]
    UnexpectedEndOfName,

    #[error("unexpected end of buffer while parsing a compression pointer")]
    UnexpectedEndOfPointer,

    #[error("label exceeds the bounds of the buffer")]
    LabelExceedsBuffer,

    #[error("question section is truncated")]
    QuestionSectionTruncated,

    #[error("answer record is truncated")]
    AnswerRecordTruncated,

    #[error("answer record's RDATA is truncated")]
    AnswerRdataTruncated,
}

pub type Result<T> = std::result::Result<T, MessageError>;
