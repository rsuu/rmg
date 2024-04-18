pub fn load_avif(_bytes: &[u8]) -> eyre::Result<(u32, u32, Vec<Vec<u8>>)> {
    #[cfg(feature = "de_avif")]
    {
        return feat::load_avif(_bytes);
    }

    eyre::bail!("")
}

#[cfg(feature = "de_avif")]
mod feat {
    #[inline]
    pub fn load_avif(bytes: &[u8]) -> eyre::Result<(u32, u32, Vec<Vec<u8>>)> {
        todo!()
    }
}
