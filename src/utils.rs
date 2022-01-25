use hyper::{body, Response};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub fn bool_to_string(b: bool) -> String {
    match b {
        true => "1".to_string(),
        false => "0".to_string(),
    }
}

pub async fn response_into_string(res: Response<body::Body>) -> Result<String, Box<dyn Error>> {
    let bytes = body::to_bytes(res.into_body()).await?;
    let s = std::str::from_utf8(&bytes)?;
    Ok(s.to_string())
}

// Checks for various signs of a body request in the string represtation of a body, i.e. `404
// Error`, `DOCTYPE`, etc.
pub fn was_bad_request(body: &str) -> bool {
    todo!()
}

// The download files method of tricebot will return the temp files.
// This function will zip them up and will eventually return a zipped file.
pub fn zip_tempfiles(files: &HashMap<String, std::fs::File>) -> () {
    todo!()
}
/* Python source:
           if (len(replayStrs) == 0):
               return None
           tmpFile = tempfile.TemporaryFile(mode="wb+", suffix="tricebot.py", prefix="replaydownloads.zip")
           #tmpFile = open("I hate python.zip", "wb+")
           zipf = zipfile.ZipFile(tmpFile, "w", zipfile.ZIP_DEFLATED)
           for i in range(0, len(replayStrs)):
               replayStr = replayStrs[i]
               name = replayNames[i]
               zipf.writestr(name, replayStr, compress_type=zipfile.ZIP_DEFLATED, compresslevel=9)
           zipf.close()
           tmpFile.seek(0)
           return tmpFile
*/
