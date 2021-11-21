use crate::error::Error;
use crate::result::Result;
use protofish::context::Context;
use std::path::Path;

pub async fn load_protobufs(path: impl AsRef<Path>) -> Result<Context> {
    let common_types = get_common_types().await?;
    let protos = tokio::fs::read_to_string(path)
        .await
        .map_err(Error::UnableToReadProtobuf)?;

    Ok(Context::parse(&[common_types, vec![protos]].concat())?)
}

async fn get_common_types() -> Result<Vec<String>> {
    let mut bufs = vec![];
    let mut dir = tokio::fs::read_dir("google_protobuf")
        .await
        .map_err(Error::UnableToReadProtobuf)?;
    while let Some(entry) = dir
        .next_entry()
        .await
        .map_err(Error::UnableToReadProtobuf)?
    {
        if !entry
            .file_name()
            .to_str()
            .expect("common types proto file name cannot be converted to &str")
            .contains(".proto")
        {
            continue;
        }

        let contents = tokio::fs::read_to_string(entry.path())
            .await
            .expect("unable to read file");

        bufs.push(contents);
    }

    Ok(bufs)
}
