use super::FatFileSystem;
use libfs::block::{Block, BlockDevice, BlockIndex};

#[derive(Debug, Copy, Clone, PartialEq)]
/// Represent a FAT Cluster position.
pub struct Cluster(pub u32);

impl Cluster {
    /// Compute the offset of the data from the cluster position.
    pub fn to_data_block_index<T>(self, fs: &FatFileSystem<T>) -> BlockIndex
    where
        T: BlockDevice,
    {
        let first_block_of_cluster = (self.0 - 2) * u32::from(fs.boot_record.blocks_per_cluster());
        BlockIndex(fs.first_data_offset.0 + first_block_of_cluster)
    }

    /// Compute the offset in the cluster map of the cluster chain.
    pub fn to_fat_offset(self) -> u32 {
        self.0 * 4
    }

    /// Compute the block index of a cluster in the cluster map.
    pub fn to_fat_block_index<T>(self, fs: &FatFileSystem<T>) -> BlockIndex
    where
        T: BlockDevice,
    {
        let fat_offset = self.to_fat_offset();

        let fat_block_index =
            u32::from(fs.boot_record.reserved_block_count()) + (fat_offset / Block::LEN_U32);
        BlockIndex(fat_block_index)
    }
}
