pub mod proto {
    pub use hickory_proto::*;
}

pub mod resolver {
    pub use hickory_resolver::*;
    use proto::rr::Record;

    pub trait TtlClip {
        fn set_max_ttl(&mut self, ttl: u32);
        fn set_min_ttl(&mut self, ttl: u32);
        fn set_ttl(&mut self, ttl: u32);
    }

    impl TtlClip for Record {
        #[inline]
        fn set_max_ttl(&mut self, max_ttl: u32) {
            if self.ttl() > max_ttl {
                self.set_ttl(max_ttl);
            }
        }

        #[inline]
        fn set_min_ttl(&mut self, min_ttl: u32) {
            if self.ttl() < min_ttl {
                self.set_ttl(min_ttl);
            }
        }

        #[inline]
        fn set_ttl(&mut self, ttl: u32) {
            Record::set_ttl(self, ttl);
        }
    }
}

pub mod server {
    pub use hickory_server::*;
}
