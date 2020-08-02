use crate::client::CosClient;
use crate::signature::Signature;

use anyhow::anyhow;
use quick_xml::events::Event;
use quick_xml::Reader;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;

#[derive(Debug)]
pub struct ListAllMyBucketsResult {
    pub owner: Owner,
    pub buckets: Vec<Bucket>,
}

#[derive(Debug)]
pub struct Owner {
    id: String,
    display_name: String,
}

#[derive(Debug)]
pub struct Bucket {
    name: String,
    location: String,
    creation_date: String,
}

impl CosClient {
    /// 查询存储桶列表
    pub async fn list_bucket(&self) -> anyhow::Result<ListAllMyBucketsResult> {
        let url = "http://service.cos.myqcloud.com/";

        let mut headers = HeaderMap::new();

        let signature = Signature::new(&self.secret_id, &self.secret_key);
        headers.insert(
            "Authorization",
            signature.create_authorization(7200).parse().unwrap(),
        );

        let client = reqwest::Client::new();
        let resp = client.get(url).headers(headers).send().await?;
        let resp_text;
        match resp.status() {
            StatusCode::OK => resp_text = resp.text().await?,
            _ => return Err(anyhow!("Err")),
        };

        // let resp_text = r##"
        //     <ListAllMyBucketsResult>
        //         <Owner>
        //             <ID>qcs::cam::uin/100014682658:uin/100014682658</ID>
        //             <DisplayName>100014682658</DisplayName>
        //         </Owner>
        //         <Buckets>
        //             <Bucket>
        //                 <Name>blog-1302455983</Name>
        //                 <Location>ap-guangzhou</Location>
        //                 <CreationDate>2020-07-24T04:34:12Z</CreationDate>
        //             </Bucket>
        //             <Bucket>
        //                 <Name>test-1302455983</Name>
        //                 <Location>ap-guangzhou</Location>
        //                 <CreationDate>2020-07-31T07:27:15Z</CreationDate>
        //             </Bucket>
        //         </Buckets>
        //     </ListAllMyBucketsResult>
        // "##;

        // xml 反序列化为 struct
        {
            let mut reader = Reader::from_str(&resp_text);
            reader.trim_text(true);

            let mut buf = Vec::new();

            let owner: Owner;
            let mut id = String::new();
            let mut display_name = String::new();

            let mut buckets = vec![];
            let mut name = String::new();
            let mut location = String::new();
            let mut creation_date = String::new();

            // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
            loop {
                // dbg!(reader.read_event(&mut buf));
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) => match e.name() {
                        b"ID" => id = reader.read_text(e.name(), &mut Vec::new()).unwrap(),
                        b"DisplayName" => {
                            display_name = reader.read_text(e.name(), &mut Vec::new()).unwrap()
                        }

                        b"Name" => name = reader.read_text(e.name(), &mut Vec::new()).unwrap(),
                        b"Location" => {
                            location = reader.read_text(e.name(), &mut Vec::new()).unwrap()
                        }
                        b"CreationDate" => {
                            creation_date = reader.read_text(e.name(), &mut Vec::new()).unwrap()
                        }
                        _ => (),
                    },
                    Ok(Event::End(ref e)) => match e.name() {
                        b"Bucket" => {
                            let bucket = Bucket {
                                name: name.clone(),
                                location: location.clone(),
                                creation_date: creation_date.clone(),
                            };
                            buckets.push(bucket);
                        }
                        _ => (),
                    },
                    Ok(Event::Eof) => {
                        owner = Owner { id, display_name };
                        break;
                    } // exits the loop when reaching end of file
                    Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                    _ => (), // There are several other `Event`s we do not consider here
                }

                // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
                buf.clear();
            }

            Ok(ListAllMyBucketsResult { owner, buckets })
        }
    }
}
