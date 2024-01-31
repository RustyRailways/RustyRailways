
pub trait HasUrl {
    fn get_url(&self)->&'static str;
}
pub const URL_MASTER: &str = "http://192.168.0.100:9000/master_message";
pub const IP_MASTER: &str = "192.168.0.100";
pub const MASTER_PORT_CONTROLLER: &str = "9000";
pub const MASTER_PORT_VISUALIZER: &str = "9001";
pub const URL_T1: &str = "http://192.168.0.101/train_message";
pub const URL_T2: &str = "http://192.168.0.102/train_message";
pub const URL_S1: &str = "http://192.168.0.103/";
pub const URL_S2: &str = "http://s2/";
pub const URL_S3: &str = "http://s3/";
pub const URL_S4: &str = "http://s4/";
pub const URL_S5: &str = "http://s5/";
pub const URL_S6: &str = "http://s6/";
pub const URL_S7: &str = "xxx";
pub const URL_S8: &str = "xxx";
pub const URL_S9: &str = "xxx";
pub const URL_S10: &str = "xxx";