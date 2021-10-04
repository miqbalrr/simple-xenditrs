pub mod domain_data;
pub mod xendit;

#[cfg(test)]
mod tests {
    const KEY: &str = "xnd_development_OomAfOUth+GowsY6LeJOHzLCZtSj84J9kXDn+Rxj/mHW+byhDQVxhg==";

    use crate::xendit;
    use tokio;

    #[test]
    fn fva_bca_check() {
        let xendit = xendit::Xendit::new(KEY.to_owned());
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(xendit.list_fva_banks())
            .unwrap();
        let res = res.iter().find(|&bank| bank.code == "BCA".to_owned());
        assert_eq!(res.unwrap().code, "BCA".to_owned())
    }

    #[test]
    fn va_detail_check() {
        let va_id = "6157df3b68e7625fdc4b37a0";
        let xendit = xendit::Xendit::new(KEY.to_owned());
        let res = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(xendit.virtual_account_detail(va_id))
            .unwrap();
        assert_eq!(res.id, va_id.to_owned())
    }
}
