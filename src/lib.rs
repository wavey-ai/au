use bytes::Bytes;
use chrono::Duration;
use h264::iterate_annex_b;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AuKind {
    AAC,
    AVC,
    AVCC,
}

#[derive(Debug, Clone)]
pub struct Fmp4 {
    pub init: Option<Bytes>,
    pub key: bool,
    pub data: Bytes,
    pub duration: u32,
}

#[derive(Debug, Clone)]
pub struct AuPayload {
    pub kind: AuKind,
    pub data: Option<Bytes>,
    pub sps: Option<Bytes>,
    pub pps: Option<Bytes>,
    pub dts: Option<Duration>,
    pub pts: Option<Duration>,
    pub path: Option<u64>,
    pub key: bool,
    pub audio_codec_id: Option<u8>,
    pub audio_bitrate_kbps: Option<u32>,
    pub audio_sample_rate: Option<u32>,
    pub audio_channels: Option<u8>,
    pub seq: u64,
    pub track_id: usize,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub fps: Option<f64>,
}

impl AuPayload {
    pub fn new() -> Self {
        Self {
            kind: AuKind::AAC,
            data: None,
            sps: None,
            dts: None,
            pps: None,
            pts: None,
            path: None,
            key: false,
            audio_codec_id: None,
            audio_channels: None,
            audio_sample_rate: None,
            audio_bitrate_kbps: None,
            width: None,
            height: None,
            seq: 0,
            track_id: 0,
            fps: None,
        }
    }

    pub fn pts(&self) -> i64 {
        self.pts.unwrap_or_default().num_milliseconds()
    }

    pub fn dts(&self) -> i64 {
        self.dts.unwrap_or_default().num_milliseconds()
    }

    pub fn nalus_from_annex_b(&self) -> Vec<&[u8]> {
        let mut ret = Vec::new();

        if let Some(data) = &self.data {
            for nalu in iterate_annex_b(&data) {
                ret.push(nalu);
            }
        }

        ret
    }

    pub fn nalus_from_lp(&self) -> Vec<&[u8]> {
        self.data
            .iter()
            .flat_map(|data| {
                let mut offset = 0;
                let mut nalus = Vec::new();
                while offset + 4 <= data.len() {
                    let length = u32::from_be_bytes([
                        data[offset],
                        data[offset + 1],
                        data[offset + 2],
                        data[offset + 3],
                    ]) as usize;
                    if offset + 4 + length > data.len() {
                        break;
                    }
                    nalus.push(&data[offset + 4..offset + 4 + length]);
                    offset += 4 + length;
                }
                nalus
            })
            .collect()
    }
}
