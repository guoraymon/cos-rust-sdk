use chrono::Local;
use crypto::{
    hmac::Hmac,
    sha1::Sha1,
    mac::Mac,
    digest::Digest
};

pub struct Signature {
    secret_id: String,
    secret_key: String
}

impl Signature {
    pub fn new(secret_id: &str, secret_key: &str) -> Signature {
        Signature {
            secret_id: secret_id.into(),
            secret_key: secret_key.into()
        }
    }


    /// 请求签名
    /// 签名步骤：https://cloud.tencent.com/document/product/436/7778
    pub fn create_authorization(&self, expires: i64) -> String {
        // 步骤1：生成 KeyTime
        // 拼接签名有效时间，格式为StartTimestamp;EndTimestamp，即为 KeyTime。例如：1557902800;1557910000。
        let key_time =  {
            let timestamp = Local::now().timestamp();
            format!("{};{}", timestamp, timestamp + expires)
        };
        // println!("key_time: {:?}", key_time);

        // 步骤2：生成 SignKey
        // 使用 HMAC-SHA1 以 SecretKey 为密钥，以 KeyTime 为消息，计算消息摘要（哈希值，16进制小写形式），即为 SignKey，例如：eb2519b498b02ac213cb1f3d1a3d27a3b3c9bc5f。
        let sign_key = {
            let mut hmac = Hmac::new(Sha1::new(), self.secret_key.as_bytes());
            hmac.input(key_time.as_bytes());
            hex::encode(hmac.result().code())
        };
        // println!("sign_key: {:?}", sign_key);

        // 步骤3：生成 UrlParamList 和 HttpParameters
        let url_param_list = format!("");

        // 步骤4：生成 HeaderList 和 HttpHeaders
        // TODO::未完成
        let header_list = format!("");

        // 步骤5：生成 HttpString
        // TODO::未完成
        let http_string = format!("get\n/\n\n\n");
        // println!("http_string: {:?}", http_string);

        // 步骤6：生成 StringToSign
        let string_to_sign = {
            let mut sha1 = Sha1::new();
            sha1.input_str(&http_string);
            format!("sha1\n{}\n{}\n", key_time, sha1.result_str())
        };
        // println!("string_to_sign: {:?}", string_to_sign);

        // 步骤7：生成 Signature
        // 使用 HMAC-SHA1 以 SignKey 为密钥（字符串形式，非原始二进制），以 StringToSign 为消息，计算消息摘要，即为 Signature，例如：01681b8c9d798a678e43b685a9f1bba0f6c0e012。
        let signature = {
            let mut hmac = Hmac::new(Sha1::new(), sign_key.as_bytes());
            hmac.input(string_to_sign.as_bytes());
            hex::encode(hmac.result().code())
        };
        // println!("signature: {:?}", signature);

        // 步骤8：生成签名
        format!("q-sign-algorithm=sha1&q-ak={ak}&q-sign-time={time}&q-key-time={time}&q-header-list={header_list}&q-url-param-list={url_param_list}&q-signature={signature}",
                ak = self.secret_id,time = key_time, header_list = header_list, url_param_list = url_param_list, signature = signature
        )
    }
}