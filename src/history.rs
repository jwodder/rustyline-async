use std::collections::VecDeque;

use futures::channel::mpsc::{self, Receiver, Sender};

pub struct History {
	pub entries: VecDeque<String>,
	pub max_size: usize,
	pub sender: Sender<String>,
	receiver: Receiver<String>,

	current_position: Option<usize>,
}
impl Default for History {
	fn default() -> Self {
		let (sender, receiver) = mpsc::channel(20);
		Self {
			entries: Default::default(),
			max_size: Default::default(),
			sender,
			receiver,
			current_position: Default::default(),
		}
	}
}

impl History {
	// Update history entries
	pub async fn update(&mut self) {
		// Receive all new lines
		while let Ok(Some(line)) = self.receiver.try_next() {
			// Add entry to front of history
			self.entries.push_front(line);
			// Reset offset to newest entry
			self.current_position = None;
			// Check if already have enough entries
			if self.entries.len() > self.max_size {
				// Remove oldest entry
				self.entries.pop_back();
			}
		}
	}

	// Find next history that matches a given string from an index
	pub fn search_next(&mut self, _current: &str) -> Option<&str> {
		if let Some(index) = &mut self.current_position {
			if *index < self.entries.len() - 1 {
				*index += 1;
			}
			Some(&self.entries[*index])
		} else if !self.entries.is_empty() {
			self.current_position = Some(0);
			Some(&self.entries[0])
		} else {
			None
		}
	}
	// Find previous history item that matches a given string from an index
	pub fn search_previous(&mut self, _current: &str) -> Option<&str> {
		if let Some(index) = &mut self.current_position {
			if *index == 0 {
				self.current_position = None;
				return Some("");
			}
			*index -= 1;
			Some(&self.entries[*index])
		} else {
			None
		}
	}
}