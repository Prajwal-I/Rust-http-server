use super::method::{Method, MethodError};
use std::convert::{TryFrom, From};
use std::error::Error;
use std::fmt::{Display, Debug, Formatter, Result as FmtResult};
use std::str::{from_utf8, Utf8Error};

pub struct Request<'buf> {
	path: &'buf str,
	query_string: Option<&'buf str>,
	method: Method,
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {

	type Error = ParseError;

	fn try_from(buffer: &'buf[u8]) -> Result<Self, Self::Error>{
		// match from_utf8(buffer) {
		// 	Ok(request) => request,
		// 	Err(_) => return Err(ParseError::InvalidEncoding),
		// };

		// match from_utf8(buffer).or(Err(ParseError::InvalidEncoding)) {
		// 	Ok(request) => request,
		// 	Err(e) => return Err(e)
		// };
		//equvalent of above is ?

		let request = from_utf8(buffer)?;

		let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		
		if protocol != "HTTP/1.1" {
			return Err(ParseError::InvalidProtocol);
		}

		let method: Method = method.parse()?;
		let mut query_string = None;

		// match path.find('?') {
		// 	Some(i) => {
		// 		query_string = Some(&path[i+1..]);
		// 		path = &path[..i];
		// 	},
		// 	None => {}
		// }
		//Below is if let, simplified syntax for above
		//omit None block
		if let Some(i) = path.find('?') {
			query_string = Some(&path[i+1..]);
			path = &path[..i];
		}

		Ok(
			Self { 
				path: path, 
				query_string: query_string, 
				method: method 
			}
		)
	}
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i,c) in request.chars().enumerate() {
		if c == ' ' || c == '\r' {
			return Some((&request[..i], &request[i+1..]));
		}
	}
	unimplemented!()
}

pub enum ParseError {
	InvalidRequest,
	InvalidMethod,
	InvalidProtocol,
	InvalidEncoding
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl From<Utf8Error> for ParseError {
	fn from(_:Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl From<MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod
	}
}

impl ParseError {
	fn message(&self) -> &str {
		match self {
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidMethod => "Invalid Method",
			Self::InvalidProtocol => "Invalid Protocol",
			Self::InvalidRequest => "Invalid Request"
		}
	}
}

impl Error for ParseError {}