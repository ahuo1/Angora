use crate::{cond_stmt_base::CondStmtBase, tag::TagSeg};
use serde_derive::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Write, BufWriter},
    path::Path,
};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LogData {
    pub cond_list: Vec<CondStmtBase>,
    pub tags: HashMap<u32, Vec<TagSeg>>,
    pub magic_bytes: HashMap<usize, (Vec<u8>, Vec<u8>)>,
}

impl LogData {
    pub fn new() -> Self {
        Self {
            cond_list: vec![],
            tags: HashMap::new(),
            magic_bytes: HashMap::new(),
        }
    }

    // xfuzz modify.
    pub fn serialize_to_fd<W: Write>(&self, mut writer: W) -> io::Result<()> {
        // Serialize cond_list
        writer.write_all(&(self.cond_list.len() as u32).to_le_bytes())?;
        for (i, cond) in self.cond_list.iter().enumerate() {
            writer.write_all(&cond.cmpid.to_le_bytes())?;
            writer.write_all(&cond.context.to_le_bytes())?;
            writer.write_all(&cond.order.to_le_bytes())?;
            writer.write_all(&cond.belong.to_le_bytes())?;
            writer.write_all(&cond.condition.to_le_bytes())?;
            writer.write_all(&cond.level.to_le_bytes())?;
            writer.write_all(&cond.op.to_le_bytes())?;
            writer.write_all(&cond.size.to_le_bytes())?;
            writer.write_all(&cond.lb1.to_le_bytes())?;
            writer.write_all(&cond.lb2.to_le_bytes())?;
            writer.write_all(&cond.arg1.to_le_bytes())?;
            writer.write_all(&cond.arg2.to_le_bytes())?;
        }

        writer.write_all(&(self.tags.len() as u32).to_le_bytes())?;
        for (key, tags) in &self.tags {
            writer.write_all(&key.to_le_bytes())?;
            writer.write_all(&(tags.len() as u32).to_le_bytes())?;
            for tag in tags {
                writer.write_all(&(tag.sign as u8).to_le_bytes())?;
                writer.write_all(&tag.begin.to_le_bytes())?;
                writer.write_all(&tag.end.to_le_bytes())?;
            }
        }

        writer.write_all(&(self.magic_bytes.len() as u32).to_le_bytes())?;
        for (key, (vec1, vec2)) in &self.magic_bytes {
            writer.write_all(&key.to_le_bytes())?;
            writer.write_all(&(vec1.len() as u32).to_le_bytes())?;
            writer.write_all(vec1)?;
            writer.write_all(&(vec2.len() as u32).to_le_bytes())?;
            writer.write_all(vec2)?;
        }

        Ok(())
    }
}