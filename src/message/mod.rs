mod answer;
mod header;
mod name;
mod question;

pub use answer::Answer;
pub use header::Header;
pub use question::Question;

use anyhow::Result;
use header::HEADER_SIZE;

#[derive(Debug, PartialEq, Eq)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let header = Header::from_bytes(bytes)?;

        let mut offset = HEADER_SIZE;
        let mut questions = Vec::new();
        for _ in 0..header.qdcount {
            let (question, new_offset) = Question::from_bytes(bytes, offset)?;
            questions.push(question);
            offset = new_offset;
        }

        let mut answers = Vec::new();
        for _ in 0..header.ancount {
            let (answer, new_offset) = Answer::from_bytes(bytes, offset)?;
            answers.push(answer);
            offset = new_offset;
        }

        Ok(Message {
            header,
            questions,
            answers,
        })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.header.to_bytes().to_vec();
        for question in &self.questions {
            bytes.extend(question.to_bytes());
        }
        for answer in &self.answers {
            bytes.extend(answer.to_bytes());
        }
        bytes
    }
}
