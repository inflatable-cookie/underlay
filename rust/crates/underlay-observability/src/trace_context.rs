use http::header::HeaderValue;
use http::HeaderMap;

pub const TRACEPARENT_HEADER: &str = "traceparent";
pub const TRACESTATE_HEADER: &str = "tracestate";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceContext {
    version: String,
    trace_id: String,
    parent_span_id: String,
    trace_flags: String,
    tracestate: Option<String>,
}

impl TraceContext {
    pub fn parse(traceparent: &str, tracestate: Option<&str>) -> Option<Self> {
        let mut parts = traceparent.trim().split('-');
        let version = parts.next()?.to_ascii_lowercase();
        let trace_id = parts.next()?.to_ascii_lowercase();
        let parent_span_id = parts.next()?.to_ascii_lowercase();
        let trace_flags = parts.next()?.to_ascii_lowercase();

        if parts.next().is_some() {
            return None;
        }

        if !is_valid_hex(&version, 2) || version == "ff" {
            return None;
        }

        if !is_valid_hex(&trace_id, 32) || is_all_zeroes(&trace_id) {
            return None;
        }

        if !is_valid_hex(&parent_span_id, 16) || is_all_zeroes(&parent_span_id) {
            return None;
        }

        if !is_valid_hex(&trace_flags, 2) {
            return None;
        }

        let tracestate = tracestate
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned);

        Some(Self {
            version,
            trace_id,
            parent_span_id,
            trace_flags,
            tracestate,
        })
    }

    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        let traceparent = headers.get(TRACEPARENT_HEADER)?.to_str().ok()?;
        let tracestate = headers.get(TRACESTATE_HEADER).and_then(|value| value.to_str().ok());
        Self::parse(traceparent, tracestate)
    }

    pub fn inject_into(&self, headers: &mut HeaderMap) {
        if let Ok(traceparent) = HeaderValue::from_str(&self.traceparent()) {
            headers.insert(TRACEPARENT_HEADER, traceparent);
        }

        match self.tracestate() {
            Some(tracestate) => {
                if let Ok(header) = HeaderValue::from_str(tracestate) {
                    headers.insert(TRACESTATE_HEADER, header);
                }
            }
            None => {
                headers.remove(TRACESTATE_HEADER);
            }
        }
    }

    pub fn traceparent(&self) -> String {
        format!(
            "{}-{}-{}-{}",
            self.version, self.trace_id, self.parent_span_id, self.trace_flags
        )
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    pub fn parent_span_id(&self) -> &str {
        &self.parent_span_id
    }

    pub fn trace_flags(&self) -> &str {
        &self.trace_flags
    }

    pub fn tracestate(&self) -> Option<&str> {
        self.tracestate.as_deref()
    }
}

fn is_valid_hex(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_all_zeroes(value: &str) -> bool {
    value.chars().all(|ch| ch == '0')
}

#[cfg(test)]
#[path = "tests/trace_context_tests.rs"]
mod tests;
