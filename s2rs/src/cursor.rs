
#[derive(Default, Clone)]
pub struct Cursor {
    pub start: usize,
    pub end: Option<usize>,
}

impl Cursor {
    pub fn new(start: usize, end: Option<usize>) -> Self {
        Self {
            start,
            end
        }
    }

    pub fn kill(&mut self) {
        self.end = Some(self.start);
    }

    pub fn with_start(start: usize) -> Self {
        Self {
            start,
            end: None
        }
    }

    pub fn limited(start: usize, limit: usize) -> Self {
        Self {
            start,
            end: Some(start + limit)
        }
    }

    pub fn can_progress(&self) -> bool {
        match self.end {
            Some(end) => end > self.start,
            None => true
        }
    }

    pub fn progress(&mut self, limit: usize) -> Self {
        let start = self.start;
        self.start = if let Some(end) = self.end {
            (self.start + limit).min(end)
        } else {
            self.start + limit
        };
        Self {
            start,
            end: Some(self.start)
        }
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.end.map(|end| end - self.start)
    }
}

impl From<(usize, usize)> for Cursor {
    fn from((start, end): (usize, usize)) -> Self {
        Self::new(start, Some(end))
    }
}

impl From<(usize, Option<usize>)> for Cursor {
    fn from((start, end): (usize, Option<usize>)) -> Self {
        Self::new(start, end)
    }
}