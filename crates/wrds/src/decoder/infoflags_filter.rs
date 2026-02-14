// SPDX-FileCopyrightText: 2026 Patrick Nguyen
//
// SPDX-License-Identifier: MPL-2.0

use crate::{decoder::mode_filter::ModeFilter, types::InfoFlags};

const FILTER_COUNT: usize = 6;
const FILTER_MIN: usize = 5;

type Filter = ModeFilter<bool, FILTER_COUNT>;

impl Default for Filter {
    fn default() -> Self {
        Filter::new(FILTER_MIN).unwrap()
    }
}

#[derive(Debug, Default)]
pub struct InfoFlagsFilters {
    tp_filter: Filter,
    ta_filter: Filter,
    ms_filter: Filter,
    stereo_filter: Filter,
}

impl InfoFlagsFilters {
    /// Push TP sample to filter
    pub fn push_tp(&mut self, tp: bool) {
        self.tp_filter.push(tp);
    }

    /// Push TA sample to filter
    pub fn push_ta(&mut self, ta: bool) {
        self.ta_filter.push(ta);
    }

    /// Push MS sample to filter
    pub fn push_ms(&mut self, ms: bool) {
        self.ms_filter.push(ms);
    }

    /// Push stereo sample to filter
    pub fn push_stereo(&mut self, stereo: bool) {
        self.stereo_filter.push(stereo);
    }

    /// Reset current state
    pub fn reset(&mut self) {
        self.tp_filter.reset();
        self.ta_filter.reset();
        self.ms_filter.reset();
        self.stereo_filter.reset();
    }

    /// Returns current InfoFlags
    pub fn info_flags(&self) -> InfoFlags {
        let mut flags = InfoFlags::default();

        if let Some(tp) = self.tp_filter.mode() {
            flags.set(InfoFlags::TRAFFIC_PROGRAM, tp);
        }
        if let Some(ta) = self.ta_filter.mode() {
            flags.set(InfoFlags::TRAFFIC_ANNOUNCEMENT, ta);
        }
        if let Some(ms) = self.ms_filter.mode() {
            flags.set(InfoFlags::MUSIC, ms);
        }
        if let Some(stereo) = self.stereo_filter.mode() {
            flags.set(InfoFlags::STEREO, stereo);
        }
        flags
    }
}
